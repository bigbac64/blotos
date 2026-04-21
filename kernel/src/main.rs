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
use embedded_graphics::Drawable;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{Point, Primitive, RgbColor, Size};
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;
use crate::graphie::DISPLAY_;
use crate::memory::{init_memory};
use crate::terminal::Terminal;
use crate::window::Window;


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


    Rectangle::new(Point::new(1, 1), Size::new(30, 30))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::WHITE, 3))
        .draw(&mut DISPLAY_.lock().as_mut().unwrap().0.as_framebuffer_adapter())
        .unwrap();

    {
        Box::new(Terminal::new()).register();
        let mut registry = crate::window::WINDOW_REGISTRY.lock();
        if let Some(window) = registry.get_mut(&core::any::TypeId::of::<Terminal>(), 0) {
            window.draw()
        }
    }


    Rectangle::new(Point::new(60, 1), Size::new(30, 30))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::WHITE, 3))
        .draw(&mut DISPLAY_.lock().as_mut().unwrap().0.as_framebuffer_adapter())
        .unwrap();

    println!("Memory OK");

    // 3. GDT et IDT
    gdt::init();
    interrupts::init();
    println!("GDT/IDT OK");



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
