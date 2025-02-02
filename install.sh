#!/bin/sh

echo "Sudo is needed to copy the binary to /usr/local/bin - prompting now."
sudo true

git clone https://github.com/orhnk/RASCII
cd RASCII

cargo build --release

sudo cp ./target/release/rascii /usr/local/bin

echo "RASCII has been successfully installed on your system. You can run

    rascii --help

to learn how to use RASCII."
