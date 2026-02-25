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
mod lazy_static;


use bootloader_api::{entry_point, BootInfo};
use crate::spin_lock::SpinLock;
use crate::writer::Terminal;

static DISPLAY: SpinLock<Option<Terminal>> = SpinLock::new(None);

entry_point!(kernel_main);


fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    let fb = _boot_info.framebuffer.as_mut().unwrap();
    let info = fb.info();
    let buffer = fb.buffer_mut();

    // on nettoie l'affichage du bootloader
    buffer.fill(0);


    *DISPLAY.lock() = Some(Terminal::new(buffer.as_mut_ptr(), info));
    println!("zozoL!");
    interrupts::init();
    println!("zaza!");

    println!("Welcome to KERNEL!");
    print!("SOSO");
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Panic! : {}", _info.message());
    loop {}
}
