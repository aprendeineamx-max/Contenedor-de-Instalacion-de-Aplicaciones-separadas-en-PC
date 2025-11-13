# Estrategia de Pruebas

## 1. Pirámide
- **Unitarias (Rust/TypeScript)**: hooks del agent, parsers de manifiestos, servicios Axum y componentes React.
- **Integración**: agent con contenedores dummy, redirección de filesystem/registro, stores SQLx/Redis.
- **End-to-End**: CLI y panel ejecutando flujos reales (instalación, snapshot, exportación).

## 2. Suites actuales
1. **Runtime Hooks**  
   - Verificar que `%APPDATA%`, `%LOCALAPPDATA%` y `%TEMP%` apuntan al layout del contenedor.  
   - Medir overhead de montajes WinFSP/Dokany.
2. **Installer Capture**  
   - Ejecutar instaladores MSI/EXE en sandbox y comparar outputs con el layout esperado.  
   - Validar manifiestos y launchers autogenerados.
3. **API/Backend**  
   - Contratos REST/gRPC (`cargo test -p backend`) con migraciones SQLx y soporte SQLite/Postgres.  
   - Escenarios de paginación, filtros y eliminación.  
   - RPCs `List/Create/Get/Delete` cubiertos con `tonic`.
4. **Frontend e2e**  
   - Playwright (`npm run test:e2e`) con `webServer` que arranca Next.js y valida flujo “Dashboard lista vacía / contenedores existentes”.
5. **CLI**  
   - Tests contra backend mock Axum (`cargo test -p ctnr-cli`) para garantizar wiring y manejo de errores.

## 3. Automatización
- GitHub Actions: jobs independientes para agent, backend, frontend y CLI.
- Ambientes Windows Server efímeros para validar hooks nativos y WinFSP.
- Publicación de reportes y artefactos `.ctnr` en cada pipeline.

## 4. Cobertura y métricas
- Objetivo: 70 % backend/agent, 60 % frontend.
- Benchmarks periódicos para comparar overhead del runtime vs ejecución nativa.

## 5. Próximos pasos
1. Integrar `cargo-nextest`, `vitest` y Playwright en CI.
2. Crear contenedores de referencia para pruebas automatizadas.
3. Añadir suites e2e completas (crear contenedor, snapshot, exportación) antes de GA.

