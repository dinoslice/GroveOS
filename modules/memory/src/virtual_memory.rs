use crate::MemoryResult;
use crate::page_table::PageTable;

pub type VirtAddr = *mut ();

pub struct PageAllocator {
    pml4: PageTable,
}

impl PageAllocator {
    pub fn new() -> MemoryResult<Self> {
        todo!()
    }

    pub fn allocate_page(&mut self) -> MemoryResult<VirtAddr> {
        todo!()
    }

    pub fn allocate_pages(&mut self, count: usize) -> MemoryResult<VirtAddr> {
        todo!()
    }

    pub fn allocate_page_at(&mut self, address: VirtAddr) -> MemoryResult<VirtAddr> {
        todo!()
    }

    pub fn allocate_pages_at(&mut self, address: VirtAddr, count: usize) -> MemoryResult<VirtAddr> {
        todo!()
    }

    pub fn deallocate_page(&mut self, address: VirtAddr) -> MemoryResult<()> {
        todo!()
    }

    pub fn deallocate_pages(&mut self, address: VirtAddr, count: usize) -> MemoryResult<()> {
        todo!()
    }
}