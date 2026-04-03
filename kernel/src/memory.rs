use bootloader_api::info::{MemoryRegions, MemoryRegionKind};
use x86_64::{PhysAddr, VirtAddr};
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB};
use x86_64::structures::paging::mapper::MapToError;
use crate::allocator::init_heap;

pub(crate) struct BootInfoFrameAllocator {
    memory_map: &'static MemoryRegions,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryRegions) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();

        // On liste la plage de memoire Usable
        let usable_regions = regions
            .filter(|r| r.kind == MemoryRegionKind::Usable);

        // Conceptuellement on a une liste de plage memoire (tableau 2D)
        let addr_ranges = usable_regions
            .map(|r| r.start..r.end);

        // et paf on passe en liste d'adresse, 4096 pour le Size4KiB
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator{
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}


pub fn init_memory(physical_memory_offset: u64, memory_regions: &'static MemoryRegions) -> Result<(), MapToError<Size4KiB>>{
    let phys_offset = VirtAddr::new(physical_memory_offset);
    let (level_4_frame, _) = Cr3::read();
    let phys_addr = level_4_frame.start_address();
    let virt_addr = phys_offset + phys_addr.as_u64();
    let level_4_table: &mut PageTable = unsafe {
        &mut *(virt_addr.as_mut_ptr())
    };
    let mapper = unsafe { &mut OffsetPageTable::new(level_4_table, phys_offset) };
    let frame_allocator = unsafe {
        &mut BootInfoFrameAllocator::init(memory_regions)
    };

    init_heap(mapper, frame_allocator)
}