#![no_std]

use core::fmt::Write;
use common::BootInfo;
use crate::text_framebuffer::TextFramebufferWriter;

mod text_buffer;
mod font;
mod text_framebuffer;

static mut KERNEL_FRAMEBUFFER: Option<TextFramebufferWriter> = None;

pub fn init_framebuffer(boot_info: &BootInfo) {
    unsafe {
        KERNEL_FRAMEBUFFER = Some(TextFramebufferWriter::init(boot_info));
    }
}

pub fn framebuffer_writer() -> &'static mut TextFramebufferWriter {
    #[allow(static_mut_refs)]
    unsafe {
        KERNEL_FRAMEBUFFER.as_mut().unwrap()
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;

        $crate::framebuffer_writer().write_fmt(format_args!($($arg)*)).unwrap();
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}