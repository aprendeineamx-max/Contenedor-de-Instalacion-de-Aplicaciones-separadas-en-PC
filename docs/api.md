# Contratos API

## REST (HTTP)
Base URL: `http://localhost:8080`

| Método | Ruta | Descripción |
| ------ | ---- | ----------- |
| `GET` | `/healthz` | Estado del backend y versión en ejecución. |
| `GET` | `/api/containers` | Lista contenedores registrados (filtros/paginación). |
| `POST` | `/api/containers` | Crea un contenedor y devuelve su resumen. |
| `GET` | `/api/containers/:id` | Obtiene el detalle del contenedor. |
| `DELETE` | `/api/containers/:id` | Elimina el contenedor indicado. |
| `GET` | `/api/events/containers` | Stream SSE con snapshots periódicos. |

### Ejemplo `POST /api/containers`
```http
POST /api/containers HTTP/1.1
Content-Type: application/json

{
  "name": "chrome-beta",
  "version": "118.0"
}
```

Respuesta:
```json
{
  "id": "36aab6d5-02fe-4b68-9020-195ae48bd8f3",
  "name": "chrome-beta",
  "version": "118.0",
  "status": "draft"
}
```

### Parámetros para `GET /api/containers`
- `status`: filtra por estado (`draft`, `running`, etc.).
- `search`: coincidencias parciales en `id` o `name`.
- `limit`: registros por página (1-100, default 25).
- `offset`: desplazamiento para paginación (default 0).

## gRPC
- Puerto: `0.0.0.0:50051`
- Archivo proto: `proto/containers.proto`

Servicio principal: `containers.v1.ContainerService`

| RPC | Request | Response | Descripción |
| --- | ------- | -------- | ----------- |
| `ListContainers` | `ListContainersRequest` | `ListContainersResponse` | Lista contenedores cargados. |
| `CreateContainer` | `CreateContainerRequest` | `CreateContainerResponse` | Crea contenedor y devuelve resumen. |
| `GetContainer` | `GetContainerRequest` | `GetContainerResponse` | Obtiene detalle individual. |
| `DeleteContainer` | `DeleteContainerRequest` | `DeleteContainerResponse` | Elimina contenedor existente. |

### Ejemplo `containers.v1.ListContainers`
```proto
rpc ListContainers (ListContainersRequest) returns (ListContainersResponse);
```
```json
{
  "containers": [
    {
      "id": "36aab6d5-02fe-4b68-9020-195ae48bd8f3",
      "name": "chrome-beta",
      "version": "118.0",
      "status": "draft"
    }
  ]
}
```

### Seguridad
- Todas las rutas bajo `/api/*` aceptan `X-API-Key` (definir `CONTAINERS_API_KEY` en el backend).
- Rate limiting: 120 req/min por instancia.
- Logs y trazas HTTP (`tower-http::trace`) registran usuario, latencia y resultado.

La CLI y el panel web usan REST + SSE para administración interactiva, mientras que los agentes y servicios remotos se conectan al backend mediante gRPC.
