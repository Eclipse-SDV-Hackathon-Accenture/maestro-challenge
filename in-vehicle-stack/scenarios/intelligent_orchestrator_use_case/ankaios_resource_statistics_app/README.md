# Resource Monitoring Workload

For challenges requiring resource usage statistics a workload application is provided that provides you some initial resource statistic values of the host the workload is running on. The statistics can be accessed via a REST API as JSON.

This image is public available: 

```shell
docker pull ghcr.io/eclipse-ankaios/maestro_resource_monitor:latest
```

The image url is already pre-configured in the [initial startup state](../../../eclipse-ankaios/config/startupState.yaml) of Ankaios.
So, if you select a challenge requiring the statistics please uncomment the config part in the initial startup state to enable this workload and start Ankaios again.

## Request the resource usage statistic

To fetch the resource usage statistics execute the following command:

```shell
curl -s localhost:25555
```

## Extend with custom resource usage values

The workload app is designed for easily adding new resource statistic values. Depending on your input for the OpenAI model you can adapt the commands fetching the resource statistic values from the host namespace.

Open the file `src/commands.rs` and adapt existing commands or add a new command:

```rust
    pub static ref COMMANDS: HashMap<&'static str, &'static str> = {
        HashMap::from([
            // ...
            ("new-resource-statistic", "command for new resource statistic value")
        ])
    };
```

The commands are automatically executed and the values inserted into the json response.

After you have added commands for fetching resource usage statistics you must build a new version of the container image and
publish it to a container registry of your choice. Afterwards you must replace the initial image url in the [initial startup state](../../../eclipse-ankaios/config/startupState.yaml) of Ankaios.

## Build

Replace `<your_registry_path>` in the following command with a registry you have access to and build the resource monitor workload:

```shell
docker build -t <your_registry_path>/maestro_resource_monitor:latest -f .devcontainer/Dockerfile .
```

Push the image to your desired registry:

```shell
docker build -t <your_registry_path>/maestro_resource_monitor:latest -f .devcontainer/Dockerfile .
```

**Note:** Make sure that the image is public available (without authentication) or log in into the registry before starting the workload with Ankaios.

## Run

You can also run the workload without Ankaios for testing, by running it with the following command and settings:

```shell
docker run --rm -d --name resource_monitor --privileged --pid host --hostname resource_monitor -p 25555:25555 <your_registry_path>/maestro_resource_monitor:latest
```