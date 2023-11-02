# Ankaios Maestro Challenge Starter Template

![Smart trailer blueprint](https://user-images.githubusercontent.com/9027586/276944985-4e37b961-9ff6-4f6f-a370-18734b54ca63.png)

This repository provides a starter template for solving the Maestro challenge using the [Ankaios](https://github.com/eclipse-ankaios) workload orchestrator.
It contains a development environment with pre-installed and -configured [Ankaios](https://github.com/eclipse-ankaios) devcontainer that makes it easy for you to start developing and building container applications.

The following is provided:

- pre-installed Ankaios executables (`ank-server`, `ank-agent` and `ank`) and Podman container engine (contained in the Ankaios base devcontainer)
- an Ankaios [startupState.yaml](./config/startupState.yaml) config with pre-configured workloads for the initial system of the Ankaios Maestro Challenge (without the marked in blue workloads which are part of the challenge)
- helper scripts for starting and cleaning up the Ankaios cluster
- an initial devcontainer which you can extend with the tools you require

## General information about Ankaios
Feel free to get familiar with Ankaios' basics by checking the [Ankaios docs](https://eclipse-ankaios.github.io/ankaios/0.1/). A good point to start is the [Getting Started](https://eclipse-ankaios.github.io/ankaios/0.1/usage/quickstart/) section.

## Hackathon required knowledge about Ankaios

When you start the Ankaios server, you must provide a config file written in YAML which contains the initial startup state. This startup state lists all workloads that Ankaios shall start initially. After parsing the configuration, the Ankaios server applies the startup state as current state of the cluster keeping track of all workloads, their configurations and execution states.
If you want to add, update or delete workloads you can change the current state of the cluster by using `ank` - the Ankaios CLI. A detailed explanation about managing states can be found [here](https://eclipse-ankaios.github.io/ankaios/0.1/reference/complete-state/).
As an alternative to updating the current state, you can restart the system by deleting all workloads, shuting down Ankaios completely, changing the startup state config file and starting Ankaios again (automated for you via the [cleanup_ankaios.sh](./scripts/cleanup_ankaios.sh) script).
This isn't usually done in production, but might be helpful during development of an app workload. For more details about the general Ankaios architecture with its components, you can look into the [architectural doc](https://eclipse-ankaios.github.io/ankaios/0.1/architecture/).

Furthermore, Ankaios is designed to make it easy for workloads to communicate with the Ankaios cluster over an API. This is done via the so called [Ankaios Control Interface](https://eclipse-ankaios.github.io/ankaios/0.1/reference/control-interface/).
Using the Control Interface is only mandatory if you select a challenge requiring a direct communication of a workload to the Ankaios cluster, for example when a workload shall request Ankaios to start another workload dynamically or when a workload wants to gather information about other workloads managed by Ankaios.

Feel free to click through the entire [Ankaios documentation](https://eclipse-ankaios.github.io/ankaios/0.1/) to get familiar with the Ankaios basics.

## Devcontainer

This template uses a multi-stage Containerfile (Dockerfile) where the first one is the `dev` stage and the second one - the `prod` stage.
The `dev` stage is designed to be used during the development of your application and the `prod` stage is for running your 'production ready' application. The main difference between `dev` and `prod` is that when using the `prod` stage, the Ankaios services and all the configured workloads in the provided initial startup state are started automatically when you start the container as you would expect from a production environment.

**Note:** The Ankaios server gRPC port 25551 is published outside of the devcontainer to give you the possibility to use the Ankaios CLI also from your host system. If you want to do this, you can download the latest pre-built binaries with a single command as described [here](https://eclipse-ankaios.github.io/ankaios/0.1/usage/installation/) As mentioned above, the devcontainer already provides a preinstalled CLI, but using the CLI from outside of the container comes closer to managing a remote vehicle and might be more handy if you want to open many terminal windows in parallel.

### Devcontainer default dev dependencies

The devcontainer base image used inside `.devcontainer/Dockerfile` already includes all required packages and artifacts to enable an easy start of development.

It includes the following:

- protobuf compiler
- grpcurl
- The Ankaios protobuf file (located inside the devcontainer under `/usr/local/lib/ankaios/ankaios.proto`)

Those dependencies are needed for use-cases in which your app needs to use the [Ankaios Control Interface](https://eclipse-ankaios.github.io/ankaios/0.1/reference/control-interface/) to be able to communicate with the Ankaios cluster via the API. An example use-case would be to write a workload that shall request Ankaios to dynamically start another workload.

### Use devcontainer with VSCode

Make sure that the [Remote Development](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack) extension pack is installed in your VSCode instance.

Navigate to the root folder of this repository and run:

```shell
code .
```

VSCode detects automatically that a `.devcontainer` folder exists inside the repository and asks you in a dialog to reopen VSCode inside the devcontainer.
Please confirm the dialog to reopen VSCode inside the devcontainer.
Afterwards, you can open a terminal inside the devcontainer.

### Use devcontainer without VSCode

Customize the `.devcontainer/Dockerfile` with your required dev dependencies.

Navigate to the root folder of this repository and run the following command to build the devcontainer image:

```shell
docker build -t custom-ankaios-dev:0.1 --target dev -f .devcontainer/Dockerfile .
```

**Note:** The command above builds for the target `dev` and ignores the `prod` stage.

Start the devcontainer with the repository mounted inside:

```shell
docker run --rm -it --privileged -v path/to/ankaios_maestro_challenge:/workspaces/app -p 25551:25551 custom-ankaios-dev:0.1 /bin/bash
```

### Customizing Devcontainer

The devcontainer base image already includes dev dependencies for developing workloads using the [Ankaios Control Interface](https://eclipse-ankaios.github.io/ankaios/0.1/reference/control-interface/), but you can customize the devcontainer depending on your preferred programming language and frameworks.

To customize the devcontainer add your specific dev dependencies and packages to `.devcontainer/Dockerfile` (starting from line 5). Please leave the `prod` stage inside the Dockerfile unchanged as it is required to build the production image later.

### Using Ankaios inside the devcontainer during development

During development you might build and run the workload very often until it works entirely as desired.

The following steps shall make your life easier during development and testing.

After customizing the devcontainer and having the first runnable code, do the following:

1. Build and publish your developed image to the provided public container registry. As an alternative you can use the local registry of podman if you build your Dockerfile inside the devcontainer using podman build command.
2. Extend the initial state config under [startupState.yaml](./config/startupState.yaml) with your newly developed workloads.
3. Open a terminal and start podman as a background service by running:
```shell
podman system service --time=0 unix:///tmp/podman.sock &
```
4. Start the Ankaios server with the initial state config [startupState.yaml](./config/startupState.yaml) by running:
```shell
ank-server --address 0.0.0.0:25551 --startup-config config/startupState.yaml
```
5. Open second terminal window and start the Ankaios agent by running:
```shell
ank-agent --server-url http://0.0.0.0:25551 --name agent_A -p /tmp/podman.sock
```
6. Use the [Ankaios CLI](https://eclipse-ankaios.github.io/ankaios/0.1/reference/complete-state/) to check the execution states of your workloads by running:
```shell
$ ank --server-url http://0.0.0.0:25551 get workloads
 WORKLOAD NAME                AGENT     RUNTIME   EXECUTION STATE
 cloud_connector              agent_A   podman    Running
 digital_twin_cloud_sync      agent_A   podman    Running
 digital_twin_vehicle         agent_A   podman    Running
 dynamic_topic_management     agent_A   podman    Running
 mqtt_broker                  agent_A   podman    Running
 service_registry_discovery   agent_A   podman    Running
 your_workload_1              agent_A   podman    Running
 your_workload_2              agent_A   podman    Failed
```
7. If one of your newly developed workloads fails (that might happen quite often during development :wink: ) and you want to build and run everything again, you have two possibilities: 
    * update the current state of Ankaios on-the-fly by using the Ankaios CLI (the production use-case; for more details see [here](https://eclipse-ankaios.github.io/ankaios/0.1/reference/complete-state/)) or 
    * shut down everything and restart the cluster with the new startup config (easier during development and automated for you via the [cleanup_ankaios.sh](./scripts/cleanup_ankaios.sh) script)

Remember, if you want to start with a clean state, you can always execute the [cleanup_ankaios.sh](./scripts/cleanup_ankaios.sh) script which cleans up all Ankaios components and stops and deletes all workloads running on podman (the [startupState.yaml](./config/startupState.yaml) is left untouched). Otherwise you must do all that stuff manually.

### Build the production image

If you are done with your workload development, you must build the final production image.

This can be done easily by navigating outside the container to the root folder of this repository and executing the following build command:

```shell
docker build -t sdvblueprint.azurecr.io/ankaios-prod:latest -f .devcontainer/Dockerfile .
```

**Note:** You don't need to specify `prod` as target here, this is done automatically.

Push the image to the public registry:

```shell
docker push sdvblueprint.azurecr.io/ankaios-prod:latest
```

Run this image on your local computer and add the desired ports of your applications to access them from your host system. You can also publish multiple ports:

```shell
docker run -it --privileged --rm -p 25551:25551 -p <extra_port>:<extra_port> sdvblueprint.azurecr.io/ankaios-prod:latest /bin/bash
```

The production image will start all workloads defined in [startupState.yaml](./config/startupState.yaml) automatically for easy demonstration once you start the container with the command above.
