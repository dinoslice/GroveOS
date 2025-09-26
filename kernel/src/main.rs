#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub fn kernel_main() {

}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}