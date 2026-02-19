use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use pic8259::ChainedPics;
use crate::spin_lock::SpinLock;

pub const PIC_1_OFFSET: u8 = 32; // port d'origine de debut pour les interruptions master
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8; // port d'origine de debut pour les interruptions slave

// Programmable Interrupt Controller,
pub static PICS: SpinLock<ChainedPics> = SpinLock::new(
    // unsafe car on gère des donnees physiques (port CPU)
    unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) }
);


static IDT: SpinLock<Option<InterruptDescriptorTable>> = SpinLock::new(None);

pub fn init() {
    // permet l'activation des interruptions
    unsafe { PICS.lock().initialize() };

    let mut idt = InterruptDescriptorTable::new();
    // 33 = interruption IRQ1 = clavier
    idt[33].set_handler_fn(keyboard_interrupt_handler);

    *IDT.lock() = Some(idt);
    IDT.lock().as_ref().unwrap().load();
}

// Handler appelé à chaque pression de touche
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    // Lire le scancode depuis le port 0x60
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    crate::keyboard::process_scancode(scancode);

    unsafe {
        PICS.lock().notify_end_of_interrupt(PIC_1_OFFSET + 1);
    }
}