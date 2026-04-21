#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]
extern crate alloc;

mod framebuffer_adapter;
mod terminal;
mod spin_lock;
#[macro_use]
mod println;
mod interrupts;
mod keyboard;
mod gdt;
mod window;
pub mod utils;
pub mod graphie;
mod allocator;
mod memory;
pub mod dbg_print;

use alloc::boxed::Box;
use core::any::TypeId;
use bootloader_api::{entry_point, BootInfo, BootloaderConfig};
use bootloader_api::config::Mapping;
use crate::memory::{init_memory};
use crate::terminal::Terminal;
use crate::window::{render_updated_window, Window};


static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    // 1. Affichage
    let fb = _boot_info.framebuffer.as_mut().unwrap();
    let info = fb.info();
    let buffer = fb.buffer_mut();
    buffer.fill(0);

    // 2. Mémoire
    let pmo = _boot_info.physical_memory_offset.take().unwrap();
    let memory_regions = &_boot_info.memory_regions;
    init_memory(pmo, memory_regions).expect("Impossible d'afficher ce text display lancé apres");
    graphie::init_display(buffer.as_mut_ptr(), info);

    // On envoie le terminal vers le tas, pour pouvoir l'intégrer au register des fenêtres
    //

    Box::new(Terminal::new()).register();

    dbg_println!("Memory OK");
    dbg_println!("---");

    // 3. GDT et IDT
    gdt::init();
    dbg_println!("GDT OK");
    interrupts::init();
    dbg_println!("IDT OK");



    let heap_value = Box::new(41);
    dbg_println!("heap_value at {:p}", heap_value);
    render_updated_window();

    loop {
        x86_64::instructions::hlt();
        // mise a jour render sans interruption CPU (évite le conflit des lock infini de mon spin lock)
        // peut-être pas une bonne idee cette logique à voir si c'est bloquand plus tard
        x86_64::instructions::interrupts::without_interrupts(|| {
            render_updated_window();
        });
    }
}
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Panic! : {}", _info.message());
    dbg_println!("Panic! : {}", _info.message());
    loop {}
}
