mod client;
mod errors;
mod kv;
mod proto;

use crate::kv::KeyValueStore;
use client::MemoryClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MemoryClient::new("http://[::1]:50051".to_string()).await?;
    let mut kv_store = KeyValueStore::new(client).await?;

    kv_store.set("name", b"Alice").await?;
    println!("Set key Alice");
    kv_store.set("age", b"30").await?;
    println!("Set age 30");
    kv_store.set("city", b"New York").await?;
    println!("Set city New York");

    if let Some(name) = kv_store.get("name").await? {
        println!("Name: {}", String::from_utf8_lossy(&name));
    }
    if let Some(age) = kv_store.get("age").await? {
        println!("Age: {}", String::from_utf8_lossy(&age));
    }
    if let Some(city) = kv_store.get("city").await? {
        println!("City: {}", String::from_utf8_lossy(&city));
    }

    kv_store.set("age", b"31").await?;
    if let Some(age) = kv_store.get("age").await? {
        println!("Updated age: {}", String::from_utf8_lossy(&age));
    }

    if kv_store.delete("city").await? {
        println!("Deleted 'city' key");
    }

    if let None = kv_store.get("city").await? {
        println!("City key no longer exists");
    }

    Ok(())
}
