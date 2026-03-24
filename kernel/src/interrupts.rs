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

    idt[32].set_handler_fn(timer_interrupt_handler);
    idt[33].set_handler_fn(keyboard_interrupt_handler);

    // Exceptions courantes
    idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
    idt.page_fault.set_handler_fn(page_fault_handler);
    idt.invalid_tss.set_handler_fn(invalid_tss_handler);
    idt.segment_not_present.set_handler_fn(segment_not_present_handler);

    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX);
    }

    idt
};);

pub fn init() {
    // permet l'activation des interruptions
    unsafe { PICS.lock().initialize() };
    IDT.load();
    x86_64::instructions::interrupts::enable();
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

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Ne rien faire, juste acquitter l'interruption, necessaire car IDT[32] a une configuration par
    // défaut invalide
    unsafe {
        PICS.lock().notify_end_of_interrupt(32);  // IRQ 0 → INT 32
    }
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

// Handlers d'exceptions
extern "x86-interrupt" fn general_protection_fault_handler (
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("EXCEPTION: GENERAL PROTECTION FAULT (code: {})\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: x86_64::structures::idt::PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    panic!("EXCEPTION: PAGE FAULT\nAccessed Address: {:?}\nError Code: {:?}\n{:#?}",
           Cr2::read(), error_code, stack_frame);
}

extern "x86-interrupt" fn invalid_tss_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("EXCEPTION: INVALID TSS (code: {})\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn segment_not_present_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("EXCEPTION: SEGMENT NOT PRESENT (code: {})\n{:#?}", error_code, stack_frame);
}