# Sistema de Contenedores Win32

Plataforma para instalar y ejecutar aplicaciones Win32 dentro de contenedores portables, reutilizando binarios, datos de usuario y configuración sin tocar el host.

## Módulos principales
- `agent/`: servicio Windows que prepara planes de montaje, aplica hooks (Detours/WinFSP/Dokany) y lanza los procesos.
- `backend/`: plano de control (Rust + Axum/Tonic + SQLx) con APIs REST/gRPC, Postgres por defecto y colas Redis.
- `frontend/`: panel Next.js 14 con formularios de creación, SSE en tiempo real y pruebas Playwright.
- `cli/`: herramienta Rust para automatizar operaciones (`ctnr list/create/export`).
- `docs/`: especificaciones de contenedores, APIs y guía de hooks (`docs/spec.md`, `docs/api.md`, `docs/hooks.md`).
- `installer/`: scripts y documentación inicial para capturar instaladores dentro del contenedor.

## Requisitos
- Rust 1.79+ (`rustup default stable`).
- Node.js 20+ y `npm`.
- PostgreSQL 14+ (`DATABASE_URL=postgres://containers:containers@localhost:5432/containers`).
- Redis 7+ (`REDIS_URL=redis://localhost:6379`) para las colas de instalación.
- WinFSP 2.x o Dokany 2.x + Microsoft Detours para los hooks nativos del agent.

## Variables de entorno clave
| Variable | Descripción | Valor por defecto |
| -------- | ----------- | ----------------- |
| `DATABASE_URL` | URL SQLx (`postgres://…` o `sqlite://…`) | `postgres://containers:containers@localhost:5432/containers` |
| `REDIS_URL` | Cola para tareas async | _vacío_ |
| `CONTAINERS_HTTP_ADDR` | Dirección HTTP del backend | `0.0.0.0:8080` |
| `CONTAINERS_GRPC_ADDR` | Dirección gRPC | `0.0.0.0:50051` |
| `CONTAINERS_API_KEY` | API Key mínima para REST | _vacío_ |
| `NEXT_PUBLIC_API_BASE` | Endpoint usado por el panel | `http://127.0.0.1:8080` |

## Ejecución rápida
```bash
# Base de datos + Redis (Docker)
docker compose up -d postgres redis

# Backend (REST/gRPC + migraciones automáticas)
cargo run -p backend

# Worker de colas
cargo run -p backend --bin worker

# Agent (hooks nativos opcionales)
cargo run -p agent --features native-hooks

# CLI
cargo run -p ctnr-cli -- list

# Frontend
cd frontend
npm install
npm run dev

# Playwright e2e (usa Next.js y backend real si está disponible)
npm run test:e2e
```

## APIs y características
- **REST** (`docs/api.md`): `GET/POST/DELETE /api/containers`, `GET /api/containers/:id`, `/healthz`, `GET /api/events/containers` (SSE).
- **gRPC** (`proto/containers.proto`): `containers.v1.ContainerService`.
- **Hooks** (`docs/hooks.md`): planes de montaje (`MountPlan`), redirecciones (`PathRedirect`), hook `CreateFileW` mediante Detours y montaje WinFSP/Dokany.
- **Colas Redis**: worker (`backend/src/bin/worker.rs`) escucha `containers:create` y procesará capturas/instalaciones.
- **Seguridad**: API Key mínima (`X-API-Key`), rate limiting y trazas HTTP.

## Pruebas
- `cargo test -p backend` – REST/gRPC + migraciones SQLx + Redis stubs.
- `cargo test -p ctnr-cli` – CLI contra servidor mock Axum.
- `cargo test -p agent` – validaciones del runtime/manifest parsing.
- `npm run test:e2e` – Playwright (Chromium) levantando Next.js; intenta usar el backend real y cae a mocks si no está disponible.

## Installer Capture (primer borrador)
- Documentado en `installer/README.md`.
- Script `installer/scripts/capture.ps1` encapsula el instalador dentro de un sandbox supervisado.
- El backend expone colas Redis para programar capturas y el panel permitirá subir el instalador (en progreso).

## Estado actual
- ✅ Persistencia Postgres por defecto, SQLite para tests, migraciones versionadas.
- ✅ HookEngine genera planes, aplica Detours y controla montajes WinFSP/Dokany.
- ✅ Panel con formularios reales, SSE y pruebas Playwright.
- ✅ Worker Redis para futuras tareas de instalación/captura.
- 🚧 Próximos pasos: pipeline completo de captura, UI de colas, autenticación OIDC, empaquetado MSI/installer y drivers WinFSP/Dokany firmados.

