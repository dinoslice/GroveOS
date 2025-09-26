use uefi::prelude::*;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;
use uefi::boot::MemoryType;

#[global_allocator]
static ALLOCATOR: HeapAllocator = HeapAllocator;

pub struct HeapAllocator;

unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        boot::allocate_pool(MemoryType::LOADER_DATA, layout.size()).expect("Failed to allocate memory").as_ptr()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            boot::free_pool(NonNull::new(ptr).unwrap()).expect("Failed to deallocate memory");
        }
    }
}