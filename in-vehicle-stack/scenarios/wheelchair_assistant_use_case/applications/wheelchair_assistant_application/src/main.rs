// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.
// SPDX-License-Identifier: MIT

use std::env;

use wheelchair_digital_twin_model::{car_v1, Metadata};
use wheelchair_digital_twin_providers_common::constants::chariott::{
    INVEHICLE_DIGITAL_TWIN_SERVICE_COMMUNICATION_KIND,
    INVEHICLE_DIGITAL_TWIN_SERVICE_COMMUNICATION_REFERENCE, INVEHICLE_DIGITAL_TWIN_SERVICE_NAME,
    INVEHICLE_DIGITAL_TWIN_SERVICE_NAMESPACE, INVEHICLE_DIGITAL_TWIN_SERVICE_VERSION,
};
use wheelchair_digital_twin_providers_common::constants::{
    constraint_type, digital_twin_operation, digital_twin_protocol,
};
use wheelchair_digital_twin_providers_common::utils::{
    discover_digital_twin_provider_using_ibeji, discover_service_using_chariott, get_uri,
};
use env_logger::{Builder, Target};
use interfaces::module::managed_subscribe::v1::managed_subscribe_client::ManagedSubscribeClient;
use interfaces::module::managed_subscribe::v1::{
    Constraint, SubscriptionInfoRequest, SubscriptionInfoResponse,
};
use interfaces::invehicle_digital_twin::v1::invehicle_digital_twin_client::InvehicleDigitalTwinClient;
use interfaces::invehicle_digital_twin::v1::{EndpointInfo, EntityAccessInfo, RegisterRequest};
use log::{debug, info, warn, LevelFilter};
use paho_mqtt as mqtt;
use tokio::signal;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use tonic::{Request, Status};
use tokio::sync::watch;
use uuid::Uuid;
use serde_derive::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};

const FREQUENCY_MS_FLAG: &str = "freq_ms=";
const MQTT_CLIENT_ID: &str = "wheelchair-assistant-consumer";

// TODO: These could be added in configuration
const CHARIOTT_SERVICE_DISCOVERY_URI: &str = "http://0.0.0.0:50000";

const DEFAULT_FREQUENCY_MS: u64 = 10000; // 10 seconds

// Constants used for retry logic
const MAX_RETRIES: i32 = 10; // for demo purposes we will retry a maximum of 10 times
                             // By default we will wait 5 seconds between retry attempts
const DURATION_BETWEEN_ATTEMPTS: Duration = Duration::from_secs(5);

static door_open: AtomicBool = AtomicBool::new(false);
static steering_wheel_up: AtomicBool = AtomicBool::new(false);
static driver_seat_back: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag="type")]
struct WheelchairAssistantStateProperty {
    #[serde(rename = "WheelchairAssistantState")]
    car_wheelchair_assistant_state: car_v1::car::car_wheelchair_assistant_state::TYPE,
    #[serde(rename = "$metadata")]
    metadata: Metadata,
}

/// Get car adjustment subscription information from managed subscribe endpoint.
///
/// # Arguments
/// * `managed_subscribe_uri` - The managed subscribe URI.
/// * `constraints` - Constraints for the managed topic.
async fn get_car_adjust_subscription_info(
    managed_subscribe_uri: &str,
    constraints: Vec<Constraint>
) -> Result<SubscriptionInfoResponse, Status> {
    // Create gRPC client.
    let mut client = ManagedSubscribeClient::connect(managed_subscribe_uri.to_string())
        .await
        .map_err(|err| Status::from_error(err.into()))?;

    let request = Request::new(SubscriptionInfoRequest {
        entity_id: car_v1::car::car_wheelchair_assistant_state::ID.to_string(),              
        constraints,
    });

    let response = client.get_subscription_info(request).await?;

    Ok(response.into_inner())
}

