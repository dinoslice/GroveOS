use core::arch::asm;
use uefi::boot;
use uefi::boot::{OpenProtocolAttributes, OpenProtocolParams};
use uefi::proto::console::gop::GraphicsOutput;

#[repr(C)]
pub struct BootInfo {
    framebuffer_ptr: *mut u32,
    framebuffer_size: usize,
}

impl BootInfo {
    pub fn build() -> Self {
        let graphics_handle = boot::get_handle_for_protocol::<GraphicsOutput>().expect("Failed to get graphics handle");
        let mut graphics_protocol = unsafe {
            boot::open_protocol::<GraphicsOutput>(
                OpenProtocolParams { handle: graphics_handle, agent: boot::image_handle(), controller: None },
                OpenProtocolAttributes::GetProtocol
            ).expect("Failed to get graphics protocol")
        };

        Self {
            framebuffer_ptr: graphics_protocol.frame_buffer().as_mut_ptr().cast(),
            framebuffer_size: graphics_protocol.frame_buffer().size()
        }
    }

    pub fn store(&self) {
        unsafe {
            asm!("mov rdi, {boot_info}", boot_info = in(reg) self as *const BootInfo);
        }
    }
}