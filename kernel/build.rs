pub fn main() {
    println!("cargo:rerun-if-changed=link.ld");
    println!("cargo:rerun-if-changed=x86_64-unknown-groveos.json");
    println!("cargo:rerun-if-changed=config.toml");
}