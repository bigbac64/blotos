#![no_std]
#![no_main]

mod framebuffer_adapter;

use framebuffer_adapter::FramebufferAdapter;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    text::Text,
};
use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    let fb = _boot_info.framebuffer.as_mut().unwrap();
    let info = fb.info();
    let buffer = fb.buffer_mut();

    // on nettoie l'affichage du bootloader
    buffer.fill(0);

    let mut display = FramebufferAdapter::new(
        buffer,
        info
    );

    // Create a new character style
    let style = MonoTextStyle::new(&FONT_6X10, Rgb888::WHITE);

    // Create a text at position (20, 30) and draw it using the previously defined style
    Text::new("Hello Rust!", Point::new(20, 30), style).draw(&mut display).expect("Drawing failed");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
