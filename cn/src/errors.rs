use crate::proto::memory::{AllocationError, DeallocationError, MemoryAccessError};

#[derive(Debug)]
pub enum MemoryError {
    AllocationError(AllocationError),
    DeallocationError(DeallocationError),
    MemoryAccessError(MemoryAccessError),
}

impl std::error::Error for MemoryError {}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::AllocationError(e) => write!(f, "Allocation error: {:?}", e),
            MemoryError::DeallocationError(e) => write!(f, "Deallocation error: {:?}", e),
            MemoryError::MemoryAccessError(e) => write!(f, "Memory access error: {:?}", e),
        }
    }
}

impl From<AllocationError> for MemoryError {
    fn from(error: AllocationError) -> Self {
        MemoryError::AllocationError(error)
    }
}

impl From<DeallocationError> for MemoryError {
    fn from(error: DeallocationError) -> Self {
        MemoryError::DeallocationError(error)
    }
}

impl From<MemoryAccessError> for MemoryError {
    fn from(error: MemoryAccessError) -> Self {
        MemoryError::MemoryAccessError(error)
    }
}
