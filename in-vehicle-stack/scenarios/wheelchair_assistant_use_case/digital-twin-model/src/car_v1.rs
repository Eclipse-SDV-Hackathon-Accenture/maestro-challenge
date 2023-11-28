// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.
// SPDX-License-Identifier: MIT

// Note: This code was manually written based on the structure of the
// vehicle model in "../dtdl/car.json"
// In the future this code could be generated from a DTDL spec.

enum WheelchairAssistantState {
    INIT,
    OPEN,
    HOLD,
    DRIVE
}

pub mod car {
    pub mod is_car_door_open {
        pub const ID: &str = "dtmi:sdv:Car:IsDoorOpen;1";
        pub const NAME: &str = "IsDoorOpen";
        pub const DESCRIPTION: &str = "Is door open?";
        pub type TYPE = bool;
    }

    pub mod is_car_steeringwheel_in_assist_position {
        pub const ID: &str = "dtmi:sdv:Car:IsSteeringwheelInAssistPosition;1";
        pub const NAME: &str = "IsSteeringwheelInAssistPosition";
        pub const DESCRIPTION: &str = "Is steering wheel in assist position?";
        pub type TYPE = bool;
    }

    pub mod is_car_running {
        pub const ID: &str = "dtmi:sdv:Car:IsCarRunning;1";
        pub const NAME: &str = "IsCarRunning";
        pub const DESCRIPTION: &str = "Is car running?";
        pub type TYPE = bool;
    }

    pub mod is_car_seat_in_assist_position {
        pub const ID: &str = "dtmi:sdv:Car:IsSeatInAssistPosition;1";
        pub const NAME: &str = "IsSeatInAssistPosition";
        pub const DESCRIPTION: &str = "Is seat in assist position?";
        pub type TYPE = bool;
    }

    pub mod is_car_unlocked {
        pub const ID: &str = "dtmi:sdv:Car:IsCarUnlocked;1";
        pub const NAME: &str = "IsCarUnlocked";
        pub const DESCRIPTION: &str = "Is car unlocked?";
        pub type TYPE = bool;
    }

    pub mod car_wheelchair_distance {
        pub const ID: &str = "dtmi:sdv:Car:WheelchairDistance;1";
        pub const NAME: &str = "WheelchairDistance";
        pub const DESCRIPTION: &str = "Distance of wheelchair to car in cm";
        pub type TYPE = i32;
    }

    pub mod car_wheelchair_assistant_state {
        pub const ID: &str = "dtmi:sdv:Car:WheelchairAssistantState;1";
        pub const NAME: &str = "WheelchairAssistantState";
        pub const DESCRIPTION: &str = "Wheelchair assistant state. One of INIT, OPEN, HOLD, DRIVE";
        pub type TYPE = WheelchairAssistantState;
      }
}
