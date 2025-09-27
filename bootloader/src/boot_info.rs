use core::arch::asm;
use uefi::boot;
use uefi::boot::{OpenProtocolAttributes, OpenProtocolParams, PAGE_SIZE};
use uefi::proto::console::gop::GraphicsOutput;
use crate::page_table::PageTable;

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
            framebuffer_size: graphics_protocol.current_mode_info().stride() * graphics_protocol.current_mode_info().resolution().1,
        }
    }

    pub fn map_contents(&self, pml4: &mut PageTable) {
        for page in 0..((self.framebuffer_size * size_of::<u32>() + 0x1000 - 1) / 0x1000) as u64 {
            pml4.map_page(self.framebuffer_ptr as u64 + page * PAGE_SIZE as u64, self.framebuffer_ptr as u64 + page * PAGE_SIZE as u64, PageTable::PAGE_WRITE);
        }
    }

    pub fn store(&self) {
        unsafe {
            asm!("mov rdi, {boot_info}", boot_info = in(reg) self as *const BootInfo);
        }
    }
}