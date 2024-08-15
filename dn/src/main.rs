mod errors;
mod memory;
mod proto;
mod rpc;

use tonic::transport::Server;

use crate::memory::DataNode;
use crate::proto::memory::memory_server::MemoryServer;
use crate::rpc::MemoryService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let node = DataNode::new();
    let mmry = MemoryService::new(node);

    Server::builder()
        .add_service(MemoryServer::new(mmry))
        .serve(addr)
        .await?;

    Ok(())
}
