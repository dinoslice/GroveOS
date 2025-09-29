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

impl PageRange {
    pub fn new(start_page: u32, end_page: u32) -> Self {
        PageRange { start_page, end_page }
    }

    pub fn contains(&self, page: u32) -> bool {
        page >= self.start_page && page < self.end_page
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum NodeStatus {
    Free,
    Used,
    Reserved
}

impl NodeStatus {
    pub fn can_use(&self) -> bool {
        self == &NodeStatus::Free
    }
}