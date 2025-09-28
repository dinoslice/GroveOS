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
    #[inline(always)]
    pub fn allocate() -> &'static mut Self {
        let ptr = uefi::boot::allocate_pool(uefi::boot::MemoryType::LOADER_DATA, size_of::<Self>()).expect("Failed to allocate boot info");
        unsafe { &mut *(ptr.cast::<Self>().as_ptr()) }
    }

    #[cfg(feature = "uefi")]
    pub fn build(&mut self) {
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

        self.framebuffer_ptr = graphics_protocol.frame_buffer().as_mut_ptr() as *mut u32;
        self.framebuffer_size = graphics_protocol.frame_buffer().size() / size_of::<u32>();

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
            let phys_start = entry.phys_start >> 12;
            for page in 0..entry.page_count {
                let idx = (phys_start + page) / 8;
                let offset = (phys_start + page) % 8;
                bitmap_arr[idx as usize] |= 1 << offset;
            }
        }

        self.memory_bitmap_ptr = bitmap.as_ptr();
        self.memory_bitmap_size = bitmap_size as usize;
        self.memory_size = mem_pages as usize;
        self.memory_used = used_pages as usize;
    }

    #[cfg(feature = "uefi")]
    #[inline(always)]
    pub fn store(&self) {
        unsafe {
            asm!("mov rdi, {boot_info}", boot_info = in(reg) self as *const BootInfo);
        }
    }

    #[inline(always)]
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