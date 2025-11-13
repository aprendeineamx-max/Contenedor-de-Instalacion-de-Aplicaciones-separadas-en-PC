use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use backend::{
    app::{build_router, AppState},
    grpc::ContainerGrpc,
    proto::{
        container_service_server::ContainerService, CreateContainerRequest, DeleteContainerRequest,
        ListContainersRequest,
    },
    store::Store,
};
use http_body_util::BodyExt;
use once_cell::sync::OnceCell;
use serde_json::json;
use tonic::Request as GrpcRequest;
use tower::ServiceExt;

static TRACING: OnceCell<()> = OnceCell::new();

fn init_tracing() {
    TRACING.get_or_init(|| {
        let _ = tracing_subscriber::fmt().with_test_writer().try_init();
    });
}

async fn test_store() -> Store {
    init_tracing();
    Store::open("sqlite::memory:?cache=shared").await.unwrap()
}

#[tokio::test]
async fn rest_create_and_list_with_filters() {
    let store = test_store().await;
    let state = AppState::new("test".into(), store.clone(), None);
    let app = build_router(state);

    let body = json!({ "name": "demo", "version": "1.0" }).to_string();
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/containers")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/containers?status=draft&limit=5&offset=0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<backend::store::ContainerRecord> = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].name, "demo");
}

#[tokio::test]
async fn grpc_create_and_list() {
    let store = test_store().await;
    let grpc = ContainerGrpc::new(store);

    let request = CreateContainerRequest {
        name: "grpc-demo".into(),
        version: "2.0".into(),
    };
    grpc.create_container(GrpcRequest::new(request))
        .await
        .unwrap();

    let response = grpc
        .list_containers(GrpcRequest::new(ListContainersRequest {}))
        .await
        .unwrap();
    assert_eq!(response.get_ref().containers.len(), 1);
    assert_eq!(response.get_ref().containers[0].name, "grpc-demo");
}

#[tokio::test]
async fn rest_get_and_delete_flow() {
    let store = test_store().await;
    let state = AppState::new("test".into(), store.clone(), None);
    let app = build_router(state);

    // Create container
    let body = json!({ "name": "removable" }).to_string();
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/containers")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let created: backend::store::ContainerRecord =
        serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes()).unwrap();

    // GET should succeed
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/containers/{}", created.id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // DELETE
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/containers/{}", created.id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // GET after delete should 404
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/containers/{}", created.id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn grpc_delete_container() {
    let store = test_store().await;
    let service = ContainerGrpc::new(store.clone());

    let request = CreateContainerRequest {
        name: "grpc-delete".into(),
        version: "".into(),
    };
    let response = service
        .create_container(GrpcRequest::new(request))
        .await
        .unwrap();
    let id = response.get_ref().container.as_ref().unwrap().id.clone();

    let delete_res = service
        .delete_container(GrpcRequest::new(DeleteContainerRequest { id: id.clone() }))
        .await
        .unwrap();
    assert_eq!(delete_res.get_ref().id, id);

    let list_res = service
        .list_containers(GrpcRequest::new(ListContainersRequest {}))
        .await
        .unwrap();
    assert!(list_res.get_ref().containers.is_empty());
}
