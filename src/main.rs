#![no_main]
#![no_std]
#![feature(panic_info_message)]

use bootloader::{entry_point, BootInfo};
use conquer_once::spin::OnceCell;
use core::panic::PanicInfo;
use printk::LockedPrintk;

entry_point!(kernel_main);

pub static PRINTK: OnceCell<LockedPrintk> = OnceCell::uninit();

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("");
    log::error!("========================= KERNEL PANIC =========================");

    if let Some(panic_message) = info.message() {
        log::error!("  Kernel Panic: '{}'", panic_message);
    } else {
        log::error!("  No Panic Message Available.");
    }

    if let Some(panic_location) = info.location() {
        log::error!(
            "  At file: {}, Line {} Column {}",
            panic_location.file(),
            panic_location.line(),
            panic_location.column()
        );
    } else {
        log::error!("  No Panic Location Available");
    }

    log::error!("====================== ENDOF KERNEL PANIC ======================");
    loop {}
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let fb_info = framebuffer.info();

        for b in [0x00, 0x55, 0xCC, 0xFF] {
            for byte in framebuffer.buffer_mut() {
                *byte = b;
            }
        }

        let kernel_logger = PRINTK
            .get_or_init(move || printk::LockedPrintk::new(framebuffer.buffer_mut(), fb_info));
        log::set_logger(kernel_logger).expect("logger already set");
        log::set_max_level(log::LevelFilter::Trace);

        log::trace!("Trace");
        log::debug!("Debug");
        log::info!("Info");
        log::warn!("Warn");
        log::error!("Error");
        panic!("Have got nothing to do.");
    }
    loop {}
}
