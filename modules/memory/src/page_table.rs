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

    fn indices_of_addr(addr: VirtAddr) -> (usize, usize, usize, usize) {
        todo!()
    }

    fn get_table_or_create(&mut self, idx: usize) -> PageTable {
        todo!()
    }

    pub fn map_page(&mut self, virt: VirtAddr, phys: PhysAddr) -> MemoryResult<()> {
        todo!()
    }

    pub fn unmap_page(&mut self, virt: VirtAddr) -> MemoryResult<()> {
        todo!()
    }
}