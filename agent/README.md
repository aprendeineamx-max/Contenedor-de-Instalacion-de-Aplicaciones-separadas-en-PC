# Agent Host

Servicio residente escrito en Rust que:
- Administra el registro de contenedores y sus rutas.
- Lanza procesos dentro de los contenedores aplicando hooks (Detours/WinFSP/minifilter).
- Mantiene comunicación segura con el backend via gRPC sobre Named Pipes.
- Expone un API local minimalista para utilidades del sistema (diagnósticos, métricas).

## Próximas Tareas
- Prototipo de hooking de filesystem y `%APPDATA%`.
- Carga/descarga de hives de registro por contenedor.
- Integración con el motor de captura para recibir builds listos.

