use crate::registry::{ContainerManifest, RegisteredContainer};
use anyhow::Result;
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf};
use tokio::fs;
use tracing::info;

#[derive(Debug, Serialize, Clone)]
pub struct MountPlan {
    pub alias: String,
    pub host_path: PathBuf,
}

#[derive(Debug, Serialize, Clone)]
pub struct HookPlan {
    pub env: HashMap<String, String>,
    pub mounts: Vec<MountPlan>,
}

pub struct HookEngine;

impl HookEngine {
    pub fn new() -> Self {
        Self
    }

    pub async fn prepare(&self, container: &RegisteredContainer) -> Result<HookPlan> {
        let layout = PathLayout::from_manifest(&container.manifest, &container.root);
        layout.ensure_directories().await?;

        let env = HashMap::from([
            (
                "CONTAINER_ROOT".into(),
                container.root.to_string_lossy().into_owned(),
            ),
            (
                "APPDATA".into(),
                layout.appdata.to_string_lossy().into_owned(),
            ),
            (
                "LOCALAPPDATA".into(),
                layout.local_appdata.to_string_lossy().into_owned(),
            ),
            (
                "PROGRAMFILES".into(),
                layout.program_files.to_string_lossy().into_owned(),
            ),
            ("TEMP".into(), layout.temp.to_string_lossy().into_owned()),
            ("TMP".into(), layout.temp.to_string_lossy().into_owned()),
        ]);

        let mounts = vec![
            MountPlan {
                alias: "%APPDATA%".into(),
                host_path: layout.appdata.clone(),
            },
            MountPlan {
                alias: "%LOCALAPPDATA%".into(),
                host_path: layout.local_appdata.clone(),
            },
            MountPlan {
                alias: "%PROGRAMFILES%".into(),
                host_path: layout.program_files.clone(),
            },
            MountPlan {
                alias: "%TEMP%".into(),
                host_path: layout.temp.clone(),
            },
        ];

        info!(
            container_id = container.manifest.id.as_str(),
            plan = ?mounts,
            env_keys = ?env.keys().collect::<Vec<_>>(),
            "Plan de hooks preparado"
        );

        Ok(HookPlan { env, mounts })
    }
}

struct PathLayout {
    program_files: PathBuf,
    appdata: PathBuf,
    local_appdata: PathBuf,
    temp: PathBuf,
}

impl PathLayout {
    fn from_manifest(manifest: &ContainerManifest, root: &PathBuf) -> Self {
        Self {
            program_files: resolve_path(
                root,
                manifest.paths.program_files.as_ref(),
                "rootfs/ProgramFiles",
            ),
            appdata: resolve_path(
                root,
                manifest.paths.appdata.as_ref(),
                "user/AppData/Roaming",
            ),
            local_appdata: resolve_path(
                root,
                manifest.paths.local_appdata.as_ref(),
                "user/LocalAppData",
            ),
            temp: resolve_path(root, manifest.paths.temp.as_ref(), "temp"),
        }
    }

    async fn ensure_directories(&self) -> Result<()> {
        for dir in [
            &self.program_files,
            &self.appdata,
            &self.local_appdata,
            &self.temp,
        ] {
            fs::create_dir_all(dir).await?;
        }
        Ok(())
    }
}

fn resolve_path(root: &PathBuf, value: Option<&PathBuf>, default: &str) -> PathBuf {
    match value {
        Some(val) => root.join(val),
        None => root.join(default),
    }
}
