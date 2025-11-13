use crate::proto::container_service_server::{
    ContainerService, ContainerServiceServer,
};
use crate::proto::{
    Container, CreateContainerRequest, CreateContainerResponse, DeleteContainerRequest,
    DeleteContainerResponse, GetContainerRequest, GetContainerResponse, ListContainersRequest,
    ListContainersResponse,
};
use crate::store::{ContainerRecord, Store};
use anyhow::Result;
use std::net::SocketAddr;
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct ContainerGrpc {
    store: Store,
}

impl ContainerGrpc {
    pub fn new(store: Store) -> Self {
        Self { store }
    }

    pub async fn serve(self, addr: SocketAddr) -> Result<()> {
        tonic::transport::Server::builder()
            .add_service(ContainerServiceServer::new(self))
            .serve(addr)
            .await?;
        Ok(())
    }
}

#[tonic::async_trait]
impl ContainerService for ContainerGrpc {
    async fn list_containers(
        &self,
        _request: Request<ListContainersRequest>,
    ) -> Result<Response<ListContainersResponse>, Status> {
        let records = self.store.list().await;
        let containers = records.into_iter().map(Container::from).collect();
        Ok(Response::new(ListContainersResponse { containers }))
    }

    async fn create_container(
        &self,
        request: Request<CreateContainerRequest>,
    ) -> Result<Response<CreateContainerResponse>, Status> {
        let payload = request.into_inner();
        if payload.name.trim().is_empty() {
            return Err(Status::invalid_argument("name is required"));
        }
        let record = self
            .store
            .create(&payload.name, no_empty(payload.version))
            .await;
        Ok(Response::new(CreateContainerResponse {
            container: Some(record.into()),
        }))
    }

    async fn get_container(
        &self,
        request: Request<GetContainerRequest>,
    ) -> Result<Response<GetContainerResponse>, Status> {
        let id = request.into_inner().id;
        match self.store.get(&id).await {
            Some(record) => Ok(Response::new(GetContainerResponse {
                container: Some(record.into()),
            })),
            None => Err(Status::not_found("container not found")),
        }
    }

    async fn delete_container(
        &self,
        request: Request<DeleteContainerRequest>,
    ) -> Result<Response<DeleteContainerResponse>, Status> {
        let id = request.into_inner().id;
        self.store
            .delete(&id)
            .await
            .ok_or_else(|| Status::not_found("container not found"))?;
        Ok(Response::new(DeleteContainerResponse { id }))
    }
}

impl From<ContainerRecord> for Container {
    fn from(value: ContainerRecord) -> Self {
        Container {
            id: value.id,
            name: value.name,
            version: value.version.unwrap_or_default(),
            status: value.status,
        }
    }
}

fn no_empty(input: String) -> Option<String> {
    if input.trim().is_empty() {
        None
    } else {
        Some(input)
    }
}
