#![no_std]
#![no_main]
extern crate alloc;

mod heap;
mod page_table;

use alloc::vec;
use goblin::elf::Elf;
use goblin::elf::program_header::PT_LOAD;
use log::info;
use uefi::boot::{AllocateType, MemoryType};
use uefi::prelude::*;
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::media::file::{File, FileAttribute, FileHandle, FileInfo, FileMode};
use uefi::proto::media::fs::SimpleFileSystem;
use crate::page_table::PageTable;

#[entry]
fn efi_main() -> Status {
    let Ok(_) = uefi::helpers::init() else { return Status::NOT_STARTED };

    let mut kernel = load_kernel().expect("Failed to load kernel");

    let err = kernel.get_info::<FileInfo>(&mut [0; 0]).err().expect("Failed to get size of kernel info");
    let mut buffer = vec![0u8; err.data().expect("Failed to get size of kernel info")];
    let info = kernel.get_info::<FileInfo>(&mut buffer).expect("Failed to get kernel info");

    let mut kernel = kernel.into_regular_file().expect("Failed to get regular file handle");

    let mut header = vec![0u8; info.file_size() as usize];
    kernel.read(&mut header).expect("Failed to read kernel file");

    let elf = Elf::parse(&header).expect("Failed to parse ELF file");

    let mut pml4 = PageTable::new();

    for phdr in elf.program_headers {
        if phdr.p_type == PT_LOAD {
            let pages = (phdr.p_memsz + 0x1000 - 1) / 0x1000;
            let allocated_space = boot::allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, pages as _).expect("Failed to allocate memory");

            if phdr.p_memsz > phdr.p_filesz {
                let buffer = unsafe { core::slice::from_raw_parts_mut(allocated_space.as_ptr().add(phdr.p_filesz as usize), (phdr.p_memsz - phdr.p_filesz) as usize) };
                buffer.fill(0);
            }

            unsafe {
                header.as_ptr().offset(phdr.p_offset as isize)
                    .copy_to_nonoverlapping(allocated_space.as_ptr(), phdr.p_filesz as usize);
            }

            for i in 0..((phdr.p_memsz + 0x1000 - 1) / 0x1000) {
                pml4.map_page(phdr.p_vaddr + i * 0x1000, allocated_space.as_ptr() as u64 + i * 0x1000, PageTable::PAGE_WRITE);
            }
        }
    }

    info!("Finished mapping kernel! Entry @ {:x}", elf.entry);

    loop { }
    Status::SUCCESS
}

fn load_kernel() -> Option<FileHandle> {
    let image = boot::open_protocol_exclusive::<LoadedImage>(boot::image_handle()).ok()?;
    let mut fs = boot::open_protocol_exclusive::<SimpleFileSystem>(image.device()?).ok()?;

    let mut directory = fs.open_volume().ok()?;

    directory.open(cstr16!("kernel.elf"), FileMode::Read, FileAttribute::READ_ONLY).ok()
}