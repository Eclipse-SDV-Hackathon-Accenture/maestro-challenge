// Copyright (c) IAV  GmbH.
// Licensed under the MIT license.
// SPDX-License-Identifier: MIT

//! Module containing gRPC service implementation based on [`interfaces::digital_twin_get_provider.proto`].
//!
//! Provides a gRPC endpoint for getting if the cars ignition is off
use wheelchair_assistant_interfaces::digital_twin_get_provider::v1::digital_twin_get_provider_server::DigitalTwinGetProvider;
use wheelchair_assistant_interfaces::digital_twin_get_provider::v1::{GetRequest, GetResponse};
use tonic::{Request, Response, Status};

/// Base structure for the Car ignition off Provider gRPC service.
#[derive(Default)]
pub struct CarOffProviderImpl {}

#[tonic::async_trait]
impl DigitalTwinGetProvider for CarOffProviderImpl {
    /// This function returns the value of "is_car_off" property
    async fn get(&self, _request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        // For now, we assume that if this provider is active, if the cars ignition is off
        let get_response = GetResponse {
            property_value: false,
        };
        Ok(Response::new(get_response))
    }
}
