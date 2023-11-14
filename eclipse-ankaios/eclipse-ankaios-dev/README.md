# Ankaios Maestro Challenge Starter Template

![Smart trailer blueprint](https://user-images.githubusercontent.com/9027586/276944985-4e37b961-9ff6-4f6f-a370-18734b54ca63.png)

This repository provides a starter template for solving the Maestro challenge using the [Ankaios](https://github.com/eclipse-ankaios) workload orchestrator.
It contains a development environment with pre-installed and -configured [Ankaios](https://github.com/eclipse-ankaios) devcontainer that makes it easy for you to start developing and building container applications.

## Development environment

The following is provided:

- pre-installed Ankaios executables (`ank-server`, `ank-agent` and `ank`) and Podman container engine (contained in the Ankaios base devcontainer)
- an Ankaios [startupState.yaml](./config/startupState.yaml) config with pre-configured workloads for the initial system of the Ankaios Maestro Challenge (without the workloads marked in blue which are part of the challenge)
- helper scripts for starting and cleaning up the Ankaios cluster (the script path is exported into the PATH environment variable, so that you can execute the scripts directly without navigating into the `scripts` folder.)
- workload application for [resource usage statistics](#resource-usage-statistics) for challenges requiring resource statistics
- an initial devcontainer which you can extend with the tools you require

All services are running in the host network meaning those can be accessed with `localhost:<port>`. We recommend that you set the network mode to host for all your developed workloads as well.

## General information about Ankaios
Feel free to get familiar with Ankaios' basics by checking the [Ankaios docs](https://eclipse-ankaios.github.io/ankaios/latest/). A good point to start is the [Getting Started](https://eclipse-ankaios.github.io/ankaios/latest/usage/quickstart/) section.

## Hackathon required knowledge about Ankaios

When you start the Ankaios server, you must provide a config file written in YAML which contains the initial startup state. This startup state lists all workloads that Ankaios shall start initially. After parsing the configuration, the Ankaios server applies the startup state as current state of the cluster keeping track of all workloads, their configurations and execution states.
If you want to add, update or delete workloads you can change the current state of the cluster by using `ank` - the Ankaios CLI. A detailed explanation about managing states can be found [here](https://eclipse-ankaios.github.io/ankaios/latest/reference/complete-state/).
As an alternative to updating the current state, you can restart the system by deleting all workloads, shuting down Ankaios completely, changing the startup state config file and starting Ankaios again (automated for you via the [cleanup_ankaios.sh](./scripts/cleanup_ankaios.sh) script).
This isn't usually done in production, but might be helpful during development of an app workload. For more details about the general Ankaios architecture with its components, you can look into the [architectural doc](https://eclipse-ankaios.github.io/ankaios/latest/architecture/).

Furthermore, Ankaios is designed to make it easy for workloads to communicate with the Ankaios cluster over an API. This is done via the so called [Ankaios Control Interface](https://eclipse-ankaios.github.io/ankaios/latest/reference/control-interface/).
Using the Control Interface is only mandatory if you select a challenge requiring a direct communication of a workload to the Ankaios cluster, for example when a workload shall request Ankaios to start another workload dynamically or when a workload wants to gather information about other workloads managed by Ankaios.

Feel free to click through the entire [Ankaios documentation](https://eclipse-ankaios.github.io/ankaios/latest/) to get familiar with the Ankaios basics.

## Devcontainer

This template uses a multi-stage Containerfile (Dockerfile) where the first one is the `dev` stage and the second one - the `prod` stage.
The `dev` stage is designed to be used during the development of your application and the `prod` stage is for running your 'production ready' application. The main difference between `dev` and `prod` is that when using the `prod` stage, the Ankaios services and all the configured workloads in the provided initial startup state are started automatically when you start the container as you would expect from a production environment.

**Note:** The Ankaios server gRPC port 25551 is published outside of the devcontainer to give you the possibility to use the Ankaios CLI also from your host system. If you want to do this, you can download the latest pre-built binaries with a single command as described [here](https://eclipse-ankaios.github.io/ankaios/latest/usage/installation/). As mentioned above, the devcontainer already provides a preinstalled CLI, but using the CLI from outside of the container comes closer to managing a remote vehicle and might be more handy if you want to open many terminal windows in parallel.

### Devcontainer default dev dependencies

The devcontainer base image used inside `.devcontainer/Dockerfile` already includes all required packages and artifacts to enable an easy start of development.

It includes the following:

- protobuf compiler
- grpcurl
- The Ankaios protobuf file (located inside the devcontainer under `/usr/local/lib/ankaios/ankaios.proto`)

Those dependencies are needed for use-cases in which your app needs to use the [Ankaios Control Interface](https://eclipse-ankaios.github.io/ankaios/latest/reference/control-interface/) to be able to communicate with the Ankaios cluster via the API. An example use-case would be to write a workload that shall request Ankaios to dynamically start another workload.

### Use devcontainer with VSCode

Make sure that the [Remote Development](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack) extension pack is installed in your VSCode instance.

Open the subfolder containing this README file in VSCode:

```shell
code .
```

VSCode detects automatically that a `.devcontainer` folder exists inside the repository and asks you in a dialog to reopen VSCode inside the devcontainer.
Please confirm the dialog to reopen VSCode inside the devcontainer.
Afterwards, you can open a terminal inside the devcontainer.

### Use devcontainer without VSCode

Navigate to the subfolder containing this README file and run the following command to build the devcontainer image:

```shell
docker build -t custom-ankaios-dev:0.1 --target dev -f .devcontainer/Dockerfile .
```

**Note:** The command above builds for the target `dev` and ignores the `prod` stage.

Start the devcontainer with the subfolder containing this README file mounted inside:

```shell
docker run -it --privileged --name custom_ankaios_dev -v path/to/maestro-challenge/eclipse-ankaios/eclipse-ankaios-dev:/workspaces/app -p 25551:25551 --workdir /workspaces/app custom-ankaios-dev:0.1 /bin/bash
```

### Customizing Devcontainer

The devcontainer base image already includes dev dependencies for developing workloads using the [Ankaios Control Interface](https://eclipse-ankaios.github.io/ankaios/latest/reference/control-interface/), but you can customize the devcontainer depending on your preferred programming language and frameworks.

To customize the devcontainer add your specific dev dependencies and packages to `.devcontainer/Dockerfile` (starting from line 5). Please leave the `prod` stage inside the Dockerfile unchanged as it is required to build the production image later.

### Startup check before development

Before starting active development we recommend you start once Ankaios with the current initial state config [startupState.yaml](./config/startupState.yaml).

**Note:** If you have selected a challenge that requires resource usage statistics like cpu or memory usage, do the steps at section [Resrouce statistics](#resource-usage-statistics) before continuing.

Start Ankaios with all initial workloads:
```shell
start_ankaios.sh
``` 

Next, use the Ankaios CLI to verify that all initial workloads are up and running:

```shell
ank get workloads
```

Verify that all initial workloads inside the initial state config have execution state "Running".

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

If all workoads are up and running you can stop Ankaios and cleaning up all workloads by running:

```shell
cleanup_ankaios.sh
```

The cleanup routine stops the Ankaios server, Ankaios agent and stops and removes all workloads from the podman runtime.

Next, move on to the next section that explains how the devcontainer can be used during development.

### Using Ankaios inside the devcontainer during development

During development you might build and run the workload very often until it works entirely as desired.

The following steps shall make your life easier during development and testing.

After customizing the devcontainer and having the first runnable code, do the following:

1. Build and publish your developed image to your preferred container registry and make it public available. If you want to use a private registry make sure that you are logged in into the container registry by using [podman login](https://docs.podman.io/en/stable/markdown/podman-login.1.html) before moving on to the next steps. As an alternative you can use the local registry of podman if you build your Dockerfile inside the devcontainer using [podman build](https://docs.podman.io/en/stable/markdown/podman-build.1.html) command.
2. Extend the initial state config under [startupState.yaml](./config/startupState.yaml) with your newly developed workloads.
3. Start Ankaios with the initial state config [startupState.yaml](./config/startupState.yaml) by using the provided script [start_ankaios.sh](./scripts/start_ankaios.sh):
```shell
start_ankaios.sh
```
4. Use the [Ankaios CLI](https://eclipse-ankaios.github.io/ankaios/latest/reference/complete-state/) to check the execution states of your workloads by running:
```shell
$ ank get workloads
 WORKLOAD NAME                AGENT     RUNTIME   EXECUTION STATE
 cloud_connector            agent_A   podman    Running
 digital_twin_cloud_sync    agent_A   podman    Running
 digital_twin_vehicle       agent_A   podman    Running
 dynamic_topic_management   agent_A   podman    Running
 mqtt_broker                agent_A   podman    Running
 service_discovery          agent_A   podman    Running
 your_workload_1            agent_A   podman    Running
 your_workload_2            agent_A   podman    Failed
```
5. If one of your newly developed workloads fails (that might happen quite often during development :wink: ) and you want to build and run everything again, you just need to run [cleanup_ankaios.sh](./scripts/cleanup_ankaios.sh) script that shuts down everything and restart the cluster with the startup config easier during development and automated for you.
```shell
cleanup_ankaios.sh
```

Remember, if you want to start with a clean state, you can always execute the [cleanup_ankaios.sh](./scripts/cleanup_ankaios.sh) script which cleans up all Ankaios components and stops and deletes all workloads running on podman **including also workloads not managed by Ankaios** (the [startupState.yaml](./config/startupState.yaml) is left untouched). Otherwise you must do all that stuff manually.

### Build the production image for demonstartion

If you are done with your workload development, you must build the final production image.

This can be done easily by navigating outside the container to the root folder of this repository and executing the following build command:

Make sure to replace the path below with the path of your container registry.

```shell
docker build -t <your-registry-path>/ankaios-prod:latest -f .devcontainer/Dockerfile .
```

**Note:** You don't need to specify `prod` as target here, this is done automatically.

Push the image to the container registry:

```shell
docker push <your-registry-path>/ankaios-prod:latest
```

Make sure that the image is made public available, otherwise authentication is needed to run the image outside of the devcontainer.

Run this image on your local computer and add the desired ports of your applications to access them from your host system. You can also publish multiple ports:

```shell
docker run -it --privileged --rm -p 25551:25551 -p <extra_port>:<extra_port> <your-registry-path>/ankaios-prod:latest /bin/bash
```

The production image will start all workloads defined in [startupState.yaml](./config/startupState.yaml) automatically for easy demonstration once you start the container with the command above.

### Resource usage statistics

If you have selected a challenge that requires resource usage statistics, please uncomment the config part for the `resource_monitor` workload in [startupState.yaml](./config/startupState.yaml). This workload runs in privileged mode in the same pid namespace as the devcontainer and provides you with the system's resource usage statistics like cpu and memory usage. The workload updates internally every 5 sec the current resource usage statistics and provides the statistics via REST API. 

You can query the resource usage statistics with the following command after the workload has started:

```shell
curl localhost:25555
```

### Useful tricks

If you have installed the Ankaios CLI outside the devcontainer and not on the host itself, you can use the environment variable `ANK_SERVER_URL`
and the Ankaios CLI uses the value inside this environment variable to connect to the Ankaios server. This is more handy if you want to use the Ankaios CLI
externally in multipile terminal windows, because we can get rid of specifying the server url as CLI argument to the Ankaios CLI:

Without the environemnt variable on an external system:
```shell
ank --server-url http://192.168.1.111:25551 get workloads
```

With setting the environment variables instead:

```shell
$ export ANK_SERVER_URL=http://192.168.1.111:25551
$ ank get workloads
```

For more details display the help of Ankaios CLI by running: 
```shell
ank --help
```