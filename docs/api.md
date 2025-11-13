# Contratos API

## REST (HTTP)
Base URL: `http://localhost:8080`

| Método | Ruta | Descripción |
| ------ | ---- | ----------- |
| `GET` | `/healthz` | Estado del backend y versión en ejecución. |
| `GET` | `/api/containers` | Lista contenedores registrados. Soporta filtros. |
| `POST` | `/api/containers` | Crea un contenedor placeholder y devuelve su resumen. |
| `DELETE` | `/api/containers/:id` | Elimina el contenedor indicado. |

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
- `search`: busca por coincidencias parciales en `id` o `name`.
- `limit`: número máximo de registros (1-100, default 25).
- `offset`: desplazamiento para paginación (default 0).

## gRPC
- Puerto: `0.0.0.0:50051`
- Archivo proto: `proto/containers.proto`

Servicio principal: `containers.v1.ContainerService`

| RPC | Request | Response | Descripción |
| --- | ------- | -------- | ----------- |
| `ListContainers` | `ListContainersRequest` | `ListContainersResponse` | Lista contenedores cargados. |
| `CreateContainer` | `CreateContainerRequest` | `CreateContainerResponse` | Crea contenedor en memoria. |
| `GetContainer` | `GetContainerRequest` | `GetContainerResponse` | Obtiene detalle. |
| `DeleteContainer` | `DeleteContainerRequest` | `DeleteContainerResponse` | Elimina contenedor. |

### Ejemplo `containers.v1.ListContainers`
```proto
rpc ListContainers (ListContainersRequest) returns (ListContainersResponse);
```
```json
// Response data
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

La CLI y el panel web consumirán los endpoints REST durante las primeras iteraciones, mientras que el agent/backplane utilizarán gRPC para operaciones internas y agentes remotos.
