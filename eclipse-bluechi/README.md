# Eclipse BlueChi

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

## Development Environment

The development environment provides the following components:

* Eclipse Chariott
* Eclipse Agemo
* Eclipse Kuksa (databroker)
* Eclipse Ibeji
* Eclipse Freyja
* Eclipse BlueChi
* systemd
* Podman
* Quadlet

All services are accessible via `localhost:$port`.

### Flavors

#### devcontainer

Upstream documentation: <https://containers.dev/>

You can use devcotainers to start your containerized development environment by
using the following `devcontainer.json` file:

```json
{
    "privileged": true,
    "image": "quay.io/centos-sig-automotive/autosd-eclipse:latest",
    "overrideCommand": false
}
```

Or you can just clone the devcontainer template repository:
<https://gitlab.com/CentOS/automotive/container-images/templates/devcontainer>
and change the image to `quay.io/centos-sig-automotive/autosd-eclipse:latest`.

#### Containers

Upstream documentation: <https://docs.podman.io/en/latest/>

Start the container by running:

```sh
podman run \
--rm \
--name autosd-eclipse \
--privileged \
quay.io/centos-sig-automotive/autosd-eclipse:latest
```

Enter into the container and interact with BlueChi:

```sh
podman exec -it autosd-eclipse /bin/bash
bluechictl list-units
```

### Managing Workloads

This section describes how to deploy and perform adminstratives tasks using
systemd and BlueChi.

#### Deploying Applications

BlueChi relies on three components to handle containerized applications:

* systemd
* Quadlet
* Podman

Application definitions are stored in `/etc/containers/systemd`. An application
needs two essential files:

* `service.kube`: Used by systemd to point to a Kubernetes resource definition
  containing the workload definition.
* `service.yaml`: A Kubernetes resource definition (either `v1.Pod` or
  `apps/v1.Deployment`) that describes the workload.

Changing or updating a file in `/etc/containers/systemd` requires a `systemctl
daemon-reload` afterwards to generate the corresponding systemd unit files in
`/run/systemd/generator`.

#### Service Lifecycle

Services can be managed by using `systemctl`, systemd's administrative CLI.

Starting, stopping, restarting services is as easy as:

* `systemctl stop $svc`
* `systemc start $svc`
* `systemctl restart $svc`

> Make sure to run `systemctl daemon-reload` in case something changed in either
> the Quadlet files or systemd units.

#### Monitoring and Logs

To read your service logs run `systemctl logs $service`. Alternatively Podman
can be used to list container processes and their ID's or name by running
`podman ps`. Then run `podman logs $container_id_or_name` to read logs from a
container.

BlueChi's CLI (`bluechictl`), can also be used to retrieve information from
managed nodes:
<https://github.com/eclipse-bluechi/bluechi/blob/main/doc/man/bluechictl.1.md>.
