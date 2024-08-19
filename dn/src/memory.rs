use crate::errors::{AllocationError, DeallocationError, MemoryAccessError};

pub struct DataNode {
    mem: Vec<Option<Vec<u8>>>,
}

const MAX_ALLOCATION: usize = 1024 * 1024; // 1mb

impl DataNode {
    pub fn new() -> Self {
        return DataNode { mem: Vec::new() };
    }

    pub fn allocate_memory(&mut self, size: usize) -> Result<usize, AllocationError> {
        if size > MAX_ALLOCATION {
            return Err(AllocationError::AllocationTooLarge);
        }

        let id = self.mem.len();
        self.mem.push(Some(vec![0u8; size]));
        Ok(id)
    }

    pub fn free_memory(&mut self, id: usize) -> Result<(), DeallocationError> {
        self.mem
            .get_mut(id)
            .ok_or(DeallocationError::InvalidMemoryAddress)
            .and_then(|slot| {
                if slot.is_some() {
                    *slot = None;
                    Ok(())
                } else {
                    Err(DeallocationError::InvalidMemoryAddress)
                }
            })
    }

    pub fn read_memory(
        &self,
        id: usize,
        offset: usize,
        length: usize,
    ) -> Result<&[u8], MemoryAccessError> {
        println!("{:?}\n\n\n", self.mem);
        println!("-----New access----");
        self.mem
            .get(id)
            .ok_or(MemoryAccessError::InvalidMemoryAddress)
            .and_then(|slot| slot.as_ref().ok_or(MemoryAccessError::InvalidMemoryAddress))
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
            .get_mut(id)
            .ok_or(MemoryAccessError::InvalidMemoryAddress)
            .and_then(|slot| slot.as_mut().ok_or(MemoryAccessError::InvalidMemoryAddress))
            .and_then(|memory| {
                if offset + data.len() <= memory.len() {
                    memory[offset..offset + data.len()].copy_from_slice(data);
                    Ok(())
                } else {
                    Err(MemoryAccessError::OutOfBoundsAccess)
                }
            })
    }

    pub fn get_memory_size(&self, id: usize) -> Result<usize, MemoryAccessError> {
        self.mem
            .get(id)
            .ok_or(MemoryAccessError::InvalidMemoryAddress)
            .and_then(|slot| slot.as_ref().ok_or(MemoryAccessError::InvalidMemoryAddress))
            .map(|memory| memory.len())
    }
}
