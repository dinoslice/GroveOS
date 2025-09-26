#![no_std]
#![no_main]

use core::panic::PanicInfo;
use log::{error, info};
use uefi::prelude::*;

#[entry]
fn efi_main() -> Status {
    let Ok(_) = uefi::helpers::init() else { return Status::NOT_STARTED };

    info!("Hello, world!");

    loop { }
    Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    error!("{}", _info);
    loop {}
}