#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    let fb = _boot_info.framebuffer.as_mut().unwrap();
    let info = fb.info();
    let buffer = fb.buffer_mut();

    buffer[0] = 0x00; // B
    buffer[1] = 0x00; // G
    buffer[2] = 0xff; // R

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
