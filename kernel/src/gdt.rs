use crate::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

/// cette partie est encore un peu flou
/// f

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static!(static ref TSS: TaskStateSegment = {
    let mut tss = TaskStateSegment::new();

    // Allouer une stack de 20 KB (5 pages de 4KB) TODO : pourquoi
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE]; // sert a aloué l'espace memoire dynamiquement

        let stack_start = VirtAddr::from_ptr(unsafe { &raw const STACK });
        let stack_end = stack_start + STACK_SIZE as u64;
        stack_end
    };

    tss
};);

struct Selectors {
    code_selector: SegmentSelector,
    data_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

lazy_static!(static ref GDT: (GlobalDescriptorTable, Selectors) = {
    let mut gdt = GlobalDescriptorTable::new();

    // Segment de code kernel
    let code_selector = gdt.append(Descriptor::kernel_code_segment());

    // Segment de data
    let data_selector = gdt.append(Descriptor::kernel_data_segment());

    // Segment TSS
    let tss_selector = gdt.append(Descriptor::tss_segment(&TSS));

    (gdt, Selectors { code_selector, data_selector, tss_selector })
};);

pub fn init() {
    use x86_64::instructions::segmentation::{CS, Segment, DS, ES, SS};
    use x86_64::instructions::tables::load_tss;

    crate::dbg_println!("Chargement GDT...");
    GDT.0.load();
    crate::dbg_println!("GDT chargée"); // TODO gérer l'UTF8
    unsafe {
        crate::dbg_println!("Configuration segments...");
        CS::set_reg(GDT.1.code_selector);
        crate::dbg_println!("  CS = {:?}", GDT.1.code_selector);

        DS::set_reg(GDT.1.data_selector);
        ES::set_reg(GDT.1.data_selector);
        SS::set_reg(GDT.1.data_selector);
        crate::dbg_println!("  DS/ES/SS = {:?}", GDT.1.data_selector);

        load_tss(GDT.1.tss_selector);
        crate::dbg_println!("  TSS = {:?}", GDT.1.tss_selector);
    }

    crate::dbg_println!("GDT init terminée");
}