#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod framebuffer_adapter;
mod writer;
mod spin_lock;
#[macro_use]
mod println;
mod interrupts;
mod keyboard;


static DISPLAY: SpinLock<Option<StaticFramebufferAdapter>> = SpinLock::new(None);

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use crate::framebuffer_adapter::StaticFramebufferAdapter;
use crate::spin_lock::SpinLock;

entry_point!(kernel_main);


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

        interrupts::init()
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("Panic! : {}", _info.message());
    loop {}
}
