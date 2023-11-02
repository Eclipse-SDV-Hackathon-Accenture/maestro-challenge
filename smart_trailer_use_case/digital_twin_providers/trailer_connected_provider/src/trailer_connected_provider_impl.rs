// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.
// SPDX-License-Identifier: MIT

//! Module containing gRPC service implementation based on [`interfaces::trailer_connected_provider.proto`].
//!
//! Provides a gRPC endpoint for getting if the trailer is connected
use smart_trailer_interfaces::trailer_connected_provider::v1::trailer_connected_provider_server::TrailerConnectedProvider;
use smart_trailer_interfaces::trailer_connected_provider::v1::{GetRequest, GetResponse};
use tonic::{Request, Response, Status};

/// Base structure for the Trailer Connected Provider gRPC service.
#[derive(Default)]
pub struct TrailerConnectedProviderImpl {}

#[tonic::async_trait]
impl TrailerConnectedProvider for TrailerConnectedProviderImpl {
    /// This function returns the value of "is_trailer_connected" property
    async fn get(&self, _request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        // For now, we assume that if this provider is active, the trailer is connected
        // To expand this use case, we could simulate the trailer being disconnected as well
        let get_response = GetResponse {
            is_trailer_connected: true,
        };
        Ok(Response::new(get_response))
    }
}
