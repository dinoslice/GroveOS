use crate::{MemoryResult, VirtAddr};
use crate::physical_memory::PhysAddr;

const RECURSIVE_ENTRY: usize = 510;

const PT_LEVEL_PML4: u8 = 3;
const PT_LEVEL_PDPT: u8 = 2;
const PT_LEVEL_PD: u8 = 1;
const PT_LEVEL_PT: u8 = 0;

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

    /// This function assumes that the requested page table is located in the overall page table structure
    unsafe fn get_page_table(level: u8, indices: &[usize]) -> PageTable {
        let mut addr = 0;
        let mut shift: u8 = 39;

        for _ in 0..level {
            addr |= RECURSIVE_ENTRY << shift;
            shift -= 9;
        }

        for index in indices {
            addr |= index << shift;
            shift -= 9;
        }

        unsafe {
            Self(core::slice::from_raw_parts_mut(addr as *mut u64, 512))
        }
    }

    pub fn map_page(&mut self, virt: VirtAddr, phys: PhysAddr) -> MemoryResult<()> {
        todo!()
    }

    pub fn unmap_page(&mut self, virt: VirtAddr) -> MemoryResult<()> {
        todo!()
    }
}