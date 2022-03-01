#![no_main]
#![no_std]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let mut value = 0x55;
        let mut i: u8 = 0;
        loop {
            i += 1;
            for byte in framebuffer.buffer_mut() {
                *byte = value;
                value = value.wrapping_add(i);
            }
        }
    }

    loop {}
}
