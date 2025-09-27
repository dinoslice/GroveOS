#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[unsafe(export_name = "_start")]
pub extern "C" fn kernel_main() -> ! {
    let boot_info = unsafe { BootInfo::load() } ;
    let framebuffer = boot_info.framebuffer();

    framebuffer.fill(0);

    loop {}
}

#[repr(C)]
struct BootInfo {
    framebuffer_ptr: *mut u32,
    framebuffer_size: usize,
}

impl BootInfo {
    pub unsafe fn load() -> Self {
        let ptr = unsafe {
            let rdi: u64;
            asm!("mov {}, rdi", out(reg) rdi);
            rdi as *mut BootInfo
        };

        unsafe {
            ptr.read()
        }
    }

    fn framebuffer(&self) -> &mut [u32] {
        unsafe { core::slice::from_raw_parts_mut(self.framebuffer_ptr, self.framebuffer_size) }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}