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

use alloc::boxed::Box;
use bootloader_api::{entry_point, BootInfo, BootloaderConfig};
use bootloader_api::config::Mapping;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{OffsetPageTable, PageTable};
use x86_64::VirtAddr;
use crate::allocator::init_heap;
use crate::memory::{init_memory, BootInfoFrameAllocator};
use crate::spin_lock::SpinLock;
use crate::terminal::Terminal;

static DISPLAY: SpinLock<Option<Terminal>> = SpinLock::new(None);


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
    *DISPLAY.lock() = Some(Terminal::new(buffer.as_mut_ptr(), info));

    // 2. GDT et IDT
    gdt::init();
    interrupts::init();
    println!("GDT/IDT OK");

    // 3. Mémoire ensuite
    let pmo = _boot_info.physical_memory_offset.take().unwrap();
    let memory_regions = &_boot_info.memory_regions;
    init_memory(pmo, memory_regions).expect("Memory init failed");
    println!("Memory OK");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    loop { x86_64::instructions::hlt(); }
}
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Panic! : {}", _info.message());
    loop {}
}
