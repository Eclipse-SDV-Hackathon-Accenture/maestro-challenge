# Eclipse Bluechi

Upstream documentation: https://bluechi.readthedocs.io/en/latest/

Bluechi is a Systemd controller that runs on top of Systemd to allow
multi-node workload management and dependency.

It allows both container and non-container workloads and uses Podman
as its container management tool.

It also realies on a tool called "Quadlet" which enables the usage of
Kubernetes YAML files to run container based workloads.

## Links

* [bluechi docs](https://bluechi.readthedocs.io/en/latest/)
* [bluechictl docs](https://github.com/eclipse-bluechi/bluechi/blob/main/doc/man/bluechictl.1.md)
* [podman](https://docs.podman.io/en/latest/)
* [podman and quadlet](https://www.redhat.com/sysadmin/quadlet-podman)


## Development Environment

The provided development environment provides the following components:

* Eclipse Chariott
* Eclipse Agemo
* Eclipse Kuksa (databroker)
* Eclipse Ibeji
* Eclipse Freyja
* Eclipse Bluechi
* Systemd
* Podman
* Quadlet

All services are running in the host networking meaning that those
can be accessed by using `localhost:$service_port`.

### Flavors

#### Devcontainer

Upstream documentation: https://containers.dev/

You can use devcotainers to start your containerized development environment by using the following
`devcontainer.json` file:

```json
{
    "privileged": true,
    "image": "quay.io/centos-sig-automotive/autosd-eclipse:latest",
    "overrideCommand": false
}
```

Or you can just clone the devcontainer template repository: https://gitlab.com/CentOS/automotive/container-images/templates/devcontainer (you will need to change the image to `quay.io/centos-sig-automotive/autosd-eclipse:latest` if so).

#### Containers

Commands in this section are using `podman` but it can be replaced by `docker`.

Upstream documentation:

* https://docs.podman.io/en/latest/
* https://docs.docker.com/

Start the container by running:

```sh
podman run \
--rm \
--name autosd-eclipse \
--privileged \
quay.io/centos-sig-automotive/autosd-eclipse:latest
```

You can now enter into the container and start playing with bluechi:

```sh
podman exec -it autosd-eclipse /bin/bash
``` 

### Managing Workloads

This section describes how to deploy and perform adminstratives tasks using systemd and bluechi.

#### Deploying Applications

Bluechi relies on three components to handler containerized applications: Systemd, quadlet and podman.

Application definitions are stored in `/etc/containers/systemd`, you can see that
all other services are defined in that folder.

An application needs two essential files:

* `$service.kube`: used by systemd to point to a kubernetes yaml file with the actual app definiton.
* `service.yaml`: A kubernetes YAML file (either `v1.Pod` or `apps/v1.Deployment`) that will run your sevices/application.

Changing or updating such files need a `systemctl daemon-reload`.

You can see all generated Systemd unit files in `/run/systemd/generator`. 

#### Service Lifecycle

Services can be managed by using `systemctl`, Systemd's admin cli.

Starting, stopping, restarting services is as easy as:

* systemctl stop $svc
* systemc start $svc
* systemctl restart $svc

You will need to reload Systemd's daemon in case something changed in your service
definition files (Kubernetes YAML/Systemd Unit files):

* `systemctl daemon-reload`

#### Monitoring and Logs

You can use systemctl to read your service logs: `systemctl logs $service`.

Podman can also be used to list container processes by running `podman ps` and then `podman logs $container_id_or_name` can be
used to read logs from a container.

Bluechi's cli (`bluechictl`), can also be used to proxy all systemctl as well as its own node monitoring tools: https://github.com/eclipse-bluechi/bluechi/blob/main/doc/man/bluechictl.1.md.
