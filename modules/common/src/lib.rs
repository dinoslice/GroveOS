#![no_std]

mod boot_info;

pub use boot_info::*;

pub fn halt() -> ! {
    unsafe {
        loop {
            core::arch::asm!("hlt");
        }
    }
}