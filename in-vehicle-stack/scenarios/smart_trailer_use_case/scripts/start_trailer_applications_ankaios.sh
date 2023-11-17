#!/bin/bash

# This script requires jq and grpcurl to be installed
# The commands for this can be added here

# Get the directory of where the script is located
# All relative paths will be in relation to this
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# The Ibeji gRPC server address
SERVER="0.0.0.0:5010"

# The Ibeji FindById gRPC service and method
SERVICE="invehicle_digital_twin.InvehicleDigitalTwin"
METHOD="FindById"

# The request body: The IsTrailerConnected signal
BODY='{"id":"dtmi:sdv:Trailer:IsTrailerConnected;1"}'

PROTO_PATH="${SCRIPT_DIR}/../../../interfaces/invehicle_digital_twin/v1"
PROTO="invehicle_digital_twin.proto"

EXPECTED_PROTOCOL="grpc"
EXPECTED_OPERATION="get"


# Call FindById in a loop until something is returned
while true; do
  OUTPUT=$(grpcurl -import-path $PROTO_PATH -proto $PROTO -plaintext -d "$BODY" $SERVER $SERVICE/$METHOD 2>&1)

  # Check if the output contains entityAccessInfo (the response from Ibeji when a provider is found)
  if echo "$OUTPUT" | grep -iq "EntityAccessInfo"
  then
    echo "The FindById call was successful. Output:"
    echo "$OUTPUT"
    break
  else
    echo "Provider not found: $OUTPUT"
    echo "The trailer is not connected. Retrying..."
    sleep 5 
    
  fi
done

# Parse the output as a JSON object using jq and extract the endpoints
ENDPOINTS=$(echo $OUTPUT | jq -c '.entityAccessInfo.endpointInfoList[]')

# Loop through each endpoint
for ENDPOINT in $ENDPOINTS
do
  # Check if protocol is what we expect
  if [[ $(echo $ENDPOINT | jq -r '.protocol' | tr '[:upper:]' '[:lower:]') == $EXPECTED_PROTOCOL ]]
  then
    OPERATIONS=$(echo $ENDPOINT | jq -r '.operations[]')
    # Loop through each operation and check if this endpoint supports the expected operation
    for OPERATION in $OPERATIONS
    do
      if [[ $(echo $OPERATION | tr '[:upper:]' '[:lower:]') == $EXPECTED_OPERATION ]]
      then
        URI=$(echo $ENDPOINT | jq -r '.uri')
        CONTEXT=$(echo $ENDPOINT | jq -r '.context')

        # We need the authority for the server, so remove the http://
        get_server=$(echo "$URI" | sed 's/http:\/\///g')

        # Call get for the "trailer connected provider" to check if it's connected
        GET_PROTO_PATH="${SCRIPT_DIR}/../interfaces"
        GET_PROTO="digital_twin_get_provider.proto"
        GET_SERVER=$get_server
        GET_SERVICE="digital_twin_get_provider.DigitalTwinGetProvider"
        GET_METHOD="Get"
        GET_OUTPUT=$(grpcurl -import-path $GET_PROTO_PATH -proto $GET_PROTO -plaintext $GET_SERVER $GET_SERVICE/$GET_METHOD 2>&1)

        # For now, this always returns true, this can be expanded to simulate connecting and disconnecting the trailer
        if [[ $(echo $GET_OUTPUT | jq -r '.propertyValue') ]]
        then
          echo "Trailer is connected! Starting workloads to manage it"

          # Start up the other workloads using podman
          CFG_PROVIDER=$'image: localhost/trailer_properties_provider_ank:latest\ncommandOptions: ["--network", "host", "--name", "trailer_properties_provider"]'
          CFG_APP=$'image: localhost/smart_trailer_application_ank:latest\ncommandOptions: ["--network", "host", "--name", "smart_trailer_application"]'

          ank run workload trailer_properties_provider --runtime podman --config "$CFG_PROVIDER" --agent agent_A
          ank run workload smart_trailer_application --runtime podman --config "$CFG_APP" --agent agent_A
        
          echo "Started the Trailer Properties Digital Twin Provider and Smart Trailer Application"
          exit 0
        fi
      fi
    done
  fi
done
# We didn't find an endpoint which satisfied our conditions
exit 1