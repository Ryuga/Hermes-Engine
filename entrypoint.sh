#!/bin/bash
# Ensure the run directory exists
mkdir -p /run/isolate

# Start Isolate's cgroup keeper in the background
isolate-cg-keeper &

# Give it a second to initialize and write the cgroup file
sleep 2

# Execute the main application passed via CMD
exec "$@"