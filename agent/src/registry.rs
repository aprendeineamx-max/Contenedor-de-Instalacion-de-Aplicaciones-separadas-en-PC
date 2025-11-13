use anyhow::{Context, Result};
use serde::Deserialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tokio::fs;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RuntimeConfig {
    pub build: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PathConfig {
    pub program_files: Option<PathBuf>,
    pub appdata: Option<PathBuf>,
    pub local_appdata: Option<PathBuf>,
    pub temp: Option<PathBuf>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContainerManifest {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
    #[serde(default)]
    pub runtime: RuntimeConfig,
    #[serde(default)]
    pub paths: PathConfig,
}

#[derive(Default, Clone)]
pub struct ContainerRegistry {
    containers: HashMap<String, ContainerManifest>,
}

impl ContainerRegistry {
    pub async fn load_from(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        if !fs::try_exists(&root).await? {
            return Ok(Self::default());
        }

        let mut entries = fs::read_dir(&root).await?;
        let mut containers = HashMap::new();

        while let Some(entry) = entries.next_entry().await? {
            if !entry.file_type().await?.is_dir() {
                continue;
            }

            let manifest_path = entry.path().join("config.yml");
            if !fs::try_exists(&manifest_path).await? {
                continue;
            }

            let manifest_content = fs::read_to_string(&manifest_path).await?;
            let manifest: ContainerManifest = serde_yaml::from_str(&manifest_content)
                .with_context(|| format!("Manifiesto inválido en {}", manifest_path.display()))?;

            containers.insert(manifest.id.clone(), manifest);
        }

        Ok(Self { containers })
    }

    pub fn list(&self) -> Vec<&ContainerManifest> {
        self.containers.values().collect()
    }
}
