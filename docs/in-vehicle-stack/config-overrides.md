## How to Override Configuration for In-Vehicle Stack Containers

The following In-Vehicle Stack services have configuration that can be overridden:

- Eclipse Agemo:
    - [Configuration](https://github.com/eclipse-chariott/Agemo/blob/main/docs/config-overrides.md)
- Eclipse Freyja:
    - [Mapping Configuration](https://github.com/eclipse-ibeji/freyja/blob/main/mapping_clients/in_memory_mock_mapping_client/README.md)
    - [Cloud Connector Configuration](https://github.com/eclipse-ibeji/ibeji-example-applications/blob/main/cloud_connectors/azure/digital_twins_connector/src/core/adt_instance_config.sample.json)
- Eclipse Ibeji:
    - [In-Vehicle Digtial Twin Configuration](https://github.com/eclipse-ibeji/ibeji/blob/main/core/invehicle-digital-twin/template/invehicle_digital_twin_settings.yaml)
    - [Managed Subscribe Configuration](https://github.com/eclipse-ibeji/ibeji/blob/main/core/module/managed_subscribe/template/managed_subscribe_settings.yaml)

Please refer to the above links to determine what configuration files you would like to override. Then follow the steps below for the specific orchestrator you are using.

### Override In-Vehicle Stack configuration in Eclipse Ankaios

#### Prerequisites

- Follow steps to setup the [Ankaios Development Environment](../../eclipse-ankaios/README.md#ankaios-maestro-challenge-development-environment)

#### Steps

1. In the Ankaios devcontainer, open a terminal and create a directory under `/etc` to store the
desired project configuration files:

    ```shell
    mkdir /etc/<project_name>/config
    ```

    for example:

    ```shell
    mkdir /etc/freyja/config
    ```

1. Create the desired configuration files in the newly created directory:

    ```shell
    touch <config_file_name>.<ext>
    ```

    for example, to create a configuration file to override the mapping configuration for Freyja:

    ```shell
    touch mock_mapping_config.json
    ```

1. Populate the newly created configuration files with the configuration you wish to override using
any text editor. 

1. Next, we need to pass the configuration into the containers. To do this, we will need to modify
[startupState.yaml](../../eclipse-ankaios/config/startupState.yaml). This will be done by mounting the
directory created in step 1 into the container using the `commandOptions` parameter list. Find the
entry for the project you are trying to override config for and add following to the options:

    ```yaml
    "--mount", "type=bind,src=/etc/<project_name>/config,dst=/mnt/config,ro=true"
    ```

    >Note: Replace `<project_name>` with the project you are overriding config for.

    for example:

    ```yaml
    runtimeConfig: |
      image: sdvblueprint.azurecr.io/sdvblueprint/eclipse-freyja/local-with-ibeji:0.1.0
      commandOptions: ["--network", "host", "--name", "cloud-sync", "--mount", "type=bind,src=/etc/freyja/config,dst=/mnt/config,ro=true"]
    ```

The In-Vehicle Stack service will now use your modified configuration. Note that any configuration
changes will require a restart of the in-vehicle stack. You can restart the in-vehicle stack by following the steps for
[Ankaios](../../eclipse-ankaios/README.md#startup-check-before-development).

### Override In-Vehicle Stack configuration in Eclipse BlueChi

#### Prerequisites

- Follow steps to setup [Eclipse BlueChi](../../eclipse-bluechi/README.md)

#### Steps

TODO: Add steps for overridding configuration within Eclipse BlueChi
