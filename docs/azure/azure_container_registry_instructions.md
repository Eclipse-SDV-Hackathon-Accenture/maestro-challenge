# Azure Container Registry 

Azure Container Registry is a managed registry service based on the open-source Docker Registry 2.0 allowing you to create and maintain your container images in Azure.

We have pre-built amd64 and arm64 container images for the in-vehicle software stack comprised by Eclipse Ibeji, Eclipse Agemo, Eclipse Freyja, and Eclipse Chariott so that you can focus on building your own scenarios.

## Maestro Challenge Azure Container Registry

You can access the pre-built container images here: 
* sdvblueprint.azurecr.io

You will be prompted for username and password to pull the pre-built container images.
* username: 

> **Note** 
please contact the Maestro Hack Coaches for the password.

## How to use 

You can pull the images using [Docker](https://docs.docker.com/engine/reference/commandline/pull/) or [Podman](https://docs.podman.io/en/latest/markdown/podman-pull.1.html) as shown below:

When you run the below command, you will be prompted for username and password. Please use the username provided above and contact the Maestro Challenge hack coaches for the password.

> **Podman**
```
podman pull sdvblueprint.azurecr.io/sdvblueprint/eclipse-agemo/pub-sub-service:0.1.0
podman pull sdvblueprint.azurecr.io/sdvblueprint/eclipse-chariott/service-discovery:0.1.1
podman pull sdvblueprint.azurecr.io/sdvblueprint/eclipse-ibeji/invehicle-digital-twin:0.1.0
podman pull sdvblueprint.azurecr.io/sdvblueprint/eclipse-mosquitto/mqtt-broker
podman pull sdvblueprint.azurecr.io/sdvblueprint/eclipse-freyja/local-with-ibeji:0.1.0
podman pull sdvblueprint.azurecr.io/sdvblueprint/eclipse-freyja/cloud-with-ibeji:0.1.0
podman pull sdvblueprint.azurecr.io/sdvblueprint/eclipse-freyja/azure-cloud-connector:0.1.0
```

> **Docker**
```
docker pull sdvblueprint.azurecr.io/sdvblueprint/eclipse-agemo/pub-sub-service:0.1.0
docker pull sdvblueprint.azurecr.io/sdvblueprint/eclipse-chariott/service-discovery:0.1.1
docker pull sdvblueprint.azurecr.io/sdvblueprint/eclipse-ibeji/invehicle-digital-twin:0.1.0
docker pull sdvblueprint.azurecr.io/sdvblueprint/eclipse-mosquitto/mqtt-broker
docker pull sdvblueprint.azurecr.io/sdvblueprint/eclipse-freyja/local-with-ibeji:0.1.0
docker pull sdvblueprint.azurecr.io/sdvblueprint/eclipse-freyja/cloud-with-ibeji:0.1.0
docker pull sdvblueprint.azurecr.io/sdvblueprint/eclipse-freyja/azure-cloud-connector:0.1.0
```

> **Note** 
Ensure you use the full domain name of the container registry.

## Aditional Information

If you would like to know more about Azure Container Registry, extensive documentation and examples are available [Introduction to Container registries in Azure](https://learn.microsoft.com/en-us/azure/container-registry/container-registry-intro) 