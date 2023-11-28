// Copyright (c) IAV  GmbH.
// Licensed under the MIT license.
// SPDX-License-Identifier: MIT

mod wheelchair_distance_increasing_provider_impl;

use std::net::SocketAddr;

use wheelchair_digital_twin_model::car_v1;
use wheelchair_digital_twin_providers_common::constants::chariott::{
    INVEHICLE_DIGITAL_TWIN_SERVICE_COMMUNICATION_KIND,
    INVEHICLE_DIGITAL_TWIN_SERVICE_COMMUNICATION_REFERENCE, INVEHICLE_DIGITAL_TWIN_SERVICE_NAME,
    INVEHICLE_DIGITAL_TWIN_SERVICE_NAMESPACE, INVEHICLE_DIGITAL_TWIN_SERVICE_VERSION,
};
use digital_twin_providers_common::constants::{digital_twin_operation, digital_twin_protocol};
use digital_twin_providers_common::utils::discover_service_using_chariott;
use env_logger::{Builder, Target};
use interfaces::invehicle_digital_twin::v1::invehicle_digital_twin_client::InvehicleDigitalTwinClient;
use interfaces::invehicle_digital_twin::v1::{EndpointInfo, EntityAccessInfo, RegisterRequest};
use interfaces::module::managed_subscribe::v1::managed_subscribe_callback_server::ManagedSubscribeCallbackServer;
use log::{debug, info, warn, LevelFilter};
use tokio::signal;
use tokio::sync::watch;
use tokio::time::{sleep, Duration};
use tonic::transport::Server;
use tonic::Status;

use crate::wheelchair_distance_increasing_provider_impl::WheelchairDistanceIncreasingProviderImpl;

// TODO: These could be added in configuration
const CHARIOTT_SERVICE_DISCOVERY_URI: &str = "http://0.0.0.0:50000";
const PROVIDER_AUTHORITY: &str = "0.0.0.0:4060";

const DEFAULT_MIN_INTERVAL_MS: u64 = 10;

/// Register the wheelchair distance property's endpoint.
///
/// # Arguments
/// * `invehicle_digital_twin_uri` - The In-Vehicle Digital Twin URI.
/// * `provider_uri` - The provider's URI.
async fn register_wheelchair_distance(
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
        name: car_v1::car::car_wheelchair_distance::NAME.to_string(),
        id: car_v1::car::car_wheelchair_distance::ID.to_string(),
        description: car_v1::car::car_wheelchair_distance::DESCRIPTION.to_string(),
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

/// Start the wheelchair distance data stream. Add 1 cm every 10 ms, so make 1 m per second.
///
/// # Arguments
/// `min_interval_ms` - minimum frequency for data stream.
fn start_wheelchair_distance_increasing_data_stream(min_interval_ms: u64) -> watch::Receiver<i32> {
    debug!("Starting the Provider's wheelchair distance increasing data stream.");
    let mut distance: i32 = 0;
    let (sender, receiver) = watch::channel(distance);
    tokio::spawn(async move {
        loop {
            debug!(
                "Recording new value for {} of {distance}",
                car_v1::car::car_wheelchair_distance::ID
            );

            if let Err(err) = sender.send(distance) {
                warn!("Failed to get new value due to '{err:?}'");
                break;
            }

            debug!("Completed the publish request");

            // Calculate the new distance, it increases by 1 m every second, until it is 10 m.
			// This function simulates the person going away from the car.
			if distance < 1000 {
				distance += 1;
			}

            sleep(Duration::from_millis(min_interval_ms)).await;
        }
    });

    receiver
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging.
    Builder::new()
        .filter(None, LevelFilter::Info)
        .target(Target::Stdout)
        .init();

    info!("The Provider has started.");

    let provider_uri = format!("http://{PROVIDER_AUTHORITY}"); // Devskim: ignore DS137138

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

    debug!("The Provider retrieved Chariott's Service Discovery URI.");

    // Start mock data stream.
    let data_stream = start_wheelchair_distance_increasing_data_stream(DEFAULT_MIN_INTERVAL_MS);
    debug!("The Provider has started the wheelchair distance increasing data stream.");

    // Setup provider management cb endpoint.
    let provider = WheelchairDistanceIncreasingProviderImpl::new(data_stream, DEFAULT_MIN_INTERVAL_MS);

    // Start service.
    let addr: SocketAddr = PROVIDER_AUTHORITY.parse()?;
    let server_future = Server::builder()
        .add_service(ManagedSubscribeCallbackServer::new(provider))
        .serve(addr);

    // This could be enhanced with retries for robustness
    register_wheelchair_distance(&invehicle_digital_twin_uri, &provider_uri).await?;
    debug!("The Provider has registered with Ibeji.");

    server_future.await?;

    signal::ctrl_c()
        .await
        .expect("Failed to listen for control-c event");

    info!("The Provider has completed.");

    Ok(())
}
