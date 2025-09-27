# GroveOS

**GroveOS** is an open-source, hobby Operating System built from the ground up in rust for fun

---

## What is GroveOS?
GroveOS is a small, modular kernel and operating system written in Rust. It's built with a focus on:
- Clean architecture
- Extensibility through dynamically loaded modules
- Full control over low-level memory and system features

This project is not intended to be production-ready. It's a playground for experimentation with low-level system development!

---

## Features (Planned)
While GroveOS is still in early development, here's the planned roadmap:
- [ ] Full Virtual Memory Management System
- [ ] Modular design with dynamically loaded modules
- [ ] Custom heap implementation
- [ ] Filesystem operations
- [ ] User programs
- [ ] Built-in shell
- [ ] Custom `libc` implementation

---

## Current Status
GroveOS is in **early development**.

However, the `main` branch mostly has working code that:
- Builds without errors
- Will boot successfully in QEMU (but does pretty much nothing as of now)

---

## How to Build

### Prerequisites
- Rust (with nightly toolchain)
- QEMU

### Build the Project
```bash
git clone https://github.com/MSKatKing/GroveOS.git
cd groveos
cargo build-os # Builds the project and outputs the result in ./esp. Image file support coming soon...
cargo run-os # Builds and runs the project with QEMU
```

---

## Sources
Most, if not all, of the documentation and information used to create this project was found on [the OSDev wiki](https://wiki.osdev.org).
Huge shoutout to the wonderful contributors to that project! This project truly would not exist without them.

---

## How to Contribute
Coming soon...

For now, if you want to suggest a feature be added just create an issue and describe your idea there!