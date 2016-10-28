#!/bin/sh
./mount.sh
sudo cp target/x86_64-unknown-kernel/debug/kernel /mnt2/boot
./umount.sh