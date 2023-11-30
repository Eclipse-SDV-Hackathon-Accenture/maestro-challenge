// Copyright (c) IAV  GmbH.
// Licensed under the MIT license.
// SPDX-License-Identifier: MIT

//! Module containing gRPC service implementation based on [`interfaces::digital_twin_get_provider.proto`].
//!
//! Provides a gRPC endpoint for getting if the car key is unlocked
use wheelchair_assistant_interfaces::digital_twin_get_provider::v1::digital_twin_get_provider_server::DigitalTwinGetProvider;
use wheelchair_assistant_interfaces::digital_twin_get_provider::v1::{GetRequest, GetResponse};
use tonic::{Request, Response, Status};

/// Base structure for the Carkey Unlock Provider gRPC service.
#[derive(Default)]
pub struct CarkeyUnlockProviderImpl {}

#[tonic::async_trait]
impl DigitalTwinGetProvider for CarkeyUnlockProviderImpl {
    /// This function returns the value of "is_car_unlocked" property
    async fn get(&self, _request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        // For now, we assume that if this provider is active, the car is unlocked
        let get_response = GetResponse {
            property_value: true,
        };
        Ok(Response::new(get_response))
    }
}
