#![no_std]

mod virtual_memory;
mod page_table;
mod physical_memory;
mod kernel;
mod err;

use common::BootInfo;
pub use virtual_memory::*;
pub use err::*;
use crate::physical_memory::PhysicalMemoryAllocator;

pub fn init_module(boot_info: &BootInfo) {
    PhysicalMemoryAllocator::init(boot_info);
}