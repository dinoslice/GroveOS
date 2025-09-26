use std::fs;
use std::process::Command;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.contains(&String::from("--build")) || args.contains(&String::from("--run")) {
        println!("[*] Building bootloader...");
        if !Command::new("cargo").args(&["build", "--package", "bootloader", "--target", "x86_64-unknown-uefi"]).status().unwrap().success() {
            eprintln!("[!] Failed to build bootloader.");
        }

        println!("[*] Preparing output folder...");
        if let Err(err) = fs::create_dir_all("esp/EFI/BOOT") {
            eprintln!("[!] Failed to create build output folder: {}", err);
            return;
        }

        if let Err(err) = fs::copy("target/x86_64-unknown-uefi/debug/bootloader.efi", "esp/EFI/BOOT/BOOTX64.EFI") {
            eprintln!("[!] Failed to move bootloader.efi to build output folder: {}", err);
            return;
        }

        if args.contains(&String::from("--run")) {
            println!("[*] Running qemu...");
            if !Command::new("qemu-system-x86_64").args(&[
                "-drive", "if=pflash,format=raw,readonly=on,file=ovmf/OVMF_CODE.fd",
                "-drive", "if=pflash,format=raw,readonly=on,file=ovmf/OVMF_VARS.fd",
                "-drive", "format=raw,file=fat:rw:esp",
                "-d", "int,cpu_reset",
                "-D", "qemu.log"
            ]).status().unwrap().success() {
                eprintln!("[!] Failed to run qemu-system-x86_64.");
            }
        }
    }
}
