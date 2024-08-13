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
        if size > MAX_ALLOCATION.into() {
            return Err(AllocationError::AllocationTooLarge);
        }
        let id = self.next_id;
        self.mem.insert(id, vec![0u8; size]);
        self.next_id += 1;
        Ok(id)
    }

    pub fn free_memory(&mut self, id: usize) -> Result<(), DeallocationError> {
        if !self.mem.contains_key(&id) {
            return Err(DeallocationError::InvalidMemoryAddress);
        }

        self.mem.remove(&id);
        Ok(())
    }

    pub fn read_memory(
        &self,
        id: usize,
        offset: usize,
        length: usize,
    ) -> Result<&[u8], MemoryAccessError> {
        let memory = self
            .mem
            .get(&id)
            .ok_or(MemoryAccessError::InvalidMemoryAddress)?;
        memory
            .get(offset..offset + length)
            .ok_or(MemoryAccessError::OutOfBoundsAccess)
    }

    pub fn write_memory(
        &mut self,
        id: usize,
        offset: usize,
        data: &[u8],
    ) -> Result<(), MemoryAccessError> {
        let memory = self
            .mem
            .get_mut(&id)
            .ok_or(MemoryAccessError::InvalidMemoryAddress)?;
        if offset + data.len() > memory.len() {
            return Err(MemoryAccessError::OutOfBoundsAccess);
        }
        memory[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }
}