/// Receive car adjustment updates.
///
/// # Arguments
/// * `broker_uri` - The broker URI.
/// * `topic` - The topic.
async fn receive_car_adjust_updates(
    broker_uri: &str,
    topic: &str,
) -> Result<JoinHandle<()>, String> {
    // Create a unique id for the client.
    let client_id = format!("{MQTT_CLIENT_ID}-{}", Uuid::new_v4());

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(broker_uri)
        .client_id(client_id)
        .finalize();

    let client = mqtt::Client::new(create_opts)
        .map_err(|err| format!("Failed to create the client due to '{err:?}'"))?;

    let receiver = client.start_consuming();

    // Setup task to handle clean shutdown.
    let ctrlc_cli = client.clone();
    tokio::spawn(async move {
        _ = signal::ctrl_c().await;

        // Tells the client to shutdown consuming thread.
        ctrlc_cli.stop_consuming();
    });

    // Last Will and Testament
    let lwt = mqtt::MessageBuilder::new()
        .topic("test")
        .payload("Receiver lost connection")
        .finalize();

    let conn_opts = mqtt::ConnectOptionsBuilder::new_v5()
        .keep_alive_interval(Duration::from_secs(30))
        .clean_session(false)
        .will_message(lwt)
        .finalize();

    let _connect_response = client
        .connect(conn_opts)
        .map_err(|err| format!("Failed to connect due to '{err:?}"));

    let mut _subscribe_response = client
        .subscribe(topic, mqtt::types::QOS_1)
        .map_err(|err| format!("Failed to subscribe to topic {topic} due to '{err:?}'"));

    // Copy topic for separate thread.
    let topic_string = topic.to_string();

    let sub_handle = tokio::spawn(async move {
        for msg in receiver.iter() {
            if let Some(msg) = msg {
                // Here we log the message received. This could be expanded to parsing the message,
                // Obtaining the wheelchair assistant state and making decisions based on the weight
                // For example, adjusting body functions or powertrain of the towing vehicle.
                let payload_str = msg.payload_str();
                let msg_des: WheelchairAssistantStateProperty = serde_json::from_str(&payload_str).unwrap();
                let new_state = msg_des.car_wheelchair_assistant_state;
                info!("{}", new_state);
                info!("{}", msg);

                if new_state == 3 { //HOLD
                    info!("Adjusting the car!");
                    door_open.store(true, Ordering::Relaxed);
                    steering_wheel_up.store(true, Ordering::Relaxed);
                    driver_seat_back.store(true, Ordering::Relaxed);
                } else {
                    info!("No need to rearrange");
                }

            } else if !client.is_connected() {
                if client.reconnect().is_ok() {
                    _subscribe_response = client
                        .subscribe(topic_string.as_str(), mqtt::types::QOS_1)
                        .map_err(|err| {
                            format!("Failed to subscribe to topic {topic_string} due to '{err:?}'")
                        });
                } else {
                    break;
                }
            }
        }

        if client.is_connected() {
            debug!("Disconnecting");
            client.unsubscribe(topic_string.as_str()).unwrap();
            client.disconnect(None).unwrap();
        }
    });

    Ok(sub_handle)
}

/// Register the car seat adjustment property's endpoint.
///
/// # Arguments
/// * `invehicle_digital_twin_uri` - The In-Vehicle Digital Twin URI.
/// * `provider_uri` - The provider's URI.
async fn provider_register_seat_adjustment(
    invehicle_digital_twin_uri: &str,
    provider_uri: &str,
) -> Result<(), Status> {
    let endpoint_info = EndpointInfo {
        protocol: digital_twin_protocol::GRPC.to_string(),
        operations: vec![digital_twin_operation::MANAGEDSUBSCRIBE.to_string()],
        uri: provider_uri.to_string(),
        context: "GetSubscriptionInfo".to_string(),
    };

    let entity_access_info = EntityAccessInfo {
        name: car_v1::car::is_car_seat_in_assist_position::NAME.to_string(),           
        id: car_v1::car::is_car_seat_in_assist_position::ID.to_string(),                 
        description: car_v1::car::is_car_seat_in_assist_position::DESCRIPTION.to_string(),       
        endpoint_info_list: vec![endpoint_info],
    };

    let mut client = InvehicleDigitalTwinClient::connect(invehicle_digital_twin_uri.to_string())
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    let request = tonic::Request::new(RegisterRequest {
        entity_access_info_list: vec![entity_access_info],
    });
    let _response = client.register(request).await?;

    Ok(())
}

/// Register the car door adjustment property's endpoint.
///
/// # Arguments
/// * `invehicle_digital_twin_uri` - The In-Vehicle Digital Twin URI.
/// * `provider_uri` - The provider's URI.
async fn provider_register_door_adjustment(
    invehicle_digital_twin_uri: &str,
    provider_uri: &str,
) -> Result<(), Status> {
    let endpoint_info = EndpointInfo {
        protocol: digital_twin_protocol::GRPC.to_string(),
        operations: vec![digital_twin_operation::MANAGEDSUBSCRIBE.to_string()],
        uri: provider_uri.to_string(),
        context: "GetSubscriptionInfo".to_string(),
    };

    let entity_access_info = EntityAccessInfo {
        name: car_v1::car::is_car_door_open::NAME.to_string(),           
        id: car_v1::car::is_car_door_open::ID.to_string(),                 
        description: car_v1::car::is_car_door_open::DESCRIPTION.to_string(),       
        endpoint_info_list: vec![endpoint_info],
    };

    let mut client = InvehicleDigitalTwinClient::connect(invehicle_digital_twin_uri.to_string())
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    let request = tonic::Request::new(RegisterRequest {
        entity_access_info_list: vec![entity_access_info],
    });
    let _response = client.register(request).await?;

    Ok(())
}

