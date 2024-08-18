#[derive(Debug)]
pub enum AllocationError {
    AllocationTooLarge,
}

impl std::fmt::Display for AllocationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AllocationError::AllocationTooLarge => {
                write!(f, "Requested too much memory in allocation")
            }
        }
    }
}
impl std::error::Error for AllocationError {}

#[derive(Debug)]
pub enum DeallocationError {
    InvalidMemoryAddress,
}

impl std::fmt::Display for DeallocationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeallocationError::InvalidMemoryAddress => {
                write!(f, "Couldn't locate memory address to deallocate")
            }
        }
    }
}
impl std::error::Error for DeallocationError {}

#[derive(Debug)]
pub enum MemoryAccessError {
    InvalidMemoryAddress,
    OutOfBoundsAccess,
}

impl std::fmt::Display for MemoryAccessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryAccessError::InvalidMemoryAddress => write!(f, "Couldn't locate memory address"),
            MemoryAccessError::OutOfBoundsAccess => write!(f, "Memory access out of bounds"),
        }
    }
}
