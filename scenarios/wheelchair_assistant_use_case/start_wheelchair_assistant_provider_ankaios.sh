#!/bin/bash
# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.
# SPDX-License-Identifier: MIT

set -e

# Deklariere das Array
PROVIDER_CONTAINERS=(
  "car_off_provider"
  "car_on_provider"
  "carkey_lock_provider"
  "carkey_unlock_provider"
  "wheelchair_distance_decreasing_provider"
  "wheelchair_distance_increasing_provider"
)

PROVIDER_CONTAINERS=(
  "car_off_provider"
)

# Get the directory of where the script is located
# All relative paths will be in relation to this
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Iterate over provider containers to build
for CONTAINER in "${PROVIDER_CONTAINERS[@]}"; do
  CFG_PROVIDER="image: sdvblueprint.azurecr.io/sdvblueprint/in-vehicle-stack/--${CONTAINER}:0.1.0\ncommandOptions: [\"--network\", \"host\", \"--name\", \"${CONTAINER}\"]"
  echo "${CFG_PROVIDER}"
  ank run workload ${CONTAINER} --runtime podman --config "$CFG_PROVIDER" --agent agent_A
done
