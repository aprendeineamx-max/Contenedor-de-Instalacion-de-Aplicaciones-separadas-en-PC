use anyhow::{Context, Result};
use std::{
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    time::Duration,
};
use tokio::time::sleep;
use tracing::{info, warn};

/// Monta el `rootfs` del contenedor usando herramientas WinFSP/Dokany.
/// La implementación actual invoca binarios externos (`winfsp-launcher.exe` o `dokanctl.exe`),
/// permitiendo intercambiar proveedores sin modificar el resto del runtime.
pub struct MountSession {
    pub mount_point: PathBuf,
    child: Option<Child>,
}

impl MountSession {
    pub async fn mount(root: impl AsRef<Path>, preferred_drive: Option<char>) -> Result<Self> {
        let root = root.as_ref();
        let mount_point = determine_mount_point(preferred_drive)?;

        if let Some(winfsp) = find_winfsp() {
            info!(?root, ?mount_point, "Montando rootfs vía WinFSP");
            let mut child = Command::new(winfsp)
                .args([
                    "--foreground",
                    "--FileSystemName",
                    "ContainerFS",
                    "--MountPoint",
                    mount_point.to_string_lossy().as_ref(),
                ])
                .arg(root)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .context("No se pudo lanzar winfsp-launcher")?;

            // winfsp tarda unos ms en crear el volumen
            sleep(Duration::from_millis(500)).await;

            return Ok(Self {
                mount_point,
                child: Some(child),
            });
        }

        if let Some(dokanctl) = find_dokany() {
            info!(?root, ?mount_point, "Montando rootfs vía Dokany");
            Command::new(&dokanctl)
                .args([
                    "/m",
                    "/r",
                    root.to_string_lossy().as_ref(),
                    "/l",
                    mount_point.to_string_lossy().as_ref(),
                ])
                .status()
                .context("No se pudo ejecutar dokanctl")?;

            return Ok(Self {
                mount_point,
                child: None,
            });
        }

        warn!("No se encontraron WinFSP ni Dokany; se continuará sin volumen montado.");
        Ok(Self {
            mount_point,
            child: None,
        })
    }
}

impl Drop for MountSession {
    fn drop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
        }
        // Dokany desmonta automáticamente al terminar el proceso.
    }
}

fn determine_mount_point(preferred: Option<char>) -> Result<PathBuf> {
    if let Some(letter) = preferred {
        return Ok(PathBuf::from(format!("{}:", letter)));
    }
    Ok(PathBuf::from(r"\\?\GLOBALROOT\device\ContainerFS"))
}

fn find_winfsp() -> Option<PathBuf> {
    std::env::var_os("WINFSP_LAUNCHER")
        .map(PathBuf::from)
        .or_else(|| which::which("winfsp-launcher.exe").ok())
}

fn find_dokany() -> Option<PathBuf> {
    std::env::var_os("DOKANCTL")
        .map(PathBuf::from)
        .or_else(|| which::which("dokanctl.exe").ok())
}
