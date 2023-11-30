#### Wheelchair Assistant Scenario Overview
In this scenario we try to provide a handicapped person an easier access to a vehicle and thus more mobility. Just as musicians in an orchestra must work together and communicate effectively to create beautiful music, various systems and technologies must work seamlessly together to provide complete support to the handicapped person. Our system detects that a handicapped person with a wheelchair is approaching a vehicle and aims to provide complete support by personalizing to the individual needs of the user. The system demonstrates a full driving scenario from arriving at the car to reaching the desired destination with maximum support from automation software.

A signal is raised to the orchestrator to start up the necessary providers and applications to manage the wheelchair as well as to make adjustments inside the car. These include a Digital Twin Provider, which exposes signals from the wheelchair to higher-level applications, and a Wheelchair Assistant application, which makes use of and reacts to these signals. In order to start our orchestra, the tools we use are: Eclipse Chariott, Ibeji, Agemo, Freyja, and an Eclipse Mosquitto MQTT broker.

##### Ankaios as Orchestrator

Ankaios is developed specifically for automotive use cases. It's independant of communication frameworks like SOME/IP, DDS or REST API.
It also does not depend on systemd or any specific systems and can be started with any init system.
With this it's possible to dynamically startup an application that is only required in a particular situation. Since Ankaios supports ASPICE processes, it can be used widely by leading car manufacturers.
For these reasons Ankaios fits in perfectly for the wheelchair scenario.

##### Dynamic Orchestration

![orchester.png](docs%2Fwheelchair_assistant_use_case%2Forchester.png)

This use case demonstrates a more complex example of dynamic orchestration.

At the beginning the car is parked and in the init state when the person arrives at the parking lot. While approaching, the person unlocks the car and it switches to the open state in which the door is unlocked. In order to simulate this, Digital Twin Provider "carkey_unlock_provider" sends the signal to set the cars state to open. Another Digital Twin Provider "wheelchair_distance_decreasing_provider" simulates the approaching process. Meanwhile a [script](./in-vehicle-stack/scenarios/wheelchair_assistant_use_case/scripts/) will be run to monitor Ibeji and detect when the handicapped person is approaching the vehicle. The script will continuously monitor the wheelchair distance from the vehicle and based on the distance read, automatic configuration of the car starts to make the entry experience as effortless as possible. This automatic configuration is implemented by the [Wheelchair Assistant Application](./in-vehicle-stack/scenarios/wheelchair_assistant_use_case/applications/wheelchair_assistant_application/). Through this application, the front and back doors open up, the steering wheel adjusts to a higher position and the driver seat is adjusted to the back and lowered. Once this is done, the car is in Hold state and ready to be turned on which is simulated by the "car_on_provider". Once the driver enters and turns on the ignition, the doors close and steering wheel and seat go back to their default state. Once the person arrives at the desired destination, the described process takes place in the reverse way to make sure they leave the car comfortably. This process is achieved by the "car_off_provider", "wheelchair_distance_increasing_provider" and the "car_lock_provider".

Every action and state change through transition is uploaded to the cloud through Freyja cloud syncronization and can be visualized by Azure.
All our services are registered by Chariott.

##### The Wheelchair Assistant Applications
 An explanation of the Wheelchair Assistant Applications can be found in [Wheelchair Assistant Application](./in-vehicle-stack/scenarios/wheelchair_assistant_use_case/applications/wheelchair_assistant_application/).

 ##### State machine overview

![wheel_assistant_states.png](docs%2Fwheelchair_assistant_use_case%2Fdiagrams%2Fwheel_assistant_states.png)

 ##### Architecture

![wheel_chair_scenario.png](docs%2Fwheelchair_assistant_use_case%2Fdiagrams%2Fwheel_chair_scenario.png)