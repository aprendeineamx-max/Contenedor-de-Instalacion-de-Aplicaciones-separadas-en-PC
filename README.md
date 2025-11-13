# Sistema de Contenedores Win32

Este repositorio alberga el desarrollo del sistema que permite instalar y ejecutar aplicaciones Win32 dentro de contenedores portables, replicando instalaciones completas sin afectar el host.

## Módulos Principales
- `agent/`: servicio residente en Rust que orquesta contenedores, coordina hooks y expone APIs locales.
- `backend/`: plano de control (Rust + Axum/Tonic) con lógica de negocio, RBAC y colas de tareas.
- `frontend/`: panel web Next.js 14 (TypeScript) para administrar contenedores desde el navegador.
- `cli/`: herramienta de línea de comandos (Rust) para automatizar la plataforma desde scripts/CI.
- `docs/`: especificaciones, diagramas y material de diseño.
- `infrastructure/`: definiciones IaC, manifests de despliegue y scripts de instalación.

## Flujo de Trabajo Inicial
1. Diseñar los contratos entre módulos (gRPC/REST) y el esquema de base de datos (SQLite por defecto).
2. Implementar el runtime/agent mínimo que redirige rutas críticas de usuario.
3. Levantar backend + frontend con datos simulados para iterar en la experiencia de usuario.
4. Desarrollar el pipeline de captura de instaladores y exportación `.ctnr`.

## Cómo ejecutar los scaffolds
### Prerrequisitos
- Rust 1.79+ y `cargo`.
- Node.js 20+ y `npm`.

Para instalar rápidamente en Windows:
- `winget install Rustlang.Rustup` y luego `rustup default stable`.
- `winget install OpenJS.NodeJS.LTS` (o instala la versión requerida desde nodejs.org).

### Pasos
```bash
# Backend (API con SQLite + gRPC)
cargo run -p backend

# Agent (placeholder runtime)
cargo run -p agent

# CLI
cargo run -p ctnr-cli -- list

# Frontend
cd frontend
npm install
npm run dev
```

> El backend expone `/healthz` y `/api/containers` sobre SQLite. La CLI ya consume esos endpoints para validar el wiring inicial.

## APIs disponibles
- REST (`docs/api.md`): `GET/POST/DELETE /api/containers` + `/healthz` en `http://localhost:8080`.
- gRPC (`proto/containers.proto`): Servicio `containers.v1.ContainerService` en `0.0.0.0:50051`, pensado para comunicación agent <-> backend.

## Pruebas
- Backend (REST + gRPC + SQLite): `cargo test -p backend`
- CLI (contra servidor mock en memoria): `cargo test -p ctnr-cli`

## Estado Actual
- ✅ Especificación técnica inicial en `docs/spec.md`.
- 🚧 Estructura base de carpetas y documentación.
- ⏳ Próximos pasos: definir contratos API, preparar plantillas de proyectos y configurar toolchains (Rust, Node.js, etc.).

## Contacto y Soporte
Las discusiones iniciales y issues pueden abrirse directamente en este repositorio. Posteriormente se migrarán a un portal público con documentación completa.
