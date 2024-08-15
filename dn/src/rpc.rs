use crate::errors::{AllocationError, DeallocationError, MemoryAccessError};

use crate::memory::DataNode;
use std::sync::Arc;
use tokio::sync::Mutex;

use proto::memory_server::{Memory, MemoryServer};
use tonic::{Code, Status};
mod proto {
    tonic::include_proto!("memory");
}

struct MemoryService {
    data_node: Arc<Mutex<DataNode>>,
}

impl MemoryService {
    fn new(data_node: DataNode) -> Self {
        MemoryService {
            data_node: Arc::new(Mutex::new(data_node)),
        }
    }
}

#[tonic::async_trait]
impl Memory for MemoryService {
    async fn allocate_memory(
        &self,
        request: tonic::Request<proto::AllocateRequest>,
    ) -> Result<tonic::Response<proto::AllocateResponse>, tonic::Status> {
        let input = request.into_inner();
        let mut mem = self.data_node.lock().await;
        let response = mem.allocate_memory(input.size as usize);

        match response {
            Ok(id) => Ok(tonic::Response::new(proto::AllocateResponse {
                result: Some(proto::allocate_response::Result::Size(id as u64)),
            })),
            Err(err) => {
                let status = match err {
                    AllocationError::InsufficientMemory => {
                        Status::new(Code::ResourceExhausted, "Out of memory")
                    }
                    AllocationError::AllocationTooLarge => {
                        Status::new(Code::InvalidArgument, "Invalid size requested")
                    }
                    _ => Status::new(Code::Internal, "Internal allocation error"),
                };
                Err(status)
            }
        }
    }

    async fn free_memory(
        &self,
        request: tonic::Request<proto::FreeRequest>,
    ) -> Result<tonic::Response<proto::FreeResponse>, tonic::Status> {
        todo!()
    }

    async fn read_memory(
        &self,
        request: tonic::Request<proto::ReadRequest>,
    ) -> Result<tonic::Response<proto::FreeResponse>, tonic::Status> {
        todo!()
    }

    async fn write_memory(
        &self,
        request: tonic::Request<proto::WriteRequest>,
    ) -> Result<tonic::Response<proto::WriteResponse>, Status> {
        todo!()
    }
}
