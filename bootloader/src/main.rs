#![no_std]
#![no_main]

use log::info;
use uefi::prelude::*;

#[entry]
fn efi_main() -> Status {
    let Ok(_) = uefi::helpers::init() else { return Status::NOT_STARTED };

    info!("Hello, world!");

    loop { }
    Status::SUCCESS
}