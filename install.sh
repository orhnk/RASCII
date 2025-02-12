#!/bin/sh

echo "Sudo is needed to copy the binary to /usr/local/bin - prompting now."
sudo true

if [ -d "RASCII" ]; then
    echo "Error: RASCII directory already exists"
    exit 1
fi

if ! git clone https://github.com/orhnk/RASCII; then
    echo "Error: Failed to clone RASCII repository"
    exit 1
fi

cd RASCII

cargo build --release

sudo cp ./target/release/rascii /usr/local/bin

echo "RASCII has been successfully installed on your system. You can run

    rascii --help

to learn how to use RASCII."
