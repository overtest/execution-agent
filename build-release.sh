#!/bin/bash

# Install `seccomp` dev and user packages
sudo apt-get install -y libseccomp-dev seccomp
# Build a Cargo project in release mode
cargo build --release
