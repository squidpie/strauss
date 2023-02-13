#!/bin/bash
# entrypoint.val.sh
# Copyright (C) 2023
# Squidpie

# turn on bash's job control
set -m

# Start the dut process and put it in the background
${1} &

# Start Debug process
sudo tcpdump -w startup.pcap

# number must match order you called ${1}
fg %1