use crate::proto::memory::{
    memory_client::MemoryClient as GrpcMemoryClient, AllocateRequest, AllocateResponse,
    AllocationError, DeallocationError, FreeRequest, FreeResponse, GetMemorySizeRequest,
    GetMemorySizeResponse, MemoryAccessError, ReadRequest, ReadResponse, WriteRequest,
    WriteResponse,
};
use tonic::{transport::Channel, Response, Status};

pub struct MemoryClient {
    client: GrpcMemoryClient<Channel>,
}

impl MemoryClient {
    pub async fn new(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::from_shared(addr)?.connect().await?;
        let client = GrpcMemoryClient::new(channel);
        Ok(Self { client })
    }

    pub async fn allocate_memory(&mut self, size: u64) -> Result<u64, AllocationError> {
        let request = AllocateRequest { size };
        let response: Response<AllocateResponse> = self
            .client
            .allocate_memory(request)
            .await
            .map_err(|e: Status| match e.code() {
                tonic::Code::InvalidArgument => AllocationError::AllocationTooLarge,
                _ => AllocationError::Unspecified,
            })?;
        match response.into_inner().result {
            Some(crate::proto::memory::allocate_response::Result::Size(size)) => Ok(size),
            Some(crate::proto::memory::allocate_response::Result::Error(error)) => {
                // convert i32 to AllocationError
                match AllocationError::from_i32(error) {
                    Some(allocation_error) => Err(allocation_error),
                    None => Err(AllocationError::Unspecified),
                }
            }
            None => Err(AllocationError::Unspecified),
        }
    }

    pub async fn free(&mut self, id: u64) -> Result<(), DeallocationError> {
        let request = FreeRequest { id };
        let response: Response<FreeResponse> =
            self.client
                .free_memory(request)
                .await
                .map_err(|e: Status| match e.code() {
                    tonic::Code::OutOfRange => DeallocationError::DeallocationInvalidMemoryAddress,
                    _ => DeallocationError::Unspecified,
                })?;

        match response.into_inner().result {
            Some(crate::proto::memory::free_response::Result::Ok(true)) => Ok(()),
            Some(crate::proto::memory::free_response::Result::Error(error)) => {
                // convert i32 to deallocation error
                match DeallocationError::from_i32(error) {
                    Some(deallocation_error) => Err(deallocation_error),
                    None => Err(DeallocationError::Unspecified),
                }
            }
            _ => Err(DeallocationError::Unspecified),
        }
    }

    pub async fn read(
        &mut self,
        id: u64,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, MemoryAccessError> {
        let request = ReadRequest { id, offset, length };
        let response: Response<ReadResponse> =
            self.client
                .read_memory(request)
                .await
                .map_err(|e: Status| match e.code() {
                    tonic::Code::NotFound => MemoryAccessError::AccessInvalidMemoryAddress,
                    tonic::Code::OutOfRange => MemoryAccessError::OutOfBoundsAccess,
                    _ => MemoryAccessError::Unspecified,
                })?;
        match response.into_inner().result {
            Some(crate::proto::memory::read_response::Result::Memory(mem)) => Ok(mem),
            Some(crate::proto::memory::read_response::Result::Error(error)) => {
                // convert i32 to read error
                match MemoryAccessError::from_i32(error) {
                    Some(read_error) => Err(read_error),
                    None => Err(MemoryAccessError::Unspecified),
                }
            }
            None => Err(MemoryAccessError::Unspecified),
        }
    }

    pub async fn write(
        &mut self,
        id: u64,
        offset: u64,
        data: Vec<u8>,
    ) -> Result<(), MemoryAccessError> {
        let request = WriteRequest { id, offset, data };
        let response: Response<WriteResponse> =
            self.client
                .write_memory(request)
                .await
                .map_err(|e: Status| match e.code() {
                    tonic::Code::NotFound => MemoryAccessError::AccessInvalidMemoryAddress,
                    tonic::Code::OutOfRange => MemoryAccessError::OutOfBoundsAccess,
                    _ => MemoryAccessError::Unspecified,
                })?;
        match response.into_inner().result {
            Some(crate::proto::memory::write_response::Result::Ok(true)) => Ok(()),
            Some(crate::proto::memory::write_response::Result::Error(error)) => {
                // convert i32 to write error
                match MemoryAccessError::from_i32(error) {
                    Some(write_error) => Err(write_error),
                    None => Err(MemoryAccessError::Unspecified),
                }
            }
            _ => Err(MemoryAccessError::Unspecified),
        }
    }

    pub async fn get_memory_size(&mut self, id: u64) -> Result<u64, MemoryAccessError> {
        let request = GetMemorySizeRequest { id };
        let response: Response<GetMemorySizeResponse> = self
            .client
            .get_memory_size(request)
            .await
            .map_err(|e: Status| match e.code() {
                tonic::Code::NotFound => MemoryAccessError::AccessInvalidMemoryAddress,
                _ => MemoryAccessError::Unspecified,
            })?;

        match response.into_inner().result {
            Some(crate::proto::memory::get_memory_size_response::Result::Size(size)) => Ok(size),
            Some(crate::proto::memory::get_memory_size_response::Result::Error(error)) => {
                match MemoryAccessError::from_i32(error) {
                    Some(size_error) => Err(size_error),
                    None => Err(MemoryAccessError::Unspecified),
                }
            }
            _ => Err(MemoryAccessError::Unspecified),
        }
    }
}
