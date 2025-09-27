#![no_std]
#![no_main]

use core::panic::PanicInfo;
use common::BootInfo;

#[unsafe(export_name = "_start")]
pub extern "C" fn kernel_main() -> ! {
    let boot_info = unsafe { BootInfo::load() } ;
    let framebuffer = boot_info.framebuffer();

    framebuffer.fill(0);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}