/// Register the car steering wheel adjustment property's endpoint.
///
/// # Arguments
/// * `invehicle_digital_twin_uri` - The In-Vehicle Digital Twin URI.
/// * `provider_uri` - The provider's URI.
async fn provider_register_steering_wheel_adjustment(
    invehicle_digital_twin_uri: &str,
    provider_uri: &str,
) -> Result<(), Status> {
    let endpoint_info = EndpointInfo {
        protocol: digital_twin_protocol::GRPC.to_string(),
        operations: vec![digital_twin_operation::MANAGEDSUBSCRIBE.to_string()],
        uri: provider_uri.to_string(),
        context: "GetSubscriptionInfo".to_string(),
    };

    let entity_access_info = EntityAccessInfo {
        name: car_v1::car::is_car_steeringwheel_in_assist_position::NAME.to_string(),           
        id: car_v1::car::is_car_steeringwheel_in_assist_position::ID.to_string(),                 
        description: car_v1::car::is_car_steeringwheel_in_assist_position::DESCRIPTION.to_string(),       
        endpoint_info_list: vec![endpoint_info],
    };

    let mut client = InvehicleDigitalTwinClient::connect(invehicle_digital_twin_uri.to_string())
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
    let request = tonic::Request::new(RegisterRequest {
        entity_access_info_list: vec![entity_access_info],
    });
    let _response = client.register(request).await?;

    Ok(())
}

/// Start the seat adjustment data stream.
fn provider_start_seat_adjustment_data_stream() {
    debug!("Starting the Provider's seat adjustment data stream.");
    
    let (sender, reciever) = watch::channel(driver_seat_back.load(Ordering::Relaxed));
    tokio::spawn(async move {

        loop {
            debug!(
                "Recording new value for {} of {}",
                car_v1::car::is_car_seat_in_assist_position::ID,
                driver_seat_back.load(Ordering::Relaxed)
            );

            if let Err(err) = sender.send(driver_seat_back.load(Ordering::Relaxed)) {
                warn!("Failed to get new value due to '{err:?}'");
                break;
            }

            debug!("Completed the publish request");
        }
    });
}

/// Start the door adjustment data stream.
fn provider_start_door_adjustment_data_stream() {
    debug!("Starting the Provider's door adjustment data stream.");
    
    let (sender, reciever) = watch::channel(door_open.load(Ordering::Relaxed));
    tokio::spawn(async move {

        loop {
            debug!(
                "Recording new value for {} of {}",
                car_v1::car::is_car_door_open::ID,
                door_open.load(Ordering::Relaxed)
            );

            if let Err(err) = sender.send(door_open.load(Ordering::Relaxed)) {
                warn!("Failed to get new value due to '{err:?}'");
                break;
            }

            debug!("Completed the publish request");
        }
    });
}

