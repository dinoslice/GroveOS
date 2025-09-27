use common::BootInfo;
use crate::{MemoryError, MemoryResult};

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

    #[inline(always)]
    pub fn get() -> MemoryResult<&'static mut PhysicalMemoryAllocator> {
        #[allow(static_mut_refs)]
        unsafe {
            if let Some(allocator) = PHYSICAL_ALLOCATOR.as_mut() {
                Ok(allocator)
            } else {
                Err(MemoryError::PhysicalAllocatorNotInitialized)
            }
        }
    }

    /// This method will panic if `PhysicalMemoryAllocator::init` has not yet been called
    #[inline(always)]
    pub unsafe fn get_unchecked() -> &'static mut PhysicalMemoryAllocator {
        #[allow(static_mut_refs)]
        unsafe {
            PHYSICAL_ALLOCATOR.as_mut().expect("Physical memory allocator not initialized")
        }
    }

    #[inline(always)]
    pub fn used_page_count(&self) -> usize {
        self.pages_in_use
    }

    #[inline(always)]
    pub fn free_page_count(&self) -> usize {
        self.total_pages - self.pages_in_use
    }

    pub fn allocate_page(&mut self) -> MemoryResult<PhysAddr> {
        todo!()
    }

    pub fn deallocate_page(&mut self, addr: PhysAddr) -> MemoryResult<()> {
        todo!()
    }
}