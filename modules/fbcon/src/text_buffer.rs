use core::fmt::Write;

pub struct TextBuffer {
    ptr: &'static mut [u8],
    cursor: usize,
}

impl TextBuffer {
    pub fn new() -> TextBuffer {
        let page = memory::PageAllocator::kernel().expect("failed to get kernel allocator").allocate_page().expect("failed to allocate page");

        Self {
            ptr: unsafe { core::slice::from_raw_parts_mut(page.cast::<u8>(), 0x1000) },
            cursor: 0,
        }
    }

    pub fn clear(&mut self) {
        self.ptr.fill(0);
        self.cursor = 0;
    }

    pub fn resize(&mut self) {
        let curr_pages = self.ptr.len() / 0x1000;
        let new_pages = memory::PageAllocator::kernel().expect("failed to get kernel allocator").allocate_pages(curr_pages + 1).expect("failed to allocate pages");

        let new_buffer = unsafe { core::slice::from_raw_parts_mut(new_pages.cast::<u8>(), 0x1000 * (curr_pages + 1)) };
        new_buffer.copy_from_slice(&self.ptr);

        memory::PageAllocator::kernel().expect("failed to get kernel allocator").deallocate_pages(self.ptr.as_mut_ptr().cast(), curr_pages).expect("failed to deallocate pages");

        self.ptr = new_buffer;
    }

    pub fn get_line(&self, line: usize) -> Option<&str> {
        let mut curr_line = 0;
        let mut start = 0;

        for (i, &b) in self.ptr[..self.cursor].iter().enumerate() {
            if curr_line == line {
                let end = match self.ptr[i..self.cursor].iter().position(|&c| c == b'\n') {
                    Some(end) => i + end,
                    None => continue,
                };
                return core::str::from_utf8(&self.ptr[start..end]).ok();
            }

            if b == b'\n' {
                curr_line += 1;
                start = i + 1;
            }
        }

        if curr_line == line {
            return core::str::from_utf8(&self.ptr[start..self.cursor]).ok();
        }

        None
    }
}

impl Write for TextBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            if self.cursor >= self.ptr.len() {
                self.resize();
            }

            self.ptr[self.cursor] = c;
            self.cursor += 1;
        }

        Ok(())
    }
}