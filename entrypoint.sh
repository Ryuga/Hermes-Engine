#!/bin/bash

# Why are we doing this weird split-folder architecture instead of using the official daemon?
# The official 'isolate-cg-keeper' assumes you're on a bare-metal server using systemd.
#
# cgroups v2 has a strict rule that a folder cannot contain active processes and delegate
# memory tracking to its child folders. If I leave our main app running in the root,
# the Linux kernel throws a "Device or resource busy" fit when Isolate tries to track memory.
#
# So, I wrote this script that does the following.
# Moves Hermes and Docker's init scripts into their own '/init' folder to completely empty out the root.
# Once the root is empty, the kernel is happy, and it lets us delegate memory tracking down into '/isolate_sandboxes'.

# Mount a pristine cgroups v2 filesystem
mount -t cgroup2 none /sys/fs/cgroup

# Create the split architecture
mkdir -p /sys/fs/cgroup/init
mkdir -p /sys/fs/cgroup/isolate_sandboxes

# Move Hermes and Docker scripts into 'init' to empty the root
for pid in $(cat /sys/fs/cgroup/cgroup.procs); do
    echo "$pid" > /sys/fs/cgroup/init/cgroup.procs 2>/dev/null || true
done

# Turn on trackers at the newly empty root
if [ -f /sys/fs/cgroup/cgroup.subtree_control ]; then
    echo "+cpuset +cpu +memory +pids" > /sys/fs/cgroup/cgroup.subtree_control 2>/dev/null || true
fi

# Turn on trackers at isolate_sandboxes
if [ -f /sys/fs/cgroup/isolate_sandboxes/cgroup.subtree_control ]; then
    echo "+cpuset +cpu +memory +pids" > /sys/fs/cgroup/isolate_sandboxes/cgroup.subtree_control 2>/dev/null || true
fi

# Ensure the lock directory exists
mkdir -p /run/isolate

# Execute Hermes
exec "$@"
