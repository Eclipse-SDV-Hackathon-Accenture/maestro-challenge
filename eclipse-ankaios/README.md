# Ankaios Maestro challenge development environment

![Smart trailer blueprint](https://user-images.githubusercontent.com/9027586/276944985-4e37b961-9ff6-4f6f-a370-18734b54ca63.png)

This repository provides a starter template for solving the Maestro challenges using the [Ankaios](https://github.com/eclipse-ankaios/ankaios) workload orchestrator.
It contains a pre-configured devcontainer that makes it easy for you to start developing and building container applications managed by Ankaios.

The container is designed to have an immediately running environment. Once triggered, all workloads are initially started, except the blue marked workloads in the picture above which are part of your challenge.

## Links

- [Ankaios docs](https://eclipse-ankaios.github.io/ankaios/latest/)
- [Ankaios quickstart](https://eclipse-ankaios.github.io/ankaios/latest/usage/quickstart/)
- [Podman](https://docs.podman.io/en/v4.6.1/)
- [What are devcontainers?](https://containers.dev/)

## Prerequisites

- Docker [Installation instructions](https://docs.docker.com/get-docker/)

## Development environment

The following is provided inside the devcontainer:

- Ankaios executables (`ank-server`, `ank-agent` and `ank`)

- Podman 4.6.2

- Pre-configured Ankaios startup config [startupState.yaml](./config/startupState.yaml)

- Automation scripts for starting and stopping all workloads of the challenge:
    - run_maestro.sh
    - shutdown_maestro.sh

- REST API providing [resource usage statistics](#resource-usage-statistics) for the sample scenario about intelligent orchestrator

- Exposed port:
    - 25551: for optionally using the Ankaios CLI outside of the devcontainer

- [Ankaios Control Interface dependencies](#ankaios-control-interface-dependencies) 


All services are running in the host network meaning those can be accessed by `localhost:<port>`. We recommend that you set the network mode to host for all your developed workloads as well.

## Ankaios Control Interface dependencies

The devcontainer includes also dependencies for developing applications using the [Ankaios Control Interface](https://eclipse-ankaios.github.io/ankaios/latest/reference/control-interface/):

- protobuf compiler
- grpcurl
- Ankaios protobuf file (under `/usr/local/lib/ankaios/ankaios.proto`)

Those dependencies are needed for use-cases in which your app needs to use the [Ankaios Control Interface](https://eclipse-ankaios.github.io/ankaios/latest/reference/control-interface/) to be able to communicate with the Ankaios cluster via the API. An example use-case would be to write a workload that shall request Ankaios to dynamically start another workload.

## Run devcontainer with VSCode

### Prerequisites
- [Remote Development](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack) extension installed in VSCode

Open the subfolder containing this README file in VSCode:

```shell
code .
```

VSCode detects automatically that a `.devcontainer` folder exists inside this subfolder.
Please confirm the dialog to reopen VSCode inside the devcontainer.
Afterwards, open a new terminal inside the devcontainer in VSCode.

## Run devcontainer without VSCode

Navigate to the subfolder containing this README file and run the following command to build the devcontainer image:

```shell
docker build -t custom-ankaios-dev:0.1 --target dev -f .devcontainer/Dockerfile .
```

**Note:** The command above builds for the target `dev` and ignores the `prod` stage.

Start the devcontainer with the subfolder containing this README file mounted inside:

```shell
docker run -it --privileged --name custom_ankaios_dev -v <absolute/path/to/>maestro-challenge/eclipse-ankaios:/workspaces/app -p 25551:25551 --workdir /workspaces/app custom-ankaios-dev:0.1 /bin/bash
```

## Startup check before development

Before starting active development we recommend you start once Ankaios with the current startup config [startupState.yaml](./config/startupState.yaml).

**Note:** If you have selected a sample scenario requiring resource usage statistics like cpu or memory usage, uncomment the `resource_monitor` config part in the Ankaios startup config [startupState.yaml](./config/startupState.yaml). For more details, see [here](../scenarios/intelligent_orchestrator_use_case/ankaios_resource_statistics_app/README.md).

1. Log in into the Microsoft container registry
```shell
podman login sdvblueprint.azurecr.io
```

2. Start Ankaios with all workloads inside the startup config:
```shell
run_maestro.sh
``` 

3. Next, use the Ankaios CLI to verify that all initial workloads are up and running:

```shell
ank get workloads
```

4. Verify that all initial workloads inside the startup config have execution state "Running".

The output looks similar to the following:
```shell
 WORKLOAD NAME              AGENT     RUNTIME   EXECUTION STATE
 cloud_connector            agent_A   podman    Running
 digital_twin_cloud_sync    agent_A   podman    Running
 digital_twin_vehicle       agent_A   podman    Running
 dynamic_topic_management   agent_A   podman    Running
 mqtt_broker                agent_A   podman    Running
 service_discovery          agent_A   podman    Running
```

5. Stop Ankaios and clean up all workloads by running:

```shell
shutdown_maestro.sh
```

## Customizing Devcontainer

You can customize the devcontainer depending on your preferred programming language, tools and frameworks.

To customize the devcontainer add your specific dev dependencies to `.devcontainer/Dockerfile` (starting from line 5). Please leave the `prod` stage inside the Dockerfile unchanged as it is required to build the production image later.

Rebuild the container image.

## Workload development

After customizing the devcontainer, start the development of your workload apps.

- Write your code
- Write a [Dockerfile](https://docs.docker.com/engine/reference/builder/) for each workload
- Build a container image for each workload with [podman build](https://docs.podman.io/en/v4.6.1/markdown/podman-build.1.html)
- Extend Ankaios startup config [startupState.yaml](./config/startupState.yaml) by adding config parts for your workloads

Start and stop all workloads according to the section [Startup check before development](#startup-check-before-development).
Use the Ankaios ClI to check the workload states. For more details display the help of Ankaios CLI by running: 
```shell
ank --help
```

Use the [podman logs](https://docs.podman.io/en/v4.6.1/markdown/podman-logs.1.html) command to check the logs of your container applications for debugging purposes.

```shell
podman ps -a
podman logs -f <container_name|container_id>
```

## Ankaios logs

There are log files for debugging purposes of Ankaios server and agent.

The Ankaios server logs can be viewed by executing the following command:

```shell
tail -f /var/log/ankaios-server.log
```

The Ankaios agent logs can be viewed by executing the following command:

```shell
tail -f /var/log/ankaios-agent_A.log
```

## Build the production image for demonstration

If you have finished development, publish your work by building a final production image.

```shell
docker build -t <your-registry-path>/maestro-ankaios-prod:latest -f .devcontainer/Dockerfile .
```

**Note:** You don't need to specify `prod` as target here, this is done automatically.

Push the image into the container registry (log in before if authentication is needed):

```shell
docker push <your-registry-path>/maestro-ankaios-prod:latest
```

Make sure that the image is made public available, otherwise authentication is needed to run the image outside of the devcontainer.

Run the image and add the desired ports of your applications to access them from your host system. You can also publish multiple ports:

```shell
docker run -it --privileged --rm -p 25551:25551 -p <extra_port>:<extra_port> <your-registry-path>/maestro-ankaios-prod:latest /bin/bash
```

Log in into the Microsoft container registry:

```shell
podman login sdvblueprint.azurecr.io
```

Start all maestro workloads by running:

```shell
run_maestro.sh
```

Present your developed workloads.