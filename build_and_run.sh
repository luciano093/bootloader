#!/bin/bash

set -e

echo "Building kernel..."
cd kernel
cargo objcopy --release -- -O binary kernel.bin
cd ..

echo "Assembling bootloader..."
nasm -f bin boot.asm -o boot.bin

echo "Creating disk image..."
cat boot.bin kernel/kernel.bin > disk.img 

echo "Running QEMU..."
qemu-system-i386 -drive format=raw,file=disk.img