# Template for custom trailer connected provider

This folder contains a sample template about how to setup a devcontainer environment for replacing the trailer connected provider (SmartTrailer DT Provider of the architectural picture) with a custom application.

The example template pulls in the trailer connected provider proto file dependency, compiles the python library based on the protobuf file and constructs sample protobuf messages. The messages are print to stdout.

Feel free to extend or customize the Dockerfile. You can also replace or create other workload applications based on this template.

**Note:** The ubuntu image is used instead of a smaller alpine image to enable code completion suggestions in VSCode, because with the alpine's protobuf-compiler the generated python lib is different and does not allow code completion suggestions of the IDE.

## Build

Navigate into the following subfolder:

```shell
cd in-vehicle-stack/sample_trailer_connected_provider
```

Specify the version tag before executing the following build command:

```shell
podman build -t sample_trailer_connected_provider:<version_tag> -f .devcontainer/Dockerfile .
```

## Run

```shell
podman run -d --name sample_trailer_connected_provider sample_trailer_connected_provider:<version_tag>
```

## Logs

```shell
podman logs -f sample_trailer_connected_provider
```

Optionally cleanup and remove:

```shell
podman rm sample_trailer_connected_provider
```