use core::ptr::NonNull;
use crate::page_table::PageTable;
use crate::physical_memory::PhysicalMemoryAllocator;
use crate::virt::node::Node;
use crate::{MemoryResult, VirtAddr};

pub struct NodeAllocator {
    bitmap: [u8; 320],
    next_free: usize,
}

impl NodeAllocator {
    pub const VMEM_START: VirtAddr = 0xFFFF_FF00_0000_0000u64 as _;
    pub const NODE_MEM_START: VirtAddr = 0xFFFF_FF00_0000_1000u64 as _;
    pub const VMEM_SIZE: usize = 21;

    const MAX_NODES: usize = (0x1000 * 20) / size_of::<Node>();

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

    fn set_used(&mut self, idx: usize, used: bool) {
        let offset = idx % 8;
        let idx = idx / 8;

        if used {
            self.bitmap[idx] |= 1 << offset;
        } else {
            self.bitmap[idx] &= !(1 << offset);
        }
    }

    fn is_used(&self, idx: usize) -> bool {
        let offset = idx % 8;
        let idx = idx / 8;

        self.bitmap[idx] & (1 << offset) != 0
    }

    pub fn allocate(&mut self) -> Option<NonNull<Node>> {
        let mut idx = None;

        for i in self.next_free..Self::MAX_NODES {
            if !self.is_used(i) {
                idx = Some(i);
                break;
            }
        }

        if idx.is_none() {
            for i in 0..self.next_free {
                if !self.is_used(i) {
                    idx = Some(i);
                    break;
                }
            }
        }

        if let Some(idx) = idx {
            self.next_free = if idx + 1 >= Self::MAX_NODES { 0 } else { idx + 1 };

            self.set_used(idx, true);
            Some(NonNull::new(unsafe { Self::NODE_MEM_START.cast::<Node>().offset(idx as _) }).expect("memory allocation failed"))
        } else {
            None
        }
    }

    pub fn deallocate(&mut self, node: NonNull<Node>) {
        let idx = (node.as_ptr() as usize - Self::NODE_MEM_START as usize) / size_of::<Node>();

        self.set_used(idx, false);
    }
}