#!/bin/bash

if [ -z "$1" ]; then
    export BUILD_TYPE="debug"
else
    export BUILD_TYPE="$1"
fi

cargo kbuild_$BUILD_TYPE && cargo genimage_$BUILD_TYPE && qemu-system-x86_64 -drive format=raw,file=./bin/$BUILD_TYPE/boot-uefi-kust.img -bios ./bin/OVMF-pure-efi.fd
