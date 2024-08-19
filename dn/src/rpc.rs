use crate::errors::{AllocationError, DeallocationError, MemoryAccessError};
use crate::proto::memory;

use crate::memory::DataNode;
use std::sync::Arc;
use tokio::sync::Mutex;

use tonic::{Code, Status};

pub struct MemoryService {
    data_node: Arc<Mutex<DataNode>>,
}

impl MemoryService {
    pub fn new(data_node: DataNode) -> Self {
        MemoryService {
            data_node: Arc::new(Mutex::new(data_node)),
        }
    }
}

#[tonic::async_trait]
impl memory::memory_server::Memory for MemoryService {
    async fn allocate_memory(
        &self,
        request: tonic::Request<memory::AllocateRequest>,
    ) -> Result<tonic::Response<memory::AllocateResponse>, tonic::Status> {
        let input = request.into_inner();
        let mut mem = self.data_node.lock().await;
        let response = mem.allocate_memory(input.size as usize);

        match response {
            Ok(id) => Ok(tonic::Response::new(memory::AllocateResponse {
                result: Some(memory::allocate_response::Result::Size(id as u64)),
            })),
            Err(err) => {
                let status = match err {
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
        request: tonic::Request<memory::FreeRequest>,
    ) -> Result<tonic::Response<memory::FreeResponse>, tonic::Status> {
        let input = request.into_inner();
        let mut mem = self.data_node.lock().await;
        let response = mem.free_memory(input.id as usize);

        match response {
            Ok(_) => Ok(tonic::Response::new(memory::FreeResponse {
                result: Some(memory::free_response::Result::Ok(true)),
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
        request: tonic::Request<memory::ReadRequest>,
    ) -> Result<tonic::Response<memory::ReadResponse>, tonic::Status> {
        let input = request.into_inner();
        let mem = self.data_node.lock().await;
        let response = mem.read_memory(
            input.id as usize,
            input.offset as usize,
            input.length as usize,
        );

        match response {
            Ok(bytes) => Ok(tonic::Response::new(memory::ReadResponse {
                result: Some(memory::read_response::Result::Memory(bytes.to_vec())),
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
        request: tonic::Request<memory::WriteRequest>,
    ) -> Result<tonic::Response<memory::WriteResponse>, Status> {
        let input = request.into_inner();
        let mut mem = self.data_node.lock().await;
        let response = mem.write_memory(input.id as usize, input.offset as usize, &input.data);

        match response {
            Ok(_) => Ok(tonic::Response::new(memory::WriteResponse {
                result: Some(memory::write_response::Result::Ok(true)),
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

    async fn get_memory_size(
        &self,
        request: tonic::Request<memory::GetMemorySizeRequest>,
    ) -> Result<tonic::Response<memory::GetMemorySizeResponse>, Status> {
        let input = request.into_inner();
        let mem = self.data_node.lock().await;
        let response = mem.get_memory_size(input.id as usize);

        match response {
            Ok(size) => Ok(tonic::Response::new(memory::GetMemorySizeResponse {
                result: Some(memory::get_memory_size_response::Result::Size(size as u64)),
            })),
            Err(_) => Err(Status::new(Code::NotFound, "Invalid memory access")),
        }
    }
}
