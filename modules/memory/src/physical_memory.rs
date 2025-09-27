use crate::MemoryResult;

pub type PhysAddr = u64;

static mut PHYSICAL_ALLOCATOR: Option<PhysicalMemoryAllocator> = None;

struct PhysicalMemoryAllocator {
    memory_bitmap: &'static mut [u8],
    page_ptr: usize,
    
    total_pages: usize,
    pages_in_use: usize,
}

impl PhysicalMemoryAllocator {
    // TODO: build memory bitmap in BootInfo and use it here
    // TODO: move BootInfo into common module and have it passed into this function
    pub fn init() {
        todo!()
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