use crate::physical_memory::{PhysAddr, PhysicalMemoryAllocator};
use crate::{MemoryError, MemoryResult, PageAllocator, VirtAddr};
use core::arch::asm;

const RECURSIVE_ENTRY: usize = 510;

const PT_LEVEL_PML4: usize = 3;
const PT_LEVEL_PDPT: usize = 2;
const PT_LEVEL_PD: usize = 1;
const PT_LEVEL_PT: usize = 0;

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
        unsafe {
            asm!("mov cr3, {}", in(reg) self.0.as_ptr());
        }
    }

    fn indices_of_addr(addr: VirtAddr) -> (usize, usize, usize, usize) {
        let vaddr = addr as usize;
        const fn index(vaddr: usize, level: usize) -> usize {
            (vaddr >> (12 + 9 * level)) & 0x1FF
        }

        (
            index(vaddr, PT_LEVEL_PML4),
            index(vaddr, PT_LEVEL_PDPT),
            index(vaddr, PT_LEVEL_PD),
            index(vaddr, PT_LEVEL_PT),
        )
    }

    fn get_table_or_create(&mut self, indices: &[usize]) -> MemoryResult<PageTable> {
        if let Some(table) = self.get_page_table(indices) {
            Ok(table)
        } else {
            let addr = PhysicalMemoryAllocator::get()?.allocate_page()?;
            let higher = self.get_page_table(&indices[..indices.len() - 1]).ok_or(MemoryError::PageNotAllocated)?;
            higher.0[*indices.last().expect("index empty")] = addr | PT_PAGE_PRESENT | PT_PAGE_WRITE;

            Ok(self.get_page_table(indices).expect("should be mapped"))
        }
    }

    /// This function assumes that the requested page table is located in the overall page table structure
    unsafe fn get_page_table_unchecked(level: usize, indices: &[usize]) -> PageTable {
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
                    table = Self::get_page_table_unchecked(level - i, &indices[..i]);
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