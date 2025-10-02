use core::ptr::NonNull;
use crate::{MemoryResult, VirtAddr};
use crate::page_table::PageTable;
use crate::physical_memory::PhysAddr;
use crate::virt::node::{Node, NodeStatus, PageRange};
use crate::virt::node_allocator::NodeAllocator;

mod node;
mod node_allocator;

static mut KERNEL_ALLOCATOR: Option<PageAllocator> = None;

pub struct PageAllocator {
    node_allocator: &'static mut NodeAllocator,
    user_root_node: NonNull<Node>,
    kernel_root_node: NonNull<Node>,
    pml4: PageTable,
}

impl PageAllocator {
    pub fn new() -> Self {
        todo!()
    }

    pub fn kernel() -> Option<&'static mut Self> {
        #[allow(static_mut_refs)]
        unsafe {
            KERNEL_ALLOCATOR.as_mut()
        }
    }

    pub unsafe fn kernel_unchecked() -> &'static mut Self {
        #[allow(static_mut_refs)]
        unsafe {
            KERNEL_ALLOCATOR.as_mut().unwrap()
        }
    }

    pub fn install(&self) {
        self.pml4.install()
    }

    pub(crate) fn init() {
        unsafe {
            let node_allocator = NodeAllocator::new(&mut PageTable::current()).expect("failed to create kernel memory node allocator");
            let node_a = node_allocator.allocate().unwrap();
            node_a.write(Node::Leaf {
                page_range: PageRange::new(0xFF00_0000_0, 0xFF00_0001_4),
                status: NodeStatus::Reserved,
            });

            let node_b = node_allocator.allocate().unwrap();
            node_b.write(Node::Leaf {
                page_range: PageRange::new(0xFF00_0001_5, 0xFFFF_FFFF_F),
                status: NodeStatus::Used
            });

            let node_c = node_allocator.allocate().unwrap();
            node_c.write(Node::Branch {
                page_range: PageRange::new(0xFF00_0000_0, 0xFFFF_FFFF_F),
                a: node_a,
                b: node_b,
            });

            let node_d = node_allocator.allocate().unwrap();
            node_d.write(Node::Leaf {
                page_range: PageRange::new(0x8000_0000_0, 0xFEFF_FFFF_F),
                status: NodeStatus::Free,
            });

            let kernel_root_node = node_allocator.allocate().unwrap();
            kernel_root_node.write(Node::Branch {
                page_range: PageRange::new(0x8000_0000_0, 0xFFFF_FFFF_F),
                a: node_d,
                b: node_c,
            });

            let user_root_node = node_allocator.allocate().expect("failed to allocate root node");
            user_root_node.write(Node::Leaf {
                page_range: PageRange::new(0, 0x7FFF_FFFF_F),
                status: NodeStatus::Free,
            });

            KERNEL_ALLOCATOR = Some(Self {
                node_allocator,
                user_root_node,
                kernel_root_node,
                pml4: PageTable::current()
            });
        }
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