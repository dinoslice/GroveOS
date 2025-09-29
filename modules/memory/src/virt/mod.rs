use core::ptr::NonNull;
use crate::page_table::PageTable;
use crate::virt::node::Node;
use crate::virt::node_allocator::NodeAllocator;

mod node;
mod node_allocator;

pub struct PageAllocator {
    node_allocator: &'static NodeAllocator,
    root_node: NonNull<Node>,
    pml4: PageTable,
}