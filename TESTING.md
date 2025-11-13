# Estrategia de Pruebas

## 1. Pirámide de Tests
- **Unitarias (Rust/TypeScript)**: validan hooks, parsers de manifiestos, servicios Axum, componentes React.
- **Integración**: escenarios que levantan el agent con un contenedor dummy y verifican la redirección de rutas/registro.
- **End-to-End**: usan la CLI o el panel para crear, instalar y ejecutar aplicaciones reales (Notepad++, 7zip, navegadores).

## 2. Suites Iniciales
1. **Runtime Hook Tests**  
   - Ejecutar app de ejemplo y confirmar que escrituras a `%APPDATA%` terminan en `user/AppData`.  
   - Medir performance del VFS bajo carga.
2. **Installer Capture Tests**  
   - Reproducir instalación MSI/EXE en sandbox y comparar archivos esperados vs capturados.  
   - Validar que se generen manifest y launcher correctos.
3. **API/Backend**  
   - Contratos OpenAPI/gRPC con pruebas contractuales.  
   - Simulaciones de concurrencia en creación/ejecución de contenedores.
4. **Frontend e2e**  
   - Playwright/Cypress para flujos de UI (crear contenedor, lanzar app, exportar).

## 3. Automatización de Tests
- GitHub Actions con jobs separados (agent, backend, frontend).  
- Uso de ambientes efímeros Windows Server para pruebas e2e reales.  
- Publicar reportes y artefactos (.ctnr de ejemplo) por pipeline.

## 4. Cobertura y Métricas
- Objetivo inicial: 70% unitarias en agent/backend, 60% en frontend.  
- Benchmarks periódicos para comparar overhead del runtime vs ejecución nativa.

## 5. Próximos Pasos
1. Configurar toolchain de pruebas (cargo-nextest, vitest, Playwright).  
2. Crear contenedor de ejemplo para demostración automática.  
3. Integrar pruebas básicas en CI tan pronto como exista código ejecutable.

