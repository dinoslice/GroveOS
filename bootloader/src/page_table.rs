use uefi::boot;
use uefi::boot::{AllocateType, MemoryType};

macro_rules! page_table_index {
    ($addr:expr, $depth:expr) => {
        ((($addr >> (12 + 9 *  $depth)) & 0x1FF) as usize)
    };
}

#[repr(C, align(4096))]
pub struct PageTable(&'static mut [u64]);

impl PageTable {
    pub const PAGE_PRESENT: u64 = 1 << 0;
    pub const PAGE_WRITE: u64 = 1 << 1;
    pub const EXECUTE_DISABLE: u64 = 1 << 63;

    const PAGE_TABLE_FLAGS: u64 = Self::PAGE_PRESENT | Self::PAGE_WRITE;

    pub fn new() -> PageTable {
        let addr = boot::allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, 1).expect("Failed to allocate page table");

        let inner = unsafe { core::slice::from_raw_parts_mut(addr.as_ptr() as *mut u64, 512) };
        inner.fill(0);

        PageTable(inner)
    }

    pub fn as_ptr(&self) -> *const u64 {
        self.0.as_ptr()
    }

    fn get_or_allocate_table(&mut self, idx: usize) -> PageTable {
        if self.0[idx] & Self::PAGE_PRESENT != 0 {
            let other = self.0[idx] & !0xFFF;
            PageTable::from(other as *mut u8)
        } else {
            let other = PageTable::new();
            self.0[idx] = other.as_ptr() as u64 | Self::PAGE_TABLE_FLAGS;
            other
        }
    }

    pub fn map_page(&mut self, virt: u64, phys: u64, flags: u64) {
        let mut pdpt = self.get_or_allocate_table(page_table_index!(virt, 3));
        let mut pd = pdpt.get_or_allocate_table(page_table_index!(virt, 2));
        let pt = pd.get_or_allocate_table(page_table_index!(virt, 1));

        pt.0[page_table_index!(virt, 0)] = (phys & !0xFFF) | Self::PAGE_PRESENT | flags;
    }
}

impl From<*mut u8> for PageTable {
    fn from(value: *mut u8) -> Self {
        let inner = unsafe { core::slice::from_raw_parts_mut(value as *mut u64, 512) };
        Self(inner)
    }
}