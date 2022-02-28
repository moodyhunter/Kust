# Kust

It's me learning the Rust Language, following [blog_os](https://github.com/phil-opp/blog_os).

Steps to get my kernel started:

1. `cargo kbuild` to build the kernel.
2. `cargo gendisk` to generate a boot-able uefi image
3. Download [prebuilt OVMF package](https://github.com/rust-osdev/ovmf-prebuilt) and place `OVMF-pure-efi.fd` in [`bin`](./bin)
4. Hit `./run-vm.sh` to empty a ~~framebuffer show~~!

# License

MIT
