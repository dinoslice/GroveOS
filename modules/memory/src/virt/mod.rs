use core::ptr::NonNull;
use crate::{MemoryResult, VirtAddr};
use crate::page_table::PageTable;
use crate::physical_memory::PhysAddr;
use crate::virt::node::Node;
use crate::virt::node_allocator::NodeAllocator;

mod node;
mod node_allocator;

pub struct PageAllocator {
    node_allocator: &'static NodeAllocator,
    root_node: NonNull<Node>,
    pml4: PageTable,
}

impl PageAllocator {
    pub fn new() -> Self {
        todo!()
    }

    pub fn kernel() -> Self {
        todo!()
    }

    pub fn install() -> Self {
        todo!()
    }

    pub fn allocate_page(&mut self) -> MemoryResult<VirtAddr> {
        self.allocate_pages(1)
    }

    pub fn allocate_pages(&mut self, pages: usize) -> MemoryResult<VirtAddr> {
        todo!()
    }

    pub fn allocate_page_at(&mut self, addr: VirtAddr) -> MemoryResult<VirtAddr> {
        self.allocate_pages_at(addr, 1)
    }

    pub fn allocate_pages_at(&mut self, addr: VirtAddr, pages: usize) -> MemoryResult<VirtAddr> {
        todo!()
    }

    pub fn deallocate_page(&mut self, addr: VirtAddr) -> MemoryResult<()> {
        self.deallocate_pages(addr, 1)
    }

    pub fn deallocate_pages(&mut self, addr: VirtAddr, pages: usize) -> MemoryResult<()> {
        todo!()
    }

    pub fn map_page(&mut self, addr: PhysAddr) -> MemoryResult<VirtAddr> {
        todo!()
    }

    pub unsafe fn unmap_page(&mut self, addr: VirtAddr) -> MemoryResult<()> {
        todo!()
    }

    pub fn edit_flags(&mut self, addr: VirtAddr, new_flags: u64) -> MemoryResult<()> {
        todo!()
    }
}