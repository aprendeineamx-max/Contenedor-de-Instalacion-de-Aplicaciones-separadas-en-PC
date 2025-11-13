# Pipeline de Captura de Instaladores

## Objetivo
Ejecutar cualquier instalador Win32 dentro de un sandbox con hooks activos, registrar los archivos/registro modificados y generar automáticamente el `config.yml`, `rootfs/` y `launcher.exe` del contenedor.

## Flujo propuesto
1. **CLI/UI**  
   - El usuario sube el instalador (MSI/EXE) o especifica la ruta local.  
   - Se envía una tarea al backend (`containers:create`) con metadata (nombre, versión, argumentos silenciosos, etc.).
2. **Worker Redis**  
   - Descarga/copía el instalador al staging del contenedor.  
   - Invoca `installer/scripts/capture.ps1` para lanzar el instalador envuelto en hooks y monitoreo.
3. **Hook de captura**  
   - Utiliza `ProcMon`/`Detours` para redirigir writes hacia `rootfs/`.  
   - Exporta cambios de registro a `user/Registry/*.hiv`.  
   - Genera un diff (JSON) consumido por el backend.
4. **Publicación**  
   - El backend escribe `config.yml`, genera `launcher.exe` y actualiza la base de datos.  
   - El contenedor queda listo para exportarse o ejecutarse desde el Agent.

## Scripts
- `scripts/capture.ps1`: envuelve el instalador, configura variables (`CONTAINER_ROOT`, `%APPDATA%` virtual, etc.) y produce un log con los artefactos capturados.

## Próximos pasos técnicos
1. Integrar `procmon` CLI / ETW para seguimiento de archivos.  
2. Serializar los cambios en un manifiesto (`installer/logs/<timestamp>.json`).  
3. Enviar progreso a Redis para que el backend/Frontend pueda mostrarlo en vivo.

