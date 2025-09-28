#![no_std]

mod virtual_memory;
mod page_table;
mod physical_memory;
mod err;

use common::BootInfo;
pub use virtual_memory::*;
pub use err::*;
use crate::physical_memory::PhysicalMemoryAllocator;

pub fn init_module(boot_info: &BootInfo) {
    PhysicalMemoryAllocator::init(boot_info);
}

pub fn free_page_count() -> MemoryResult<usize> {
    let allocator = PhysicalMemoryAllocator::get()?;
    Ok(allocator.free_page_count())
}

pub fn used_page_count() -> MemoryResult<usize> {
    let allocator = PhysicalMemoryAllocator::get()?;
    Ok(allocator.used_page_count())
}

pub fn total_page_count() -> MemoryResult<usize> {
    Ok(free_page_count()? + used_page_count()?)
}