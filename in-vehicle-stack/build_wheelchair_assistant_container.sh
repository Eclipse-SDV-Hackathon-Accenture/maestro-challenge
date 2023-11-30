#!/bin/bash

# Deklariere das Array
PROVIDER_CONTAINERS=(
  "car_off_provider"
  "car_on_provider"
  "carkey_lock_provider"
  "carkey_unlock_provider"
  "wheelchair_distance_decreasing_provider"
  "wheelchair_distance_increasing_provider"
)

APPLICATION_CONTAINERS=(
  "wheelchair_distance_application"
)

# Iterate over provider containers to build
for CONTAINER in "${PROVIDER_CONTAINERS[@]}"; do
  podman build -t sdvblueprint.azurecr.io/sdvblueprint/in-vehicle-stack/${CONTAINER}:0.1.0 -f "scenarios/wheelchair_assistant_use_case/digital_twin_providers/${CONTAINER}/Dockerfile" .
done

# Iterate over provider containers to build
for CONTAINER in "${APPLICATION_CONTAINERS[@]}"; do
  podman build -t sdvblueprint.azurecr.io/sdvblueprint/in-vehicle-stack/${CONTAINER}:0.1.0 -f "scenarios/wheelchair_assistant_use_case/applications/${CONTAINER}/Dockerfile" .
done