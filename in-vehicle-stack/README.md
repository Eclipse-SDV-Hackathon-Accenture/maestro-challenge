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

Note: If you choose Ankaios as your orchestrator, please add your dependencies into the [Dockerfile](../eclipse-ankaios/.devcontainer/Dockerfile). For Rust, you can just uncomment the prepared lines in the Dockerfile.

### Cloud Connection

Please refer to the [In-Vehicle Stack with Cloud Connectivity](../docs/in-vehicle-stack/azure-cloud-connection.md) for
instructions on how to setup the In-Vehicle Stack to connect with Azure.

### Extending the Use Case

Please see [Hack Challenge - Extend the use case](../README.md#hack-challenge---extend-the-use-case) for ideas on how to extend the smart trailer scenario.

### Configuration overrides

To override the configuration for one of the in-vehicle stack services, please follow
[How to Override Configuration for In-Vehicle Stack Containers](../docs/in-vehicle-stack/config-overrides.md#how-to-override-configuration-for-in-vehicle-stack-containers).
