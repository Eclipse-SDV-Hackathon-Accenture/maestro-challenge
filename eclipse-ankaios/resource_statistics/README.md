# Resource Monitoring Workload

## Build

Build the resource monitor workload:

```shell
docker build -t ghcr.io/eclipse-ankaios/maestro_resource_monitor:latest:latest -f .devcontainer/Dockerfile .
```

## Run

Run the resource monitor workload:

```shell
docker run --rm -d --name resource_monitor --privileged --pid host --hostname resource_monitor -p 25555:25555 ghcr.io/eclipse-ankaios/maestro_resource_monitor:latest:latest
```

## Request the resource usage statistic

To fetch the resource usage statistics execute the following curl command:

```shell
curl -s localhost:25555
```

## Extend with custom resource usage values

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
