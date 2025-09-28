#![no_std]
#![no_main]
extern crate alloc;

mod heap;
mod page_table;

use alloc::vec;
use core::arch::asm;
use goblin::elf::Elf;
use goblin::elf::program_header::PT_LOAD;
use log::info;
use uefi::boot::{AllocateType, MemoryType, PAGE_SIZE};
use uefi::mem::memory_map::MemoryMap;
use uefi::prelude::*;
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::media::file::{File, FileAttribute, FileHandle, FileInfo, FileMode};
use uefi::proto::media::fs::SimpleFileSystem;
use common::BootInfo;
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

    let memory_map = boot::memory_map(MemoryType::LOADER_DATA).expect("Failed to grab current memory map");
    let excluded_types = [
        MemoryType::RESERVED,
        MemoryType::UNUSABLE,
        MemoryType::PAL_CODE,
        MemoryType::PERSISTENT_MEMORY,
    ];

    for mem_descriptor in memory_map.entries() {
        if excluded_types.contains(&mem_descriptor.ty) { continue; }

        let virt_start = if mem_descriptor.virt_start == 0 { mem_descriptor.phys_start } else { mem_descriptor.virt_start };
        for page in 0..mem_descriptor.page_count {
            pml4.map_page(virt_start + page * PAGE_SIZE as u64, mem_descriptor.phys_start + page * PAGE_SIZE as u64, PageTable::PAGE_WRITE);
        }
    }

    let kernel_entry: extern "C" fn() -> ! = unsafe { core::mem::transmute(elf.entry as *const ()) };

    info!("Kernel entry @ {:x?}", kernel_entry);

    let boot_info = BootInfo::build();
    map_contents(&boot_info, &mut pml4);

    let memory_map = unsafe {
        boot::exit_boot_services(None)
    };

    unsafe {
        let ptr = pml4.as_ptr() as u64;
        asm!("mov cr3, {}", in(reg) ptr);

        boot_info.store();
    }

    kernel_entry()
}

fn load_kernel() -> Option<FileHandle> {
    let image = boot::open_protocol_exclusive::<LoadedImage>(boot::image_handle()).ok()?;
    let mut fs = boot::open_protocol_exclusive::<SimpleFileSystem>(image.device()?).ok()?;

    let mut directory = fs.open_volume().ok()?;

    directory.open(cstr16!("kernel.elf"), FileMode::Read, FileAttribute::READ_ONLY).ok()
}

fn map_contents(boot_info: &BootInfo, pml4: &mut PageTable) {
    for page in 0..((boot_info.framebuffer_size * size_of::<u32>() + 0x1000 - 1) / 0x1000) as u64 {
        pml4.map_page(boot_info.framebuffer_ptr as u64 + page * PAGE_SIZE as u64, boot_info.framebuffer_ptr as u64 + page * PAGE_SIZE as u64, PageTable::PAGE_WRITE);
    }

    let start = boot_info.memory_bitmap_ptr as usize;
    for page in 0..boot_info.memory_size / (8 * PAGE_SIZE) {
        pml4.map_page((start + page * PAGE_SIZE) as u64, (start + page * PAGE_SIZE) as u64, PageTable::PAGE_WRITE);
    }
}