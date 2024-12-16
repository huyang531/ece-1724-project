#!/bin/bash

# List of ports to check and kill processes for
PORTS=(
3000
3011
)

for PORT in "${PORTS[@]}"; do
    # echo "Checking for processes listening on port $PORT..."
    # Find the process ID (PID) listening on the port
    PID=$(lsof -ti tcp:$PORT)

    if [ -n "$PID" ]; then
        # echo "Found process with PID $PID on port $PORT. Killing it..."
        kill -SIGINT $PID
        echo "Process $PID on port $PORT killed."
    else
        echo "No process found on port $PORT."
    fi
done
