use core::ptr::NonNull;
use crate::page_table::PageTable;
use crate::physical_memory::PhysicalMemoryAllocator;
use crate::virt::node::Node;
use crate::{MemoryResult, VirtAddr};

pub struct NodeAllocator {
    bitmap: [u8; 320],
    next_free: u64,
}

impl NodeAllocator {
    pub const VMEM_START: VirtAddr = 0xFFFF_FF00_0000_0000u64 as _;
    pub const VMEM_SIZE: usize = 21;

    pub fn new(pml4: &mut PageTable) -> MemoryResult<&'static mut Self> {
        let allocator = PhysicalMemoryAllocator::get()?;

        let node_allocator = allocator.allocate_page()?;
        pml4.map_page(Self::VMEM_START, node_allocator)?;

        for i in 1..=Self::VMEM_SIZE {
            let page = PhysicalMemoryAllocator::get()?.allocate_page()?;
            pml4.map_page(unsafe { Self::VMEM_START.add(i * 0x1000) }, page)?;
        }

        let node_allocator = unsafe {
            &mut *Self::VMEM_START.cast::<Self>()
        };

        node_allocator.bitmap.fill(0);
        node_allocator.next_free = 0;

        Ok(node_allocator)
    }

    pub fn allocate(&mut self) -> NonNull<Node> {
        todo!()
    }

    pub fn deallocate(&mut self, node: NonNull<Node>) {
        todo!()
    }
}