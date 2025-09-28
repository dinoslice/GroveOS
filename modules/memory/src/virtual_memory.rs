use crate::{MemoryError, MemoryResult};
use crate::page_table::PageTable;
use crate::physical_memory::PhysicalMemoryAllocator;

pub type VirtAddr = *mut ();

static mut KERNEL_ALLOCATOR: Option<PageAllocator> = None;

pub struct PageAllocator {
    pml4: PageTable,
    current_page: usize,
}

impl PageAllocator {
    pub fn new() -> MemoryResult<Self> {
        Ok(Self {
            pml4: PageTable::new()?,
            current_page: 1,
        })
    }

    pub fn kernel() -> MemoryResult<&'static mut PageAllocator> {
        #[allow(static_mut_refs)]
        unsafe {
            if let Some(allocator) = KERNEL_ALLOCATOR.as_mut() {
                Ok(allocator)
            } else {
                Err(MemoryError::PhysicalAllocatorNotInitialized)
            }
        }
    }

    /// This function panics if `memory::init_module` hasn't been called
    pub unsafe fn kernel_unchecked() -> &'static mut PageAllocator {
        Self::kernel().expect("memory module not initialized")
    }

    pub fn install(&self) {
        self.pml4.install();
    }

    pub(crate) fn init() {
        unsafe {
            KERNEL_ALLOCATOR = Some(Self {
                pml4: PageTable::current(),
                current_page: 1,
            })
        }
    }

    pub fn allocate_page(&mut self) -> MemoryResult<VirtAddr> {
        let addr = PhysicalMemoryAllocator::get()?.allocate_page()?;
        let vaddr = self.current_page * 0x1000;
        self.current_page += 1;

        self.pml4.map_page(vaddr as VirtAddr, addr)?;

        Ok(vaddr as VirtAddr)
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