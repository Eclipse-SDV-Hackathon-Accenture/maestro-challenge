# Eclipse BlueChi

![Smart trailer blueprint](../docs/diagrams/bluechi.png)

Upstream documentation: <https://bluechi.readthedocs.io/en/latest/>

BlueChi is a systemd controller that adds a thin layer to enable multi-node
workload management and cross-node dependencies.

It can handle various workloads such as containers, virtual machines or
applications running on bare metal. To run containers under systemd in an
optimal way it uses Podman's Quadlet implementation. This also enables the usage
of Kubernetes resource definitions to define the workload.

## Links

* [BlueChi documentation](https://bluechi.readthedocs.io/en/latest/)
* [BlueChi CLI
  documentation](<https://github.com/eclipse-bluechi/bluechi/blob/main/doc/man/bluechictl.1.md>)
* [Podman](https://docs.podman.io/en/latest/)
* [Podman and Quadlet](https://www.redhat.com/sysadmin/quadlet-podman)

## Prerequisites

* A container runtime such as [Docker](https://docs.docker.com/get-docker/) or [Podman](https://podman.io/docs/installation)
  * Note: non linux machines need to run docker/podman machine.

## Development Environment

There are two development environments mentioned in the next section that provide the following components:

* Eclipse Chariott
* Eclipse Agemo
* Eclipse Ibeji
* Eclipse Freyja
* Eclipse BlueChi
* systemd
* Podman
* Quadlet

All services are accessible via `localhost:$port`.

## Two Development Environments

It is strongly recommended that you use the devcontainer with VSCode.

## Run the devcontainer with VSCode

### Prerequisite
* Ensure that the [Remote Development extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack) installed in VSCode

* Upstream documentation:
  * <https://containers.dev/>
  * https://github.com/devcontainers/cli

The following steps below uses the VSCode devcontainer extension. If you prefer, you can use [VSCode's devcontainer's CLI](https://github.com/devcontainers/cli) instead.

1. Login to the Azure's container registry to allow VSCode to pull the devcontainer image:
    ```shell
    docker login sdvblueprint.azurecr.io
    ```
1. You can use the VSCode devcontainer extension to start your containerized development environment.
    ```shell
    cd <absolute/path/to>/maestro-challenge/eclipse-bluechi
    code .
    ```

1. VSCode detects automatically that a `.devcontainer` folder exists inside this subfolder. Please confirm the dialog to reopen VSCode inside the devcontainer. Afterwards, open a new terminal inside the devcontainer in VSCode.

## Run the devcontainer without VSCode

Upstream documentation: <https://www.docker.com/get-started/>

1. Login to the Azure's container registry:
    ```shell
    docker login sdvblueprint.azurecr.io
    ```

1. Start the devcontainer by running:
    ```sh
    docker run \
    -d \
    --privileged \
    --name autosd-eclipse \
    -v <absolute/path/to>/maestro-challenge/in-vehicle-stack:/workspaces/app/in-vehicle-stack \
    --workdir /workspaces/app \
    sdvblueprint.azurecr.io/sdvblueprint/eclipse-bluechi/devenv:latest
    ```

    Ensure to replace `<absolute/path/to>` with your own value.

1. Enter into the devcontainer and interact with BlueChi:
    ```sh
    docker exec -it autosd-eclipse /bin/bash
    bluechictl list-units
    ```

## Bootstrapping

You need to bootstrap all the Eclipse services once you got your eclipse-bluechi devcontainer running.

### Starting all the services
1. Inside your devcontainer, you will need to login to Azure's container registry to pull all required images:
    ```sh
    podman login \
    --username  <username> \
    --password  <password> \
    sdvblueprint.azurecr.io
    ```

2. Then it is time to start all services which can be done by executing the bootstrap script:
    ```sh
    $ bluechi-env-bootstrap
    ```
The above command will pull all the required images and start all services.

### Cleanup
1. There is also a script to stop all services:
    ```sh
    $ bluechi-env-cleanup
    ```
Keep in mind that stopping services will purge all the containers that are related to such services as well.

Both the `bluechi-env-bootstrap` and `bluechi-env-cleaup` scripts are located in `/usr/local/bin/` in case you are interested in checking them out.

## Managing Workloads

This section describes how to deploy and perform administrative's tasks using
systemd and BlueChi.

### Deploying Applications

BlueChi relies on three components to handle containerized applications:

* systemd
* Quadlet
* Podman

Application definitions are stored in `/etc/containers/systemd`. An application
needs two essential files:

* `{SERVICE_NAME}.kube`: Used by systemd to point to a Kubernetes resource definition
  containing the workload definition.

  Example of `freyja.kube`:
    ```kube
    # https://docs.podman.io/en/latest/markdown/podman-systemd.unit.5.html
    [Kube]
    Yaml=freyja.yml

    # Commented to disable the service to automatically start
    # [Install]
    # WantedBy=default.target
    ```
* `{SERVICE_NAME}.yaml`: A Kubernetes resource definition (either `v1.Pod` or
  `apps/v1.Deployment`) that describes the workload.

  Example of `freyja.yml`:
    ```yaml
    ---
    apiVersion: apps/v1
    kind: Deployment
    metadata:
      labels:
        app: freyja
      name: freyja
    spec:
      replicas: 1
      selector:
        matchLabels:
          app: freyja
      template:
        metadata:
          labels:
            app: freyja
        spec:
          hostNetwork: true
          containers:
            - name: local
              image: sdvblueprint.azurecr.io/sdvblueprint/eclipse-freyja/local-with-ibeji:0.1.0
              imagePullPolicy: IfNotPresent
    ```

If you edit the source code of a component then build and push an image of it to your container registry, you will need to edit the corresponding `{SERVICE_NAME}.yaml` file in the `/etc/containers/systemd` directory. The value of the `image` field in the `{SERVICE_NAME}.yaml` file should point to the image in your container registry.

Creating, changing or updating a file in `/etc/containers/systemd` requires you to run `systemctl daemon-reload` afterwards to generate the corresponding systemd unit files in
`/run/systemd/generator`.

### Service Lifecycle

Services can be managed by using `systemctl`, systemd's administrative CLI.

Starting, stopping, restarting services is as easy as:

* `systemctl stop {SERVICE_NAME}`
* `systemctl start {SERVICE_NAME}`
* `systemctl restart {SERVICE_NAME}`

> Make sure to run `systemctl daemon-reload` in case something changed in either Quadlet or systemd unit files.

### Monitoring and Logs

BlueChi's CLI (`bluechictl`), can be used to retrieve information from
managed nodes:
<https://github.com/eclipse-bluechi/bluechi/blob/main/doc/man/bluechictl.1.md>.

#### Using Systemctl

Simply run `systemctl status {SERVICE_NAME}` where `{SERVICE_NAME}` is the name of your .kube file.

#### Using journalctl

This is valid for any systemd defined service, simply run `journalctl -xeu {SERVICE_NAME}`

#### Podman

You can also list all active containers by running `podman ps` and then `podman logs {CONTAINER_NAME_OR_ID}` to
get logs from the container using podman. Replace `{CONTAINER_NAME_OR_ID}` with the container's name or ID.

## Running the Smart Trailer Example with BlueChi's devcontainer
Inside of the [devcontainer](#two-development-environments):
1. Follow the instructions in [Starting All the Services](#starting-all-the-services) to start up the in-vehicle stack.
1. Run the script `start_trailer_applications_bluechi.sh` to monitor for the trailer to be connected. It can be found at `in-vehicle-stack/scenarios/smart_trailer_use_case/scripts/start_trailer_applications_bluechi.sh`.
1. In another terminal window inside the devcontainer, start the `trailer-connected` service to simulate the trailer being connected:
    ```shell
    systemctl start trailer-connected
    ```
1. Verify the output in the terminal window of the `start_trailer_applications_bluechi.sh` script. You should see that two more services were started in response to the trailer being connected.
1. Use [Monitoring and Logs](#monitoring-and-logs) to check that the `smart-trailer` service is now receiving the value of the trailer weight every 10 seconds.
1. When you are ready to clean up, use the cleanup script mentioned in [Cleanup](#cleanup).
