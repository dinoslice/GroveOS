use core::arch::asm;

#[repr(C)]
pub struct BootInfo {
    pub framebuffer_ptr: *mut u32,
    pub framebuffer_size: usize,
}

impl BootInfo {
    #[cfg(feature = "uefi")]
    pub fn build() -> Self {
        use uefi::proto::console::gop::GraphicsOutput;
        use uefi::boot::*;

        let graphics_handle = get_handle_for_protocol::<GraphicsOutput>().expect("Failed to get graphics handle");
        let mut graphics_protocol = unsafe {
            open_protocol::<GraphicsOutput>(
                OpenProtocolParams { handle: graphics_handle, agent: image_handle(), controller: None },
                OpenProtocolAttributes::GetProtocol
            ).expect("Failed to get graphics protocol")
        };

        Self {
            framebuffer_ptr: graphics_protocol.frame_buffer().as_mut_ptr().cast(),
            framebuffer_size: graphics_protocol.current_mode_info().stride() * graphics_protocol.current_mode_info().resolution().1,
        }
    }

    #[cfg(feature = "uefi")]
    pub fn store(&self) {
        unsafe {
            asm!("mov rdi, {boot_info}", boot_info = in(reg) self as *const BootInfo);
        }
    }

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

    pub fn framebuffer(&self) -> &mut [u32] {
        unsafe { core::slice::from_raw_parts_mut(self.framebuffer_ptr, self.framebuffer_size) }
    }
}