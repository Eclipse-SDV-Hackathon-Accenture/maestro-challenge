#!/bin/bash

# Cleanup Ankaios ....
echo "Cleaning up Ankaios..."
pkill ank-agent
pkill ank-server
echo "OK."

# Cleanup podman
echo "Cleaning up podman..."
podman stop -a
podman rm -a
echo "OK."