use core::arch::asm;
use crate::{MemoryResult, PageAllocator, VirtAddr};
use crate::physical_memory::{PhysAddr, PhysicalMemoryAllocator};

const RECURSIVE_ENTRY: usize = 510;

const PT_LEVEL_PML4: u8 = 3;
const PT_LEVEL_PDPT: u8 = 2;
const PT_LEVEL_PD: u8 = 1;
const PT_LEVEL_PT: u8 = 0;

const PT_PAGE_PRESENT: u64 = 1;
const PT_PAGE_WRITE: u64 = 2;

pub struct PageTable(&'static mut [u64]);

impl PageTable {
    pub fn new() -> MemoryResult<Self> {
        let addr = PageAllocator::kernel()?.allocate_page()?.cast::<u64>();

        unsafe {
            addr.offset(RECURSIVE_ENTRY as isize).write(addr as u64);
        }

        Ok(Self(unsafe { core::slice::from_raw_parts_mut(addr, 512) }))
    }

    pub fn current() -> PageTable {
        unsafe {
            Self::get_page_table_unchecked(PT_LEVEL_PML4, &[])
        }
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
    unsafe fn get_page_table_unchecked(level: u8, indices: &[usize]) -> PageTable {
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

    fn get_page_table(&self, indices: &[usize]) -> Option<PageTable> {
        assert!(indices.len() <= 3);

        let mut table = PageTable(unsafe { core::slice::from_raw_parts_mut(self.0.as_ptr() as *mut _, 512) });
        let level = 3 - indices.len();

        for (i, index) in indices.iter().enumerate() {
            if table.0[*index] & PT_PAGE_PRESENT == 0 {
                return None;
            } else {
                unsafe {
                    table = Self::get_page_table_unchecked((level - i) as u8, &indices[..i]);
                }
            }
        }

        Some(table)
    }

    pub fn map_page(&mut self, virt: VirtAddr, phys: PhysAddr) -> MemoryResult<()> {
        todo!()
    }

    pub fn unmap_page(&mut self, virt: VirtAddr) -> MemoryResult<()> {
        todo!()
    }
}