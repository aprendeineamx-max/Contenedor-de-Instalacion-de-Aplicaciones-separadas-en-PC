# Sistema de Contenedores Win32

Plataforma para instalar y ejecutar aplicaciones Win32 dentro de contenedores portables, conservando binarios, datos de usuario y configuración aislada.

## Módulos principales
- `agent/`: servicio local que orquesta contenedores, aplica hooks (Detours/WinFSP) y prepara el runtime.
- `backend/`: plano de control (Rust + Axum/Tonic + SQLx) con APIs REST/gRPC y soporte para SQLite/PostgreSQL + Redis.
- `frontend/`: panel Next.js 14 responsivo con soporte para pruebas e2e (Playwright).
- `cli/`: herramienta Rust para automatizar operaciones (`ctnr create`, `ctnr list`, etc.).
- `docs/`: especificaciones funcionales, APIs y guías de runtime.
- `infrastructure/`: scripts e IaC (en preparación).

## Requisitos
- Rust 1.79+ (`rustup default stable` recomendado).
- Node.js 20+ y `npm`.
- (Opcional) Redis 7+ para colas asincrónicas (`REDIS_URL`).
- (Opcional) PostgreSQL 14+ (`DATABASE_URL=postgres://...`). Por defecto se usa `sqlite://data/containers.db`.

## Variables de entorno clave
| Variable | Descripción | Valor por defecto |
| -------- | ----------- | ----------------- |
| `DATABASE_URL` | URL SQLx (`sqlite://…`, `postgres://…`) | `sqlite://data/containers.db` |
| `REDIS_URL` | Conexión para tareas asincrónicas | _vacío_ |
| `CONTAINERS_HTTP_ADDR` | Host/puerto HTTP | `0.0.0.0:8080` |
| `CONTAINERS_GRPC_ADDR` | Host/puerto gRPC | `0.0.0.0:50051` |

## Ejecución rápida
```bash
# Backend (API REST/gRPC + SQLx + migraciones automáticas)
cargo run -p backend

# Agent (prepara planes de montaje y aplica hooks nativos con --features native-hooks)
cargo run -p agent --features native-hooks   # requiere Windows + Detours + WinFSP

# CLI
cargo run -p ctnr-cli -- list

# Frontend
cd frontend
npm install
npm run dev

# Pruebas e2e del panel (lanza Next.js automáticamente)
npm run test:e2e
```

## APIs disponibles
- REST (`docs/api.md`): `GET/POST/DELETE /api/containers`, `GET /api/containers/:id`, `/healthz`.
- gRPC (`proto/containers.proto`): servicio `containers.v1.ContainerService` (puerto `50051`) usado por agentes remotos.

## Hooks nativos
- El plan de hooks (`HookPlan`) contiene variables de entorno, montajes y redirecciones de rutas.
- En Windows, habilita `cargo run -p agent --features native-hooks` para activar el hook `CreateFileW` mediante Detours (ver `agent/src/hooks`).
- WinFSP/Dokany pueden montarse usando los `MountPlan` generados; consulta `docs/hooks.md` para flujo completo.

## Pruebas
- Backend (REST + gRPC + migraciones SQLx): `cargo test -p backend`.
- CLI (mock server Axum): `cargo test -p ctnr-cli`.
- Frontend e2e (Playwright, arranca Next.js automáticamente): `npm run test:e2e`.

## Estado
- ✅ Especificación técnica (`docs/spec.md`), roadmap (`docs/roadmap.md`) y estrategia de pruebas (`TESTING.md`).
- ✅ Persistencia multi motor (SQLite/Postgres) y Redis opcional para colas.
- ✅ HookEngine genera planes de montaje/redirección y activa Detours cuando está disponible.
- 🚧 Siguiente etapa: drivers WinFSP/Dokany, UI avanzada, captura automática de instaladores y despliegues empaquetados.

