# Estrategia de Pruebas

## 1. Pirámide de tests
- **Unitarias (Rust/TypeScript)**: validan hooks del agent, parsers de manifiestos, servicios Axum y componentes React.
- **Integración**: escenarios que lanzan el agent con contenedores dummy y verifican la redirección de rutas/registro, además de stores SQLite.
- **End-to-End**: CLI o panel web creando/instalando aplicaciones reales (Notepad++, 7zip, navegadores).

## 2. Suites iniciales
1. **Runtime Hooks**  
   - Verificar que las rutas `%APPDATA%`, `%LOCALAPPDATA%` y `%TEMP%` apuntan al layout del contenedor.  
   - Medir overhead del montaje virtual.
2. **Installer Capture**  
   - Ejecutar instaladores MSI/EXE dentro del sandbox y comparar archivos esperados vs capturados.  
   - Validar manifiestos y launchers generados.
3. **API/Backend**  
   - Contratos REST/gRPC con pruebas contractuales sobre SQLite (ver `cargo test -p backend`).  
   - Simular concurrencia en creación/ejecución de contenedores y validar filtros/paginación.  
   - Tests para los RPC (`ListContainers`, `CreateContainer`, etc.) usando `tonic`.
4. **Frontend e2e**  
   - Playwright/Cypress para flujos clave (crear contenedor, lanzar app, exportar) consumiendo la API real.
5. **CLI**  
   - Tests contra servidores mock (ver `cargo test -p ctnr-cli`) para garantizar wiring y manejo de errores.

## 3. Automatización
- GitHub Actions con jobs separados (agent, backend, frontend, CLI).  
- Ambientes efímeros Windows Server para pruebas end-to-end reales.  
- Publicar reportes y artefactos `.ctnr` desde los pipelines.

## 4. Cobertura y métricas
- Objetivo inicial: 70 % en agent/backend, 60 % en frontend.  
- Benchmarks periódicos para comparar overhead del runtime frente a ejecución nativa.

## 5. Próximos pasos
1. Integrar cargo-nextest/vitest/Playwright en CI.  
2. Generar contenedores de ejemplo reutilizables en las suites automatizadas.  
3. Añadir pruebas e2e completas para flujos de publicación/exportación antes de GA.

