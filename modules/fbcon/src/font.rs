pub const KERNEL_FONT: PSFFont<'static> = PSFFont::load_from_file(include_bytes!("fonts/Tamsyn8x16r.psf"));

pub struct PSFFont<'file> {
    magic: u32,
    version: u32,
    header_size: u32,
    flags: u32,
    num_glyphs: u32,
    bytes_per_glyph: u32,
    pub height: u32,
    pub width: u32,
    file: &'file [u8],
}

impl<'file> PSFFont<'file> {
    pub const fn load_from_file(bytes: &'file [u8]) -> PSFFont<'file> {
        const fn read_u32_le(bytes: &[u8], offset: usize) -> u32 {
            u32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]])
        }

        let magic = read_u32_le(bytes, 0);
        let version = read_u32_le(bytes, 4);
        let header_size = read_u32_le(bytes, 8);
        let flags = read_u32_le(bytes, 12);
        let num_glyphs = read_u32_le(bytes, 16);
        let bytes_per_glyph = read_u32_le(bytes, 20);
        let height = read_u32_le(bytes, 24);
        let width = read_u32_le(bytes, 28);

        Self {
            magic,
            version,
            header_size,
            flags,
            num_glyphs,
            bytes_per_glyph,
            height,
            width,
            file: bytes,
        }
    }

    pub fn get_char(&self, c: char) -> &[u8] {
        let index = c as usize;

        if index >= self.num_glyphs as usize {
            return self.get_char('?');
        }

        let start = index * self.bytes_per_glyph as usize;
        let end = start + self.bytes_per_glyph as usize;

        if (end + self.header_size as usize) > self.file.len() {
            return self.get_char('?');
        }

        &self.file[start + self.header_size as usize..end + self.header_size as usize]
    }
}