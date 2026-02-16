use std::process::Command;

fn main() {
    let mut uefi_build_cmd = Command::new("cargo");
    uefi_build_cmd.args(&["build", "--target", "x86_64-unknown-uefi"]);
    uefi_build_cmd.current_dir("bootloader");

    let Ok(status) = uefi_build_cmd.status() else {
        return;
    };

    if !status.success() {
        eprintln!("UEFI bootloader build failed with exit code {}", status.code().unwrap_or(-1));
        return;
    }

    let mut kernel_build_cmd = Command::new("cargo");
    kernel_build_cmd.args(&["build"]);
    kernel_build_cmd.current_dir("kernel");

    let Ok(status) = kernel_build_cmd.status() else {
        return;
    };

    if !status.success() {
        eprintln!("Kernel build failed with exit code {}", status.code().unwrap_or(-1));
        return;
    }

    println!("Done");
}
