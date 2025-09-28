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
        assert!(indices.len() <= 3);

        let mut table = PageTable(unsafe { core::slice::from_raw_parts_mut(self.0.as_ptr() as *mut _, 512) });

        for (i, index) in indices.iter().enumerate() {
            if table.0[*index] & PT_PAGE_PRESENT == 0 {
                let addr = PhysicalMemoryAllocator::get()?.allocate_page()?;

                table.0[*index] = addr | PT_PAGE_PRESENT | PT_PAGE_WRITE;

                table = unsafe { Self::get_page_table_unchecked(3 - i, &indices[..i]) };
                table.0.fill(0);

                continue;
            }

            unsafe { table = Self::get_page_table_unchecked(3 - i, &indices[..i]); }
        }

        Ok(table)
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

        if addr & (1 << 47) != 0 {
            addr |= 0xFFFF_0000_0000_0000;
        }

        unsafe {
            Self(core::slice::from_raw_parts_mut(addr as *mut u64, 512))
        }
    }

    pub fn map_page(&mut self, virt: VirtAddr, phys: PhysAddr) -> MemoryResult<()> {
        let (pml4_i, pdpt_i, pd_i, pt_i) = Self::indices_of_addr(virt);

        let pt = self.get_table_or_create(&[pml4_i, pdpt_i, pd_i])?;
        pt.0[pt_i] = (phys & !0xFFF) | PT_PAGE_PRESENT | PT_PAGE_WRITE;

        Ok(())
    }

    pub fn is_mapped(&self, virt: VirtAddr) -> bool {
        let mut table = PageTable(unsafe { core::slice::from_raw_parts_mut(self.0.as_ptr() as *mut _, 512) });
        let (pml4, pdpt, pd, pt) = Self::indices_of_addr(virt);
        let indices = [pml4, pdpt, pd];

        for (i, index) in indices.iter().enumerate() {
            if table.0[*index] & PT_PAGE_PRESENT == 0 {
                return false;
            }

            unsafe { table = Self::get_page_table_unchecked(3 - i, &indices[..i]); }
        }

        table.0[pt] & PT_PAGE_PRESENT != 0
    }

    pub fn unmap_page(&mut self, virt: VirtAddr) -> MemoryResult<()> {
        if !self.is_mapped(virt) {
            Err(MemoryError::PageNotAllocated)
        } else {
            let (pml4, pdpt, pd, pt) = Self::indices_of_addr(virt);
            let table = unsafe { Self::get_page_table_unchecked(PT_LEVEL_PT, &[pml4, pdpt, pd]) };
            table.0[pt] = 0;
            Ok(())
        }
    }
}