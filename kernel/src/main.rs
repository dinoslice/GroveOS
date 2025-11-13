#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use common::BootInfo;
use fbcon::println;
use memory::PageAllocator;

static mut BOOT_INFO: Option<BootInfo> = None;

fn boot_info() -> &'static BootInfo {
    #[allow(static_mut_refs)]
    unsafe {
        BOOT_INFO.as_ref().unwrap()
    }
}

#[unsafe(export_name = "_start")]
pub extern "C" fn kernel_main() -> ! {
    unsafe { BOOT_INFO = Some(BootInfo::load()) };
    let framebuffer = boot_info().framebuffer();

    framebuffer.fill(0);

    memory::init_module(boot_info());
    fbcon::init_framebuffer(boot_info());

    let page = PageAllocator::kernel().unwrap().allocate_pages(4).unwrap();
    let a = unsafe { core::slice::from_raw_parts_mut(page.cast::<u8>(), 0x4000) };

    unsafe {
        asm!("mov rax, {}", in(reg) page)
    }

    println!("hello, world!");
    println!("test multi-line\ntest multi-line 2");

    common::halt();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    common::halt();
}