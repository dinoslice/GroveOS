#![no_std]
#![no_main]
extern crate alloc;

mod heap;
mod page_table;

use alloc::vec;
use log::info;
use uefi::prelude::*;
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::media::file::{File, FileAttribute, FileHandle, FileInfo, FileMode};
use uefi::proto::media::fs::SimpleFileSystem;

#[entry]
fn efi_main() -> Status {
    let Ok(_) = uefi::helpers::init() else { return Status::NOT_STARTED };

    let mut kernel = load_kernel().expect("Failed to load kernel");

    let err = kernel.get_info::<FileInfo>(&mut [0; 0]).err().expect("Failed to get size of kernel info");
    let mut buffer = vec![0u8; err.data().expect("Failed to get size of kernel info")];
    let info = kernel.get_info::<FileInfo>(&mut buffer).expect("Failed to get kernel info");

    info!("[!] Allocated file info: {:#?}", info);

    loop { }
    Status::SUCCESS
}

fn load_kernel() -> Option<FileHandle> {
    let image = boot::open_protocol_exclusive::<LoadedImage>(boot::image_handle()).ok()?;
    let mut fs = boot::open_protocol_exclusive::<SimpleFileSystem>(image.device()?).ok()?;

    let mut directory = fs.open_volume().ok()?;

    directory.open(cstr16!("kernel.elf"), FileMode::Read, FileAttribute::READ_ONLY).ok()
}