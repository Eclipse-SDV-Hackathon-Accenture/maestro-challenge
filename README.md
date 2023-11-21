
# Maestro Challenge 🚗💻🎶
- [About](#about)
- [Sample Scenarios](#sample-scenarios)
    - [Provided Sample Scenario](#provided-sample-scenario)
    - [Other Sample Scenarios](#other-sample-scenarios)
    - [Useful References for Creating and Enhancing Sample Scenarios](#useful-references-for-creating-and-enhancing-sample-scenarios)
- [Getting Started](#getting-started)
    - [Need to know](#need-to-know)
    - [Eclipse Ankaios Orchestrator](#eclipse-ankaios-orchestrator)
    - [Eclipse BlueChi Orchestrator](#eclipse-bluechi-orchestrator)
- [In-Vehicle Software Stack Overview](#in-vehicle-software-stack-overview)
- [Projects Involved](#projects-involved)
    - [In-Vehicle Software Stack](#in-vehicle-software-stack)
    - [In-Vehicle Software Orchestrators](#in-vehicle-software-orchestrators)
- [Hack Coaches](#hack-coaches)

**Do you want to be the next maestro of the next generation of vehicle software? The time is now!**

Imagine yourself as the **maestro**. You are not just writing code. You are composing a masterpiece that will drive the future of transportation. Your work will ensure that every component, from the engine control unit to the infotainment system, works in perfect harmony. So, step up to the podium, take a deep breath, and let your creativity flow. The stage is set for you to become the maestro of in-vehicle software 🚗💻🎶.

**Come hack with us!**

![Maestro](docs/diagrams/orchestra_picture.jpg)
>Photo by <a href="https://unsplash.com/@gwundrig?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash">Manuel Nägeli</a>.

## About

We supply an in-vehicle stack with software orchestrators. Your assignment is to utilize our in-vehicle stack and software orchestrators to construct your own scenario or replicate the scenarios provided in our [Sample Scenarios](#sample-scenarios).

The tech stack in this challenge showcases complex in-vehicle services and workloads, utilizing the vehicle’s computing resources and capabilities, as well as other in-vehicle applications.

Enjoy the process of bringing your vision to life!

## Sample Scenarios

Here is a list of potential scenarios your team could develop. Feel free to invent your own scenarios too. Let your creativity shine ☀️ and have fun!

### Provided Sample Scenario

Scenario description:
- The system detects that a smart trailer is being connected to a car. A signal is raised to the orchestrator to start up the necessary providers and applications to manage the smart trailer. Some example applications could include displaying the signals or adjusting the body functions and powertrain accordingly to ensure a smooth trip.

The first diagrams in the [Eclipse Ankaios](./eclipse-ankaios/README.md) and [Eclipse BlueChi](./eclipse-bluechi/README.md) depict this provided sample scenario.

We provide the code for this scenario. Please refer to {TODO: INSERT_REFERENCE} for instructions on running this scenario. **Your task is to develop a Smart Trailer Application** using the vehicle signals emitted from our Smart Trailer Digital Twin provider.

TODO: Provide some ideas to help the user build this Smart Trailer application. Also maybe provide some additional challenges, such as mentioning what you can do when the signals are synced to the cloud. For instance, create a web UI that displays the signals.

### Other Sample Scenarios
- Leverage OpenAI to enhance the vehicle’s user experience. You could develop an application that uses OpenAI’s GPT model to power an in-vehicle virtual assistant.

- Consider using OpenAI to enhance our software orchestrators, effectively creating intelligent orchestrators! OpenAI could optimize workload placements across compute nodes and/or cloud based on factors such compute usage, memory usage, network coverage, and latency.

- Create a web user interface application, such as a dashboard, that displays the various workloads' health.

### Useful References for Creating and Enhancing Sample Scenarios

This section offers guidance for creating a new sample scenario or improving the [Provided Sample Scenario](#provided-sample-scenario). While it does not provide a comprehensive list of resources, it aims to steer you in the right direction. If this section does not provide the guidance you need, please refer to the respective project’s documentation. See [Projects Involved](#projects-involved) for the project links.

TODO: This section needs to be refined. The goal is to include key references to help users easily identify the documentation they need to enhance a scenario or create their scenario. If this section gets too long, we can place the contents in another document and reference it here.

TODO - Some example references that may need to be included:
#### Ibeji
- [How do I create an in-vehicle digital twin model?](https://github.com/eclipse-ibeji/ibeji/blob/main/docs/tutorials/in_vehicle_model/README.md)
    > This helps you construct or enhance an in-vehicle digital twin model, and enables you to reference your in-vehicle digital twin model in your code.
- [How do I create a digital twin provider or add additional capabilities to an existing provider?](https://github.com/eclipse-ibeji/ibeji/blob/main/docs/tutorials/in_vehicle_model/README.md)
    > A digital twin provider exposes a subset of the in-vehicle's hardware capabilities. This enables digital twin consumers to utilize that subset. Also you may find that you want to add additional capabilities, such as a new in-vehicle signal, to an existing digital twin provider. When you add new capabilities to a provider, such as a new in-vehicle signal, you will need to update the mapping client's configuration or the mapping service that you are using with Freyja. See the [Freyja FAQ](#freyja) for more details.
    - [How do I build a container image for my digital twin provider?](https://github.com/eclipse-ibeji/ibeji/blob/main/samples/container/README.md#provider)
        > You will need to build a container image if you are updating the smart trailer digital twin provider's source code or creating your own digital twin provider.
    - [If my digital twin provider is running in a container, how do I override its configuration file?](https://github.com/eclipse-ibeji/ibeji/blob/main/samples/container/README.md#run)
        > You will not need to rebuild the container image if you are overriding your digital twin provider's configuration file.
    - [How do I use the Managed Subscribe module?](https://github.com/eclipse-ibeji/ibeji/blob/main/samples/managed_subscribe/README.md)
        > The benefit of using the Managed Subscribe module in your digital twin provider allows your digital twin consumers to specify the frequency at which they want to receive updates.
- [How do I create a digital twin consumer?](https://github.com/eclipse-ibeji/ibeji/blob/main/docs/tutorials/consumer/README.md)
    > A digital twin consumer is a software entity that interfaces with the digital representation of the in-vehicle hardware components.
    - [How do I build a container image for my digital twin consumer?](https://github.com/eclipse-ibeji/ibeji/blob/main/samples/container/README.md#consumer)
        > You will need to build a container image if you are updating the smart trailer digital twin consumer's source code or creating your own digital twin consumer.
    - [If my digital twin consumer is running in a container, how do I override its configuration file?](https://github.com/eclipse-ibeji/ibeji/blob/main/samples/container/README.md#run-1)
        > You will not need to rebuild the container image if you are overriding your digital twin consumer's configuration file.
#### Chariott
- [How can I use the Service Discovery to register and discover other applications/services?](https://github.com/eclipse-chariott/chariott/blob/main/service_discovery/README.md)
    > An application can utilize the Service Discovery to register with the system and enable other applications to discover it via the Service Discovery system.
#### Freyja
- [How do I configure new mappings for Freyja's in-memory mapping client?](https://github.com/eclipse-ibeji/freyja/blob/main/mapping_clients/in_memory_mock_mapping_client/README.md)
    > Adding a new in-vehicle signal requires you to configure a new mapping for that in-vehicle signal.
- [How do I override the mapping configuration?](https://github.com/eclipse-ibeji/freyja/blob/main/docs/config-overrides.md)
- [How do I sync in-vehicle signals to the cloud?](https://github.com/eclipse-ibeji/ibeji-example-applications/blob/main/cloud_connectors/azure/digital_twins_connector/README.md)
    > You may want to sync your in-vehicle signals to a cloud digital representation of your in-vehicle.
    - TODO - If a user wishes to integrate their scenario with Azure, which image would I need to pull to enable Freyja to sync to the cloud?
- TODO - Add link to documentation outlining how to to override configuration for in-vehicle-stack containers'

#### Eclipse Ankaios

- [Quickstart](https://eclipse-ankaios.github.io/ankaios/main/usage/quickstart/)
- [User documentation](https://eclipse-ankaios.github.io/ankaios/main/)
- [Working with the startup configuration](https://eclipse-ankaios.github.io/ankaios/main/reference/startup-configuration/)
- [Architecture](https://eclipse-ankaios.github.io/ankaios/main/architecture/)
- [Suggestions for improvement & Feedback](https://github.com/eclipse-ankaios/ankaios/discussions)

## Getting Started

Please note that it is not necessary to use both software orchestrators. You can choose either one to implement your scenario.

### Need to know
- You need basic knowledge about containerization technologies and tools (e.g. Docker, Podman), but if you have not dealt with it yet, don't worry, just check out a little tutorial (https://docs.docker.com/get-started/) to get a basic understanding of containers and you are prepared.
- Basic skills to deal with distributed systems

### Eclipse Ankaios Orchestrator

If you have decided to use Ankaios, you will find an easy to use development environment in the subfolder [eclipse-ankaios](./eclipse-ankaios/README.md),
which you can use for all maestro challenges.

### Eclipse BlueChi Orchestrator

If you have decided to use BlueChi, you will find an easy to use development environment in the subfolder [eclipse-bluechi](./eclipse-bluechi/README.md),
which you can use for all maestro challenges.

## In-Vehicle Software Stack Overview

The in-vehicle stack comprises Eclipse Ibeji, Eclipse Agemo, Eclipse Freyja, and Eclipse Chariott. This stack enables a universal vehicle model to be used across different vehicles, dynamic management of vehicle signal topics for publishing and subscribing, synchronization of in-vehicle signals to a cloud-based digital twin, and the development of applications without the need for specific knowledge about the location of the resources they use.

We provide two software orchestrators, Ankaios and BlueChi, to orchestrate the in-vehicle stack. Feel free to choose either for this hackathon challenge.

## Projects Involved

### In-Vehicle Software Stack
- [Eclipse Agemo](https://github.com/eclipse-chariott/Agemo): Agemo incorporates a Pub Sub Service, a [gRPC](https://grpc.io/docs/what-is-grpc/introduction/) service that facilitates publish/subscribe operations for in-vehicle applications, including but not limited to Eclipse Ibeji and Eclipse Chariott. This service has the capability to register with Chariott, enhancing its discoverability by other applications such as Eclipse Ibeji. It offers dynamic creation and management of topics.

- [Eclipse Chariott](https://github.com/eclipse-chariott/chariott): Chariott operates as a [gRPC](https://grpc.io/docs/what-is-grpc/introduction/) service, offering a unified interface for application interaction. Chariott enables [Service Discovery](https://github.com/eclipse-chariott/chariott/blob/main/service_discovery/README.md), allowing provider applications to promote their capabilities by registering with Chariott’s service registry. Consumer applications in need of specific resources and capabilities can discover them via Chariott’s service registry.

- [Eclipse Freyja](https://github.com/eclipse-ibeji/freyja/): Freyja enables seamless synchronization between the vehicle’s digital twin and its cloud-based digital twin. This synchronization allows for a consistent and unified digital representation of the vehicle across both platforms.

- [Eclipse Ibeji](https://github.com/eclipse-ibeji/ibeji): Ibeji is designed with the goal of enabling a digital depiction of the vehicle’s state and capabilities. It achieves this through an adaptable, open, and dynamic architecture that provides access to the vehicle’s hardware, sensors, and capabilities. This extensible framework allows for a comprehensive and accurate representation of the vehicle’s current status and potential functionalities.

### In-Vehicle Software Orchestrators
- [Eclipse Ankaios](https://eclipse-ankaios.github.io/ankaios): Ankaios provides workload and container orchestration for automotive High Performance Computing (HPC) software . While it can be used for various fields of applications, it is developed from scratch for automotive use cases and provides a slim yet powerful solution to manage containerized applications. It supports various container runtimes with Podman as the first one, but other container runtimes and even native applications can be supported. Eclipse Ankaios is independent of existing communication frameworks like SOME/IP, DDS, or REST API.

- [Eclipse BlueChi](https://github.com/containers/bluechi): BlueChi is a systemd service controller intended for multi-node environments with a predefined number of nodes and with a focus on highly regulated ecosystems such as those requiring functional safety. Potential use cases can be found in domains such as transportation, where services need to be controlled across different edge devices and where traditional orchestration tools are not compliant with regulatory requirements.

## Hack Coaches
who can be contacted for this challenge for questions etc. (Slack-handle)

- Eclipse Agemo, Chariott, Ibeji and Freyja: Jordan Chiu (Slack handle: @Jordan Chiu)
- Eclipse Agemo, Chariott, Ibeji and Freyja: Filipe Prezado (Slack handle: @fprezado)
- Eclipse Ankaios: Chatree Akasarn (Slack handle: @Chatree Akasarn)
- Eclipse Ankaios: Oliver Klapper (Slack handle: @Oliver Klapper)
- Eclipse BlueChi: Leonardo Rossetti (Slack handle: @Leonardo Rossetti)

## All Necessary Links
