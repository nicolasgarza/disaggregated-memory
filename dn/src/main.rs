use std::collections::HashMap;

pub struct Memory {
    mem: HashMap<usize, Vec<u8>>,
    next_id: usize,
}

#[derive(Debug)]
enum AllocationError {
    AllocationTooLarge,
    InsufficientMemory,
}

impl std::fmt::Display for AllocationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AllocationError::AllocationTooLarge => {
                write!(f, "Requested too much memory in allocation")
            }
            AllocationError::InsufficientMemory => write!(f, "Insufficient memory for allocation"),
        }
    }
}
impl std::error::Error for AllocationError {}

#[derive(Debug)]
enum DeallocationError {
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

const MAX_ALLOCATION: u8 = u8::MAX;

fn new() -> Memory {
    return Memory {
        mem: HashMap::new(),
        next_id: 0,
    };
}

impl Memory {
    fn allocate_memory(&mut self, size: usize) -> Result<usize, AllocationError> {
        if size > MAX_ALLOCATION.into() {
            return Err(AllocationError::AllocationTooLarge);
        }
        let id = self.next_id;
        self.mem.insert(id, vec![0u8; size]);
        self.next_id += 1;
        Ok(id)
    }

    fn free_memory(&mut self, id: usize) -> Result<(), DeallocationError> {
        if !self.mem.contains_key(&id) {
            return Err(DeallocationError::InvalidMemoryAddress);
        }

        self.mem.remove(&id);
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
