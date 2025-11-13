# Backend (Plano de Control)

Servicio Axum/Tonic en Rust que provee:
- APIs REST/gRPC para CRUD de contenedores, tareas y snapshots.
- WebSocket/EventStream para logs y métricas en tiempo real.
- Orquestación de trabajos asincrónicos a través de Redis.
- Integración con PostgreSQL como base de datos principal.

## Pasos Iniciales
1. Definir contratos proto (containers, tasks, runtime events).
2. Crear migraciones de base de datos.
3. Implementar servicios CRUD con mock storage para habilitar el frontend rápidamente.

