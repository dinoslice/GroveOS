pub struct NodeAllocator {
    bitmap: [u8; 1280],
    allocated_groups: u8,
}