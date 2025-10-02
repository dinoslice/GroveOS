use core::ptr::NonNull;
use crate::page_table::PageTable;
use crate::virt::node::Node;

pub struct NodeAllocator {
    bitmap: [u8; 128],
    allocated_groups: u8,
}

impl NodeAllocator {
    pub fn new(pml4: &mut PageTable) -> Self {
        todo!()
    }

    pub fn allocate(&mut self) -> NonNull<Node> {
        todo!()
    }

    pub fn deallocate(&mut self, node: NonNull<Node>) {
        todo!()
    }
}