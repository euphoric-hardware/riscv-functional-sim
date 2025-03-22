#!/bin/bash

# Check if a path argument is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <path>"
    exit 1
fi

# Extract the final element of the provided path
FINAL_ELEMENT=$(basename "$1")
SPIKE_LOG="spike_${FINAL_ELEMENT}.log"
SIM_LOG="sim_${FINAL_ELEMENT}.log"

# Run the spike command only if the log file does not exist
if [ ! -f "$SPIKE_LOG" ]; then
    spike --log-commits "$1" 2> "$SPIKE_LOG"
else
    echo "Skipping spike command: $SPIKE_LOG already exists."
fi

# Run the cargo command
cargo run -- --bin "$1" --output-log "$SIM_LOG" --spike-log "$SPIKE_LOG"
