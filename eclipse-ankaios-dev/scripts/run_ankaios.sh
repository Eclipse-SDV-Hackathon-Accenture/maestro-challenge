#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

echo '[Starting Ankaios cluster ...]'
$SCRIPT_DIR/run_ankaios_default.sh &

sleep 3;
while [ `ank -s http://127.0.0.1:25551 get state workloadStates |  yq '.workloadStates.[] | select(.workloadName == "hello") | .executionState' | grep -c ExecSucceeded` = 0 ]; do 
  sleep 3; 
done

echo '[Ankaios cluster started.]'

cat $SCRIPT_DIR/../config/startupState.yaml | yq '.workloads.* | path | .[-1]' | while read p; 
do
  yq ".workloads.$p | {\"currentState\": {\"workloads\": {\"$p\": .}}}"< $SCRIPT_DIR/../config/startupState.yaml > /tmp/currentState.yaml
  echo "Starting workload '$p' ..."
  ank -s http://127.0.0.1:25551 set state -f /tmp/currentState.yaml currentState.workloads.$p
  while true; do
    if [ $(ank -s http://127.0.0.1:25551 get state workloadStates |  yq ".workloadStates.[] | select(.workloadName == \"$p\") | .executionState" | grep -Ec '(ExecRunning|ExecFailed)') = 1 ]
    then
        break
    fi
    sleep 3; 
  done
  echo "Workload '$p' started."
done 

rm /tmp/currentState.yaml

