use anyhow::Result;
use tracing::info;

/// Administra servicios instalados dentro del contenedor.
/// En el futuro interceptaremos el SCM para registrar servicios aislados.
pub struct ServiceSandbox;

impl ServiceSandbox {
    pub fn new() -> Self {
        Self
    }

    pub fn register_placeholder(&self, name: &str) -> Result<()> {
        info!(
            service = name,
            "Registrando servicio placeholder (pendiente de implementación)"
        );
        Ok(())
    }
}
