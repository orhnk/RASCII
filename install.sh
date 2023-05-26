#!/bin/sh

echo "Sudo is needed to copy the binary to /usr/bin - prompting now."
sudo true

cargo build --release

sudo cp ./target/release/rascii /usr/bin

echo "RASCII has been successfully installed on your system. You can run

    rascii --help

to learn how to use RASCII."
