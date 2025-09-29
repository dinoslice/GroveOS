use core::ptr::NonNull;

pub enum Node {
    Leaf {
        page_range: PageRange,
        status: NodeStatus,
    },
    Branch {
        page_range_a: PageRange,
        page_range_b: PageRange,
        a: NonNull<Node>,
        b: NonNull<Node>,
    }
}

pub struct PageRange {
    pub start_page: u32,
    pub end_page: u32,
}

#[repr(u8)]
pub enum NodeStatus {
    Free,
    Used,
    Reserved
}