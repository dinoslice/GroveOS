use core::fmt::Write;
use common::BootInfo;
use crate::font::{PSFFont, KERNEL_FONT};
use crate::text_buffer::TextBuffer;

pub struct TextFramebufferWriter {
    framebuffer: &'static mut [u32],
    width: usize,
    height: usize,

    font: &'static PSFFont<'static>,

    fg_color: u32,
    bg_color: u32,

    text_buffer: TextBuffer,
    start_line: usize,
}

impl TextFramebufferWriter {
    pub fn init(boot_info: &BootInfo) -> Self {
        Self {
            framebuffer: unsafe { core::slice::from_raw_parts_mut(boot_info.framebuffer_ptr, boot_info.framebuffer_size) },
            width: boot_info.framebuffer_width,
            height: boot_info.framebuffer_height,
            fg_color: 0xFFFF_FFFF,
            bg_color: 0x0000_0000,
            text_buffer: TextBuffer::new(),
            start_line: 0,
            font: &KERNEL_FONT,
        }
    }

    pub fn clear(&mut self) {
        self.start_line = 0;
        self.text_buffer.clear();
    }

    pub fn update(&mut self) {
        let mut cursor_x = 0;
        let mut cursor_y = 0;

        fn put_char(framebuffer: &mut [u32], font: &PSFFont, char: char, x: usize, y: usize, width: usize, height: usize, fg_color: u32, bg_color: u32) {
            let glyph = font.get_char(char);
            let glyph_width = font.width as usize;
            let glyph_height = font.height as usize;

            let x = x * glyph_width;
            let y = y * glyph_height;

            let bytes_per_row = (font.width as usize + 7) / 8;

            for row in 0..glyph_height {
                let row_start = row * bytes_per_row;

                for col in 0..glyph_width {
                    let byte_index = row_start + col / 8;
                    let bit_index = 7 - (col % 8);

                    if byte_index >= glyph.len() {
                        continue;
                    }

                    let byte = glyph[byte_index];
                    let pixel_on = (byte >> bit_index) & 1;

                    let x = x + col;
                    let y = y + row;

                    if x < width && y < height {
                        let pixel_index = y * width + x;
                        framebuffer[pixel_index] = if pixel_on != 0 {
                            fg_color
                        } else {
                            bg_color
                        };
                    }
                }
            }
        }

        for line in self.start_line..(self.start_line + self.height / self.font.height as usize) {
            if let Some(line) = self.text_buffer.get_line(line) {
                for char in line.chars() {
                    put_char(self.framebuffer, self.font, char, cursor_x, cursor_y, self.width as _, self.height as _, self.fg_color, self.bg_color);
                    cursor_x += 1;

                    if cursor_x >= self.width / self.font.width as usize {
                        cursor_y += 1;
                        cursor_x = 0;
                    }

                    if cursor_y >= self.height / self.font.height as usize {
                        return;
                    }
                }
            } else {
                return;
            }


            cursor_y += 1;
            cursor_x = 0;

            if cursor_y >= self.height / self.font.height as usize {
                return;
            }
        }
    }
}

impl Write for TextFramebufferWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let _ = self.text_buffer.write_str(s)?;
        self.update();
        Ok(())
    }
}