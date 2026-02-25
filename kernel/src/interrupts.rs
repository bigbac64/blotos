use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use pic8259::ChainedPics;
use crate::lazy_static;
use crate::spin_lock::SpinLock;

pub const PIC_1_OFFSET: u8 = 32; // port d'origine de debut pour les interruptions master
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8; // port d'origine de debut pour les interruptions slave

// Programmable Interrupt Controller,
pub static PICS: SpinLock<ChainedPics> = SpinLock::new(
    // unsafe car on gère des donnees physiques (port CPU)
    unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) }
);


// placement en static obligatoire pour que l'idt soit initialiser pendant que le programme est up
lazy_static!( static ref IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();
    idt[33].set_handler_fn(keyboard_interrupt_handler);
    idt
};);

pub fn init() {
    // permet l'activation des interruptions
    unsafe { PICS.lock().initialize() };
    IDT.load()
}

// Handler appelé à chaque pression de touche
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    crate::println!("istterup");
    
    // Lire le scancode depuis le port 0x60
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    crate::keyboard::process_scancode(scancode);

    unsafe {
        PICS.lock().notify_end_of_interrupt(PIC_1_OFFSET + 1);
    }
}