use common::BootInfo;
use crate::MemoryResult;

pub type PhysAddr = u64;

static mut PHYSICAL_ALLOCATOR: Option<PhysicalMemoryAllocator> = None;

pub(crate) struct PhysicalMemoryAllocator {
    memory_bitmap: &'static mut [u8],
    page_ptr: usize,

    total_pages: usize,
    pages_in_use: usize,
}

impl PhysicalMemoryAllocator {
    pub fn init(boot_info: &BootInfo) {
        let memory_bitmap = unsafe { core::slice::from_raw_parts_mut(boot_info.memory_bitmap_ptr, boot_info.memory_bitmap_size) };

        unsafe {
            PHYSICAL_ALLOCATOR = Some(Self {
                memory_bitmap,
                page_ptr: 0,

                total_pages: boot_info.memory_size,
                pages_in_use: boot_info.memory_used,
            });
        }
    }

    pub fn get() -> MemoryResult<&'static mut PhysicalMemoryAllocator> {
        todo!()
    }

    /// This method will panic if `PhysicalMemoryAllocator::init` has not yet been called
    pub unsafe fn get_unchecked() -> &'static mut PhysicalMemoryAllocator {
        todo!()
    }

    pub fn used_page_count(&self) -> usize {
        todo!()
    }

    pub fn free_page_count(&self) -> usize {
        todo!()
    }

    pub fn allocate_page(&mut self) -> MemoryResult<PhysAddr> {
        todo!()
    }

    pub fn deallocate_page(&mut self, addr: PhysAddr) -> MemoryResult<()> {
        todo!()
    }
}