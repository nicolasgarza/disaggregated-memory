use crate::errors::{AllocationError, DeallocationError, MemoryAccessError};

use crate::memory::DataNode;
use tonic::{Request, Response, Status};

use crate::proto::memory_service::{
    AllocateRequest, AllocateResponse, FreeRequest, FreeResponse, ReadRequest, ReadResponse,
    WriteRequest, WriteResponse,
};

pub struct MemoryServiceImpl {
    data_node: DataNode,
}

#[tonic::async_trait]
impl MemoryService for MemoryServiceImpl {
    async fn allocate_memory(
        &self,
        request: Request<AllocateRequest>,
    ) -> Result<Response<AllocateResponse>, Status> {
        //TODO
    }

    async fn free_memory(
        &self,
        request: Request<FreeRequest>,
    ) -> Result<Response<FreeResponse>, Status> {
        //TODO
    }

    async fn read_memory(
        &self,
        request: Request<ReadRequest>,
    ) -> Result<Response<ReadResponse>, Status> {
        //TODO
    }

    async fn write_memory(
        &self,
        request: Request<WriteRequest>,
    ) -> Result<Response<WriteResponse>, Status> {
        //TODO
    }
}
