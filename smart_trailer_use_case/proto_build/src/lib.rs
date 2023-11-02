// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.
// SPDX-License-Identifier: MIT

pub mod trailer_connected_provider {
    pub mod v1 {
        #![allow(clippy::derive_partial_eq_without_eq)]
        tonic::include_proto!("trailer_connected_provider");
    }
}
