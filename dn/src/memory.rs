use crate::errors::{AllocationError, DeallocationError, MemoryAccessError};
use std::collections::HashMap;

pub struct DataNode {
    mem: HashMap<usize, Vec<u8>>,
    next_id: usize,
}

const MAX_ALLOCATION: usize = 1024 * 1024; // 1mb

impl DataNode {
    pub fn new() -> Self {
        return DataNode {
            mem: HashMap::new(),
            next_id: 0,
        };
    }

    pub fn allocate_memory(&mut self, size: usize) -> Result<usize, AllocationError> {
        (size <= MAX_ALLOCATION)
            .then(|| {
                let id = self.next_id;
                self.mem.insert(id, vec![0u8; size]);
                self.next_id += 1;
                id
            })
            .ok_or(AllocationError::AllocationTooLarge)
    }

    pub fn free_memory(&mut self, id: usize) -> Result<(), DeallocationError> {
        self.mem
            .remove(&id)
            .map(|_| ())
            .ok_or(DeallocationError::InvalidMemoryAddress)
    }

    pub fn read_memory(
        &self,
        id: usize,
        offset: usize,
        length: usize,
    ) -> Result<&[u8], MemoryAccessError> {
        self.mem
            .get(&id)
            .ok_or(MemoryAccessError::InvalidMemoryAddress)
            .and_then(|memory| {
                memory
                    .get(offset..offset + length)
                    .ok_or(MemoryAccessError::OutOfBoundsAccess)
            })
    }

    pub fn write_memory(
        &mut self,
        id: usize,
        offset: usize,
        data: &[u8],
    ) -> Result<(), MemoryAccessError> {
        self.mem
            .get_mut(&id)
            .ok_or(MemoryAccessError::InvalidMemoryAddress)
            .and_then(|memory| {
                if offset + data.len() <= memory.len() {
                    memory[offset..offset + data.len()].copy_from_slice(data);
                    Ok(())
                } else {
                    Err(MemoryAccessError::OutOfBoundsAccess)
                }
            })
    }
}
