use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use backend::{
    app::{build_router, AppState},
    grpc::ContainerGrpc,
    proto::{
        container_service_server::ContainerService, CreateContainerRequest, ListContainersRequest,
    },
    store::Store,
};
use http_body_util::BodyExt;
use once_cell::sync::OnceCell;
use serde_json::json;
use tempfile::tempdir;
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
    let tmp = tempdir().unwrap();
    Store::new(tmp.path().join("containers.db")).await.unwrap()
}

#[tokio::test]
async fn rest_create_and_list_with_filters() {
    let store = test_store().await;
    let state = AppState::new("test".into(), store.clone());
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
