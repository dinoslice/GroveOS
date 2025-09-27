use core::arch::asm;

#[repr(C)]
pub struct BootInfo {
    pub framebuffer_ptr: *mut u32,
    pub framebuffer_size: usize,

    pub memory_bitmap_ptr: *mut u8,
    pub memory_bitmap_size: usize,
    pub memory_size: usize,
    pub memory_used: usize,
}

impl BootInfo {
    #[cfg(feature = "uefi")]
    pub fn build() -> Self {
        use uefi::proto::console::gop::GraphicsOutput;
        use uefi::mem::memory_map::MemoryMap;
        use uefi::boot::*;

        let graphics_handle = get_handle_for_protocol::<GraphicsOutput>().expect("Failed to get graphics handle");
        let mut graphics_protocol = unsafe {
            open_protocol::<GraphicsOutput>(
                OpenProtocolParams { handle: graphics_handle, agent: image_handle(), controller: None },
                OpenProtocolAttributes::GetProtocol
            ).expect("Failed to get graphics protocol")
        };

        let memory_map = memory_map(MemoryType::LOADER_DATA).expect("Failed to get memory map");
        let mut mem_pages = 0;
        let excluded_types = [
            MemoryType::RESERVED,
            MemoryType::UNUSABLE,
            MemoryType::PAL_CODE,
            MemoryType::PERSISTENT_MEMORY,
        ];

        for entry in memory_map.entries() {
            if excluded_types.contains(&entry.ty) { continue; }

            mem_pages += entry.page_count;
        }

        let bitmap_size = (mem_pages + 8 - 1) / 8;
        let bitmap = allocate_pool(MemoryType::LOADER_DATA, bitmap_size as usize).expect("Failed to allocate memory for bitmap");

        let bitmap_arr = unsafe { core::slice::from_raw_parts_mut(bitmap.as_ptr(), bitmap_size as usize) };
        bitmap_arr.fill(0);

        let mut used_pages = 0;
        for entry in memory_map.entries() {
            if entry.ty != MemoryType::LOADER_DATA { continue; }

            used_pages += entry.page_count;
            for page in 0..entry.page_count {
                let idx = (entry.phys_start + page) / 8;
                let offset = (entry.phys_start + page) % 8;
                bitmap_arr[idx as usize] |= 1 << offset;
            }
        }

        Self {
            framebuffer_ptr: graphics_protocol.frame_buffer().as_mut_ptr().cast(),
            framebuffer_size: graphics_protocol.current_mode_info().stride() * graphics_protocol.current_mode_info().resolution().1,
            memory_bitmap_ptr: bitmap.as_ptr(),
            memory_bitmap_size: bitmap_size as usize,
            memory_size: mem_pages as usize,
            memory_used: used_pages as usize,
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