/// Start the steering wheel adjustment data stream.
fn provider_start_steering_wheel_adjustment_data_stream() {
    debug!("Starting the Provider's steering wheel adjustment data stream.");
    
    let (sender, reciever) = watch::channel(steering_wheel_up.load(Ordering::Relaxed));
    tokio::spawn(async move {

        loop {
            debug!(
                "Recording new value for {} of {}",
                car_v1::car::is_car_steeringwheel_in_assist_position::ID,
                steering_wheel_up.load(Ordering::Relaxed)
            );

            if let Err(err) = sender.send(steering_wheel_up.load(Ordering::Relaxed)) {
                warn!("Failed to get new value due to '{err:?}'");
                break;
            }

            debug!("Completed the publish request");
        }
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging.
    Builder::new()
        .filter(None, LevelFilter::Info)
        .target(Target::Stdout)
        .init();

    info!("The Wheelchair Assistant Application has started.");

    const PROVIDER_AUTHORITY_SEAT: &str = "0.0.0.0:4070";
    const PROVIDER_AUTHORITY_DOOR: &str = "0.0.0.0:4080";
    const PROVIDER_AUTHORITY_STEER: &str = "0.0.0.0:4090";

    // Get the In-vehicle Digital Twin Uri from the service discovery system
    // This could be enhanced to add retries for robustness
    let invehicle_digital_twin_uri = discover_service_using_chariott(
        CHARIOTT_SERVICE_DISCOVERY_URI,
        INVEHICLE_DIGITAL_TWIN_SERVICE_NAMESPACE,
        INVEHICLE_DIGITAL_TWIN_SERVICE_NAME,
        INVEHICLE_DIGITAL_TWIN_SERVICE_VERSION,
        INVEHICLE_DIGITAL_TWIN_SERVICE_COMMUNICATION_KIND,
        INVEHICLE_DIGITAL_TWIN_SERVICE_COMMUNICATION_REFERENCE,
    )
    .await?;

    // Get subscription constraints.
    let frequency_ms = env::args()
        .find_map(|arg| {
            if arg.contains(FREQUENCY_MS_FLAG) {
                return Some(arg.replace(FREQUENCY_MS_FLAG, ""));
            }

            None
        })
        .unwrap_or_else(|| DEFAULT_FREQUENCY_MS.to_string());

    // Retrieve the provider URI.
    let mut provider_endpoint_info = None;
    let mut retries: i32 = 0;
    while provider_endpoint_info.is_none() {
        provider_endpoint_info = match discover_digital_twin_provider_using_ibeji(
            &invehicle_digital_twin_uri,
            car_v1::car::car_wheelchair_assistant_state::ID,
            digital_twin_protocol::GRPC,
            &[digital_twin_operation::MANAGEDSUBSCRIBE.to_string()],
        )
        .await
        {
            Ok(response) => Some(response),
            Err(status) => {
                info!(
                    "A provider was not found in the digital twin service for id '{}' with: '{:?}'",
                    car_v1::car::car_wheelchair_assistant_state::ID,
                    status
                );
                None
            }
        };

        if provider_endpoint_info.is_none() && retries < MAX_RETRIES {
            info!("Retrying FindById to retrieve the properties provider endpoint in {DURATION_BETWEEN_ATTEMPTS:?}.");
            sleep(DURATION_BETWEEN_ATTEMPTS).await;
            retries += 1;
        } else {
            break;
        }
    }

    let managed_subscribe_uri = provider_endpoint_info.ok_or("Maximum amount of retries was reached while trying to retrieve the digital twin provider.")?.uri;
    info!("The Managed Subscribe URI for the IsCarUnlocked property's provider is {managed_subscribe_uri}");

    // Create constraint for the managed subscribe call.
    let frequency_constraint = Constraint {
        r#type: constraint_type::FREQUENCY_MS.to_string(),
        value: frequency_ms.to_string(),
    };

    // Get the subscription information for a managed topic with constraints.
    let subscription_info =
        get_car_adjust_subscription_info(&managed_subscribe_uri, vec![frequency_constraint])
            .await?;

    // Deconstruct subscription information.
    let broker_uri = get_uri(&subscription_info.uri)?;
    let topic = subscription_info.context;
    info!("The broker URI for the car_wheelchair_assistant_state property's provider is {broker_uri}");

    // Subscribe to topic.
    let sub_handle = receive_car_adjust_updates(&broker_uri, &topic)
        .await
        .map_err(|err| Status::internal(format!("{err:?}")))?;
    

    provider_start_seat_adjustment_data_stream();
    signal::ctrl_c().await?;

    let provider_uri_seat = format!("http://{PROVIDER_AUTHORITY_SEAT}");
    provider_register_seat_adjustment(&invehicle_digital_twin_uri, &provider_uri_seat).await?;
    debug!("The Provider Seat has registered with Ibeji.");

    provider_start_door_adjustment_data_stream();
    signal::ctrl_c().await?;

    let provider_uri_door = format!("http://{PROVIDER_AUTHORITY_DOOR}");
    provider_register_door_adjustment(&invehicle_digital_twin_uri, &provider_uri_door).await?;
    debug!("The Provider Door has registered with Ibeji.");

    provider_start_steering_wheel_adjustment_data_stream();
    signal::ctrl_c().await?;

    let provider_uri_steer = format!("http://{PROVIDER_AUTHORITY_STEER}");
    provider_register_steering_wheel_adjustment(&invehicle_digital_twin_uri, &provider_uri_steer).await?;
    debug!("The Provider SteeringWheel has registered with Ibeji.");

    info!("The Consumer has completed. Shutting down...");

    // Wait for subscriber task to cleanly shutdown.
    _ = sub_handle.await;

    Ok(())
}