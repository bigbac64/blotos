#![no_std]
#![no_main]

use core::panic::PanicInfo;
// dev as nightly with `rustup override set nightly` can use stable or beta use core::panic::PanicInfo;


static HELLO: &[u8] = b"Hello, world!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x4;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
