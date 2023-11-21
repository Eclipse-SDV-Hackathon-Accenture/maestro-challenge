## In Vehicle Stack

This in-vehicle stack directory contains files related to the In-Vehicle Stack (Eclipse Chariott, Ibeji, Agemo,
and Freyja).

### Interfaces

For simplicity for the hackathon, the files in the `interfaces` directory are copied from the main
repositories. These protobuf files enable communication with the services. You can find the
original files at the links below:
- [In-vehicle Digital Twin](https://github.com/eclipse-ibeji/ibeji/tree/main/interfaces/invehicle_digital_twin/v1)
- [Managed Subscribe Module](https://github.com/eclipse-ibeji/ibeji/tree/main/interfaces/module/managed_subscribe/v1)
for using Agemo
- [Service Discovery](https://github.com/eclipse-chariott/chariott/tree/main/service_discovery/proto/core/v1)

### Compiling the interfaces

The `proto_build` directory provides the necessary files to compile the protobuf with Rust. If you
would like to develop sample applications using Rust, you can use `cargo build -p interfaces`. If
you are not using Rust, use the [protobuf compiler](https://grpc.io/docs/protoc-installation/) with
the language of your choice to generate clients for interacting with the services.

### Cloud Connection

Please refer to the [In-Vehicle Stack with Cloud Connectivity](cloud-connection.md) for
instructions on how to setup the In-Vehicle Stack to connect with Azure.

### Configuration overrides

To override configuration for one of the in-vehicle stack services, please follow
[How to Override Configuration for In-Vehicle Stack Containers](config_overrides.md/#how-to-override-configuration-for-in-vehicle-stack-containers).
