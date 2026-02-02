#![no_std]
#![no_main]

mod framebuffer_adapter;
mod console;
mod writer;

use framebuffer_adapter::FramebufferAdapter;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    text::Text,
};
use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use embedded_graphics::mono_font::ascii::FONT_9X18;
use embedded_graphics::mono_font::MonoTextStyleBuilder;

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
    let style = MonoTextStyleBuilder::new()
        .font(&FONT_9X18)
        .text_color(Rgb888::new(0, 255, 0))  // Vert phosphore
        .background_color(Rgb888::BLACK)
        .build();

    // Create a text at position (20, 30) and draw it using the previously defined style
    Text::new("Hello ma meuringue !", Point::new(20, 30), style)
        .draw(&mut display).expect("Drawing failed");
    Text::new("T'es la plus belle", Point::new(20, 50), style)
        .draw(&mut display).expect("Drawing failed");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
