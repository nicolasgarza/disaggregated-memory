mod client;
mod errors;
mod kv;
mod proto;

use crate::proto::memory::{AllocationError, DeallocationError, MemoryAccessError};
use client::MemoryClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MemoryClient::new("http://[::1]:50051".to_string()).await?;

    // allocate memory
    match client.allocate_memory(1024).await {
        Ok(size) => println!("Allocated memory of size: {}", size),
        Err(AllocationError::AllocationTooLarge) => println!("Allocation too large"),
        Err(AllocationError::InsufficientMemory) => println!("Insufficient memory"),
        Err(_) => println!("Unknown allocation error"),
    }

    // write to memory
    let id = 0;
    let data = vec![1, 2, 3, 4];

    Ok(())
}
