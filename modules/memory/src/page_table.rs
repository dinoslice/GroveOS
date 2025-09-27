use crate::{MemoryResult, VirtAddr};
use crate::physical_memory::PhysAddr;

pub struct PageTable(&'static mut [u64]);

impl PageTable {
    pub fn new() -> MemoryResult<Self> {
        todo!()
    }

    pub fn install(&self) {
        todo!()
    }

    pub fn map_page(&mut self, virt: VirtAddr, phys: PhysAddr) -> MemoryResult<()> {
        todo!()
    }

    pub fn unmap_page(&mut self, virt: VirtAddr) -> MemoryResult<()> {
        todo!()
    }
}