## In-Vehicle Stack with Cloud Connectivity

The default in-vehicle stack is all local with no cloud connection. Follow the below steps to
enable cloud connection with Azure Digital Twins using Eclipse Freyja.

### Create Azure Digital Twins Instance

1. Please refer to the following documentation to setup the Azure Digital Twins instance: [Automated Azure Digital Twins Setup for Smart Trailer Example](https://github.com/eclipse-ibeji/ibeji-example-applications/blob/main/cloud_connectors/azure/digital_twins_connector/README.md#automated-azure-digital-twins-setup-for-smart-trailer-example).
This will create the Azure Digital Twins with the appropriate DTDL for the smart trailer use case.

1. Once you have created the Azure Digital Twins instance, follow
[Open instance in Azure Digital Twins Explorer](https://learn.microsoft.com/en-us/azure/digital-twins/quickstart-azure-digital-twins-explorer#open-instance-in-azure-digital-twins-explorer)
to get the url for your instance. Copy this url for later.

### Change In-Vehicle Stack to use Cloud-Connected Components

The following changes will vary depending on which Orchestrator you are using. Broadly, we need to
change the Freyja container image from the in-memory version to the cloud version and add a
workload for the Azure cloud connector used by Freyja.

#### Eclipse Ankaios

1. Change the `digital_twin_cloud_sync` workload in
[startupState.yaml](../../eclipse-ankaios/config/startupState.yaml) to point to the `cloud-with-ibeji`
container image. The workload entry should be updated to:

    ```yaml
    digital_twin_cloud_sync:
      runtime: podman
      agent: agent_A
      workloads:
        value: freyja
      runtimeConfig: |
        image: sdvblueprint.azurecr.io/sdvblueprint/eclipse-freyja/cloud-with-ibeji:0.1.0
        commandOptions: ["--network", "host", "--name", "cloud-sync"]
    ```

1. Follow the steps under
[Override In-Vehicle Stack configuration in Eclipse Ankaios](./config-overrides.md#override-in-vehicle-stack-configuration-in-eclipse-ankaios)
to create a config directory with the path `/etc/freyja/config` and then create the
[adt_instance_config.json](https://github.com/eclipse-ibeji/ibeji-example-applications/blob/main/cloud_connectors/azure/digital_twins_connector/src/core/adt_instance_config.sample.json)
file. Replace the `AzureDigitalTwinsInstanceUrl` field with the URL to your Azure Digital Twin
instance that you obtained in Step 2 of
[Create Azure Digital Twins Instance](#create-azure-digital-twins-instance).

1. Uncomment the `cloud_connector` workload in
[startupState.yaml](../../eclipse-ankaios/config/startupState.yaml). The workload entry should be:

    ```yaml
    cloud_connector:
    runtime: podman
    agent: agent_A
    restart: true
    updateStrategy: AT_MOST_ONCE
    accessRights:
      allow: []
      deny: []
    runtimeConfig: |
      image: sdvblueprint.azurecr.io/sdvblueprint/eclipse-freyja/azure-cloud-connector:0.1.0
      commandOptions: ["--network", "host", "--name", "cloud-connector", "--mount", "type=bind,src=/etc/freyja/config,dst=/mnt/config,ro=true"]
    ```

1. Start the In-Vehicle Stack following the steps for
[Ankaios](../../eclipse-ankaios/README.md#startup-check-before-development).

1. Once you have started up the service, you will need to authenticate with Azure for the Cloud
Connector to establish connection. Run:

    ```shell
    podman logs cloud-connector
    ```

    The most recent log should be a device code auth request message. Copy the code from the
    message and open a browser to the provided URL. Paste the copied code and sign in to the
    account authenticated with your Azure Digital Twins instance:

    Here's an example of the log message:

    ```shell
    To sign in, use a web browser to open the page https://microsoft.com/devicelogin and enter the code <DEVICE_CODE> to authenticate.
    ```

2. Validate you have successfully logged in by re-running

    ```shell
    podman logs cloud-connector
    ```

    At the bottom of the logs, you should see that the output is similar to the output below:

    ```shell
    [2023-11-20T23:27::18Z] info: Main[0]
      Started the Azure Digital Twins Connector
    ```

    If you see the device code login text, wait 10 seconds and try the podman command again.

7. The In-Vehicle Stack is now initialized and connected to the cloud!

#### Eclipse BlueChi

1. In the BlueChi devcontainer, modify the `/usr/local/bin/bluechi-env-bootstrap` to remove `freyja` (this image includes the in-memory mock version of Freyja) and use `cloud-with-ibeji` and `azure-cloud-connector` services instead:
    - Add the following two lines:
        ```shell
        systemctl start cloud-with-ibeji
        systemctl start azure-cloud-connector
        ```
    - Your complete file should now look like:
      ```shell
      #!/bin/bash

      /usr/local/bin/podman-quadlet-image-sync

      systemctl start chariott
      systemctl start mosquitto
      systemctl start ibeji
      systemctl start agemo
      systemctl start cloud-with-ibeji
      systemctl start azure-cloud-connector
      ```

1. Follow the steps under
[Override In-Vehicle Stack configuration in Eclipse BlueChi](./config-overrides.md#override-in-vehicle-stack-configuration-in-eclipse-bluechi)
to create a config directory with the path `/etc/freyja/config` and then create the
[adt_instance_config.json](https://github.com/eclipse-ibeji/ibeji-example-applications/blob/main/cloud_connectors/azure/digital_twins_connector/src/core/adt_instance_config.sample.json)
file. Replace the `AzureDigitalTwinsInstanceUrl` field with the URL to your Azure Digital Twin
instance that you obtained in Step 2 of
[Create Azure Digital Twins Instance](#create-azure-digital-twins-instance).

1. The `/etc/containers/systemd/azure-cloud-connector.yml` file has the volumeMount configuration specified:

    ```yaml
    containers:
        - name: app
          image: sdvblueprint.azurecr.io/sdvblueprint/eclipse-freyja/azure-cloud-connector:0.1.0
          imagePullPolicy: IfNotPresent
          volumeMounts:
          - mountPath: /mnt/config
            name: freyjaconfig
      volumes:
        - name: freyjaconfig
          hostPath:
            path: /etc/freyja/config
            type: DirectoryOrCreate
    ```

1. Start the In-Vehicle Stack following the steps for
[BlueChi](../../eclipse-bluechi/README.md#running-the-smart-trailer-example-with-bluechis-devcontainer).

1. Once you have started up the service, you will need to authenticate with Azure for the Cloud
Connector to establish connection. Run:

    ```shell
    systemctl status azure-cloud-connector
    ```

    The most recent log should be a device code auth request message. Copy the code from the
    message and open a browser to the provided URL. Paste the copied code and sign in to the
    account authenticated with your Azure Digital Twins instance:

    Here's an example of the log message:

    ```shell
    To sign in, use a web browser to open the page https://microsoft.com/devicelogin and enter the code <DEVICE_CODE> to authenticate.
    ```

2. Validate you have successfully logged in by re-running

    ```shell
    systemctl status azure-cloud-connector
    ```

    At the bottom of the logs, you should see that the output is similar to the output below:

    ```shell
    [2023-11-20T23:27::18Z] info: Main[0]
      Started the Azure Digital Twins Connector
    ```

    If you see the device code login text, wait 10 seconds and try the podman command again.

7. The In-Vehicle Stack is now initialized and connected to the cloud!
