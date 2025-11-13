# Especificación Técnica del Sistema de Contenedores Win32

## 1. Objetivos Principales
- Permitir instalaciones aisladas de aplicaciones Win32, encapsulando binarios, dependencias, datos de usuario y estado del sistema dentro de carpetas portables (.ctnr).
- Garantizar coexistencia de múltiples versiones de una misma app sin colisiones en rutas o registro.
- Facilitar exportación/importación de contenedores y su ejecución en distintas PCs sin reconfiguración adicional.

## 2. Componentes Globales
1. **Agent Host (Rust)**  
   Servicio residente con privilegios controlados que orquesta contenedores, expone APIs REST/gRPC y coordina el runtime.
2. **Runtime de Contenedor**  
   Conjunto de hooks (Detours/easyhook) + drivers opcionales (minifilter, WinFSP/Dokany) que virtualizan filesystem/registro para cada proceso lanzado.
3. **Motor de Captura de Instaladores**  
   Sandbox temporal que monitorea FS/registry, identifica escrituras y genera el layout del contenedor.
4. **Plano de Control Web**  
   Backend Axum/Tonic + Frontend Next.js, responsable de UX, RBAC, automatización y métricas.
5. **CLI y Automations**  
   Herramienta en Rust (clap) para interacción rápida y pipelines CI.

## 3. Estructura de Carpeta del Contenedor
```
<container>/
├── config.yml             # Metadatos, variables, políticas, hash, versión runtime
├── rootfs/                # Mirror de Program Files & dependencias del instalador
├── user/                  # Datos de usuario y registros
│   ├── AppData/
│   ├── LocalAppData/
│   ├── Registry/
│   │   ├── HKCU.hiv
│   │   └── HKLM-SW.hiv
├── temp/                  # Ruta temporal montada como %TEMP%
├── cache/                 # Cachés persistentes opcionales
├── snapshots/             # Deltas copy-on-write o checkpoints etiquetados
├── bin/
│   ├── launcher.exe       # Wrapper que monta VFS + hooks
│   └── runtime/           # Bibliotecas compartidas del agente
└── logs/
    └── events.log         # Trazas internas para depuración
```

### 3.1 `config.yml`
```yaml
id: "chrome-beta-118"
name: "Chrome Beta 118"
version: "118.0.1234.5"
runtime:
  build: "container-runtime@0.3.0"
  hooks:
    filesystem: "detours"
    registry: "minifilter"        # opciones: detours, minifilter, mixto
paths:
  program_files: "rootfs/ProgramFiles"
  appdata: "user/AppData/Roaming"
  local_appdata: "user/LocalAppData"
  temp: "temp"
env:
  - key: "APPDATA"
    value: "%CONTAINER_APPDATA%"
services:
  - name: "chrome-updater"
    type: "windows-service"
snapshots:
  strategy: "vhd-diff"
security:
  signed: true
  encryption: "aes-gcm-256"
```

## 4. Virtualización de Recursos
- **Filesystem**: capas overlay con prioridad `container rootfs > base runtime > host`. WinFSP/Dokany monta un volumen virtual asignado al proceso; minifilter opcional para capturar accesos fuera del volumen.
- **Registro**: hives por contenedor (`HKCU`, subset `HKLM\Software`). Se cargan mediante `RegLoadKey` antes de lanzar el proceso y se descargan al finalizar.
- **Variables de Entorno**: wrapper reemplaza rutas estándar (`%ProgramFiles%`, `%APPDATA%`, `%TEMP%`) por las internas del contenedor.
- **Servicios/Drivers**: si la app instala servicios, se crea un stub que redirige controles al contenedor o se marca como “shared service” con advertencias.

## 5. Ciclo de Vida del Contenedor
1. `Create`  
   - Selección de plantilla base (vacía, App preconfigurada, snapshot).  
   - Definición de parámetros iniciales (nombre, versión, políticas).
2. `CaptureInstall`  
   - Ejecutar instalador dentro de sandbox monitorizado.  
   - Interceptar FS/registry, copiar archivos al layout, crear manifiesto.
3. `Build`  
   - Empaquetar runtime, generar `launcher.exe`, calcular hashes y firmar metadata.
4. `Run`  
   - Montar VFS, cargar hives, aplicar hooks, lanzar proceso objetivo, capturar logs.
5. `Snapshot`  
   - Freeze, copiar delta (VHD diff o rsync incremental), etiquetar y almacenar.
6. `Export/Import`  
   - Comprimir carpeta + metadata en `.ctnr`. Opcional: cifrar y firmar.  
   - Validar firma al importar, recrear rutas y actualizar base de datos.
7. `Retire`  
   - Descargar hives, desmontar VFS, limpiar credenciales/tokens.

## 6. Interacción con el Sistema
- **Agent <-> Runtime**: comunicación vía gRPC sobre Named Pipes. El agent gestiona permisos y entrega tokens de contenedor.
- **UI/CLI <-> Agent**: API HTTP/2 + WebSocket para eventos (logs, estado, métricas).
- **Base de Datos**: PostgreSQL para inventario, Redis para jobs en cola (instalaciones masivas, pruebas automatizadas).

## 7. Seguridad
- Firmas digitales (certificados propios) para garantizar integridad de runtimes y contenedores.  
- RBAC/OIDC en el panel; cada contenedor tiene ACLs por usuario/grupo.  
- Opcional: cifrado por contenedor (clave derivada de passphrase o TPM).  
- Auditoría centralizada: cada operación se registra con timestamp, usuario, resultado.

## 8. Observabilidad
- Logs estructurados (JSON) desde agent/runtime.  
- Métricas Prometheus: cantidad de contenedores, uso de CPU/RAM por contenedor, tiempos de instalación.  
- Trazas distribuidas (OpenTelemetry) para capturar eventos desde el UI hasta el runtime.

## 9. Compatibilidad y Limitaciones
- Apps con drivers kernel complejos requerirán soporte extendido o ejecución en contenedores Hyper-V dedicados.  
- Durante la fase inicial, solo se soportarán instaladores x64 en Windows 11; se planifica compatibilidad x86 y Windows 10 en fases posteriores.  
- Para scripts MSI con CustomActions elevados, se usará un helper firmado que proxy la elevación manteniendo el aislamiento.

## 10. Próximos Pasos Técnicos
1. Implementar prototipo de hooking usuario que demuestre redirección de `%APPDATA%`.  
2. Diseñar esquema de base de datos (entidades Container, Snapshot, Task).  
3. Crear plantillas de Next.js + Axum con contratos de API iniciales.  
4. Definir formato `.ctnr` (tarball + manifest + checksum) y flujo de firma/verificación.

