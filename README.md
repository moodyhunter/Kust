# Kust

It's me learning the Rust Language, following [blog_os](https://github.com/phil-opp/blog_os).

## OVMF Package (UEFI-based)

To run the UEFI version of this kernel, you should have a basic UEFI environment for qemu:
- Download [prebuilt OVMF package](https://github.com/rust-osdev/ovmf-prebuilt) and place `OVMF-pure-efi.fd` in [`bin`](./bin)

## Steps to run (UEFI):

- `./run-vm.sh` will resolve everything, provided that you have the OVMF package placed correctly.

## Steps to run (BIOS):

1. `cargo kbuild` to build the kernel.
2. `cargo gendisk` to generate a boot-able uefi image
3. Hit `qemu-system-x86_64 -drive format=raw,file=./target/x86_64-bootloader/release/boot-bios-kust.img` to enjoy a ~~framebuffer show~~!


# License

MIT
