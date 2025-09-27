use core::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum MemoryError {
    OutOfVirtualMemory,
    OutOfPhysicalMemory,
    RequestedAddressInUse,
    PageNotAllocated,
    PhysicalAllocatorNotInitialized,
}

impl Display for MemoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            MemoryError::OutOfVirtualMemory => write!(f, "System is out of virtual memory"),
            MemoryError::OutOfPhysicalMemory => write!(f, "System is out of physical memory"),
            MemoryError::RequestedAddressInUse => write!(f, "Requested address is already in use"),
            MemoryError::PageNotAllocated => write!(f, "Cannot deallocate page because it is not allocated"),
            MemoryError::PhysicalAllocatorNotInitialized => write!(f, "PhysicalMemoryAllocator::init has not been called yet"),
        }
    }
}

pub type MemoryResult<T> = Result<T, MemoryError>;