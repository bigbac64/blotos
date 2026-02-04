#![no_std]
#![no_main]

use core::fmt::Write;
mod framebuffer_adapter;
mod writer;

use framebuffer_adapter::FramebufferAdapter;
use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use writer::Terminal;

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

    let mut terminal = Terminal::new(&mut display);
    let _ = writeln!(&mut terminal, "hello !\nc'est pas possible");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
