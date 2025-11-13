use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContainerRecord {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
    pub status: String,
}

#[derive(Clone, Default)]
pub struct Store {
    inner: Arc<RwLock<Vec<ContainerRecord>>>,
}

impl Store {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn list(&self) -> Vec<ContainerRecord> {
        self.inner.read().await.clone()
    }

    pub async fn create(&self, name: &str, version: Option<String>) -> ContainerRecord {
        let record = ContainerRecord {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            version,
            status: "draft".into(),
        };

        self.inner.write().await.push(record.clone());
        record
    }

    pub async fn get(&self, id: &str) -> Option<ContainerRecord> {
        self.inner.read().await.iter().find(|c| c.id == id).cloned()
    }

    pub async fn delete(&self, id: &str) -> Option<ContainerRecord> {
        let mut guard = self.inner.write().await;
        if let Some(pos) = guard.iter().position(|c| c.id == id) {
            Some(guard.remove(pos))
        } else {
            None
        }
    }
}
