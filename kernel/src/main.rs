#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(export_name = "_start")]
pub fn kernel_main() {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}