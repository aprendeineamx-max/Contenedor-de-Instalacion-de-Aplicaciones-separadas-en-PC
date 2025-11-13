# Estrategia de Pruebas

## 1. Pirámide
- **Unitarias (Rust/TypeScript)**: hooks del agent, parsers de manifiestos, servicios Axum y componentes React.
- **Integración**: agent con contenedores dummy, montajes WinFSP/Dokany, stores SQLx/Redis y workers.
- **End-to-End**: CLI y panel ejecutando flujos reales (creación, instalación, exportación).

## 2. Suites actuales
1. **Runtime Hooks**  
   - Validar que `%APPDATA%`, `%LOCALAPPDATA%` y `%TEMP%` apuntan al layout del contenedor.  
   - Medir overhead de montajes WinFSP/Dokany.
2. **Installer Capture**  
   - Ejecutar instaladores en sandbox (PowerShell + hooks) y comparar outputs con el layout esperado.  
   - Generar manifiestos/launchers automáticamente.
3. **API/Backend**  
   - `cargo test -p backend`: REST/gRPC, migraciones SQLx y SQLite/Postgres.  
   - Cobertura de filtros/paginación, SSE (`/api/events/containers`) y colas Redis.  
   - RPCs `List/Create/Get/Delete` mediante `tonic`.
4. **Workers/Queues**  
   - `cargo run -p backend --bin worker` + Redis para simular tareas `containers:create` y registrar resultados.  
5. **Frontend e2e**  
   - Playwright (`npm run test:e2e`) con Next.js en vivo y backend real cuando está disponible.
6. **CLI**  
   - Tests contra backend mock Axum (`cargo test -p ctnr-cli`).

## 3. Automatización
- GitHub Actions: jobs independientes para agent, backend, frontend, CLI y worker.
- Ambientes Windows Server efímeros para validar hooks nativos.
- Publicación de reportes (`trace.zip`, cobertura) y artefactos `.ctnr`.

## 4. Cobertura y métricas
- Objetivo inicial: 70 % backend/agent, 60 % frontend.
- Benchmarks periódicos para comparar overhead del runtime vs ejecución nativa.
- Métricas Prometheus y logs estructurados (pendiente de automatizar en CI).

## 5. Próximos pasos
1. Integrar `cargo-nextest`, `vitest` y Playwright en CI pipeline.  
2. Crear contenedores de referencia para reutilizar en pruebas.  
3. Añadir suites e2e completas (crear → capturar instalador → ejecutar) antes del GA.  
4. Automatizar pruebas de hooks WinFSP/Dokany con VMs Windows Server.

