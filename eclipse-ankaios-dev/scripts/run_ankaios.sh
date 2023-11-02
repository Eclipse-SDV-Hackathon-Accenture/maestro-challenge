#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
ANKAIOS_LOG_DIR="/var/log"
mkdir -p ${ANKAIOS_LOG_DIR}

# Start podman service
pgrep podman &>/dev/null
if [ $? -ne 0 ] 
then
    echo "Starting podman service"
    podman system service --time=0 unix:///tmp/podman.sock &
else 
    echo "The podman service is already running."
fi
sleep 2

# Start the Ankaios server
echo "Starting Ankaios server"
ank-server --startup-config ${SCRIPT_DIR}/../config/startupState.yaml --address 127.0.0.1:25551 > ${ANKAIOS_LOG_DIR}/ankaios-server.log 2>&1 &

sleep 2
# Start an Ankaios agent
echo "Starting Ankaios agent agent_A"
ank-agent --name agent_A -p /tmp/podman.sock --server-url http://127.0.0.1:25551 > ${ANKAIOS_LOG_DIR}/ankaios-agent_A.log 2>&1 &

# Wait for any process to exit
wait -n

# Exit with status of process that exited first
exit $?