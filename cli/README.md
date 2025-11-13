# CLI

Herramienta en Rust (clap) que reproduce las capacidades del panel:
- `ctnr create`, `ctnr install`, `ctnr run`, `ctnr snapshot`, `ctnr export`.
- Autenticación contra el backend (tokens API/OIDC).
- Modo offline para interactuar directamente con el agent en la misma máquina.

## Roadmap
- Scaffold básico con comandos stub y documentación de uso.
- Integración gRPC/REST.
- Tests end-to-end que validen operaciones completas sobre contenedores de ejemplo.

