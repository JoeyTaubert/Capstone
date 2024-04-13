#!/bin/bash

# TESTING PURPOSES ONLY (use of sudo)
#
# This script compiles the binary and adds permissions for the binary
#     to run a pcap.
#
# This script should be run from the main Rust project directory
#     (alongside src folder).
#

# Get the parent directory
pdir=$(basename "$(pwd)")

bpath="./target/release/$pdir"

# Compile the binary
cargo build --release

# If the build command was successful, 
if [ $? -eq 0 ]; then
    echo "Build successful. Setting capabilities..."

    # Requires sudo
    sudo setcap 'cap_net_raw,cap_net_admin=eip' $bpath

    # If the capabilities were set correctly, execute the binary
    if [ $? -eq 0 ]; then
        echo "Running the binary with added capabilities..."
        $bpath
    fi
else
    echo "Cargo build failed"
fi
