#!/bin/bash
cargo kbuild && cargo gendisk && qemu-system-x86_64 -drive format=raw,file=./target/x86_64-kust/debug/boot-uefi-kust.img -bios ./bin/OVMF-pure-efi.fd
