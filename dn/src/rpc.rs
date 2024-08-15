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
                };
                Err(status)
            }
        }
    }

    async fn free_memory(
        &self,
        request: tonic::Request<proto::FreeRequest>,
    ) -> Result<tonic::Response<proto::FreeResponse>, tonic::Status> {
        let input = request.into_inner();
        let mut mem = self.data_node.lock().await;
        let response = mem.free_memory(input.id as usize);

        match response {
            Ok(_) => Ok(tonic::Response::new(proto::FreeResponse {
                result: Some(proto::free_response::Result::Ok(true)),
            })),
            Err(err) => {
                let status = match err {
                    DeallocationError::InvalidMemoryAddress => {
                        Status::new(Code::OutOfRange, "Invalid memory access")
                    }
                };
                Err(status)
            }
        }
    }

    async fn read_memory(
        &self,
        request: tonic::Request<proto::ReadRequest>,
    ) -> Result<tonic::Response<proto::ReadResponse>, tonic::Status> {
        let input = request.into_inner();
        let mem = self.data_node.lock().await;
        let response = mem.read_memory(
            input.id as usize,
            input.offset as usize,
            input.length as usize,
        );

        match response {
            Ok(bytes) => Ok(tonic::Response::new(proto::ReadResponse {
                result: Some(proto::read_response::Result::Memory(bytes.to_vec())),
            })),
            Err(err) => {
                let status = match err {
                    MemoryAccessError::InvalidMemoryAddress => {
                        Status::new(Code::NotFound, "Invalid memory access")
                    }
                    MemoryAccessError::OutOfBoundsAccess => {
                        Status::new(Code::OutOfRange, "Out of bounds access")
                    }
                };
                Err(status)
            }
        }
    }

    async fn write_memory(
        &self,
        request: tonic::Request<proto::WriteRequest>,
    ) -> Result<tonic::Response<proto::WriteResponse>, Status> {
        let input = request.into_inner();
        let mut mem = self.data_node.lock().await;
        let response = mem.write_memory(input.id as usize, input.offset as usize, &input.data);

        match response {
            Ok(_) => Ok(tonic::Response::new(proto::WriteResponse {
                result: Some(proto::write_response::Result::Ok(true)),
            })),
            Err(err) => {
                let status = match err {
                    MemoryAccessError::InvalidMemoryAddress => {
                        Status::new(Code::NotFound, "Invalid memory access")
                    }
                    MemoryAccessError::OutOfBoundsAccess => {
                        Status::new(Code::OutOfRange, "Out of bounds access")
                    }
                };
                Err(status)
            }
        }
    }
}
