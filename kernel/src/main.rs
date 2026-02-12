#![no_std]
#![no_main]

use core::fmt::Write;
mod framebuffer_adapter;
mod writer;
mod spin_lock;
mod println;

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use crate::framebuffer_adapter::StaticFramebufferAdapter;
use crate::spin_lock::SpinLock;

entry_point!(kernel_main);

static DISPLAY: SpinLock<Option<StaticFramebufferAdapter>> = SpinLock::new(None);


fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    let fb = _boot_info.framebuffer.as_mut().unwrap();
    let info = fb.info();
    let buffer = fb.buffer_mut();

    // on nettoie l'affichage du bootloader
    buffer.fill(0);

    unsafe {
        let display = StaticFramebufferAdapter::new(
            buffer.as_mut_ptr(),
            info
        );

        *DISPLAY.lock() = Some(display);
    }


    println!("Hello World!\nles {} polette", 5);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("Panic! : {}", _info.message());
    loop {}
}
