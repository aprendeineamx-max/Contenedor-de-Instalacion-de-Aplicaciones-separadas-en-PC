# Roadmap de Desarrollo

## Fase P0 – Fundamentos (Semanas 1-3)
1. Prototipo de hooks de filesystem/registro a nivel usuario (Detours + WinFSP).  
2. Definición de contratos gRPC/REST y esquema de base de datos.  
3. Scaffold de proyectos (agent, backend, frontend, cli) con pipelines básicos.

## Fase P1 – Captura e Instalación (Semanas 4-8)
1. Motor de captura que ejecute instaladores dentro de un sandbox vigilado.  
2. Generación automática de `config.yml`, `launcher.exe` y empaquetado del contenedor.  
3. UI/CLI para subir instaladores, monitorear progreso y revisar logs.

## Fase P2 – Panel y Operaciones (Semanas 9-12)
1. Backend con almacenamiento real (PostgreSQL + Redis) y RBAC inicial.  
2. Panel Next.js con dashboard, wizard de creación y detalle de contenedores.  
3. Integración de WebSockets para eventos en vivo y métricas básicas.

## Fase P3 – Portabilidad & Snapshots (Semanas 13-16)
1. Implementación de snapshots (VHDX diff o NTFS sparse).  
2. Exportación/importación `.ctnr` con firma y verificación de integridad.  
3. CLI y API para clonar contenedores y compartir plantillas.

## Fase P4 – Seguridad y Marketplace (Semanas 17-20)
1. OIDC, roles granulares, auditoría completa.  
2. Cifrado por contenedor y gestión de claves (TPM/PFX).  
3. Marketplace de plantillas y agentes remotos multi-PC.

## Fase P5 – Observabilidad y Automatización (Semanas 21+)
1. Integración con Prometheus/Grafana y OpenTelemetry.  
2. Motor de scripts/tests automáticos (PowerShell/Python) dentro de contenedores.  
3. Publicación de SDK/Plugins para partners externos.

