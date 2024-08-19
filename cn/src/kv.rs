use crate::client::MemoryClient;
use crate::errors::MemoryError;
use crate::proto::memory::AllocationError;
use std::collections::HashMap;

const HEADER_SIZE: u64 = 1024; //metadata header
const MAX_KEY_SIZE: usize = 256;
const MAX_VALUE_SIZE: usize = 1024;

pub struct KeyValueStore {
    client: MemoryClient,
    //header_id: u64,
    data: HashMap<String, (u64, u64)>, // map keys to (memory_id, offset)
}

impl KeyValueStore {
    pub async fn new(mut client: MemoryClient) -> Result<Self, MemoryError> {
        let _header_id = client.allocate_memory(HEADER_SIZE).await?;
        Ok(Self {
            client,
            //header_id,
            data: HashMap::new(),
        })
    }

    pub async fn set(&mut self, key: &str, value: &[u8]) -> Result<(), MemoryError> {
        if key.len() > MAX_KEY_SIZE || value.len() > MAX_VALUE_SIZE {
            return Err(MemoryError::AllocationError(
                AllocationError::InsufficientMemory,
            ));
        }

        let total_size = key.len() + value.len();
        let memory_id = self.client.allocate_memory(total_size as u64).await?;
        println!("Allocated: {} bytes", memory_id);

        // write key
        self.client
            .write(memory_id, 0, key.as_bytes().to_vec())
            .await?;

        // write value
        self.client
            .write(memory_id, key.len() as u64, value.to_vec())
            .await?;

        if let Some((old_id, _)) = self.data.insert(key.to_string(), (memory_id, 0)) {
            self.client.free(old_id).await?;
        }

        Ok(())
    }

    pub async fn get(&mut self, key: &str) -> Result<Option<Vec<u8>>, MemoryError> {
        if let Some(&(memory_id, offset)) = self.data.get(key) {
            let key_size = key.len() as u64;
            println!(
                "memory_id: {}, offset: {}, key_size: {}, length: 8",
                memory_id, offset, key_size
            );
            let total_size = self.client.read(memory_id, 0, u64::MAX).await?;

            let value_start = key_size as usize;
            let value = total_size[value_start..].to_vec();
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(&mut self, key: &str) -> Result<bool, MemoryError> {
        if let Some((memory_id, _)) = self.data.remove(key) {
            self.client.free(memory_id).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
