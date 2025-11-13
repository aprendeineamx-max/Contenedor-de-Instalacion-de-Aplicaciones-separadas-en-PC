# Infrastructure

Define los artefactos necesarios para instalar y operar la plataforma:
- Scripts PowerShell para desplegar el agent y configurar drivers.
- Manifests IaC (Terraform/Ansible) para hospedar el backend/UI en la nube.
- Configuraciones de CI/CD (GitHub Actions) para compilar runtimes, contenedores demo y publicar releases.

## Acciones Pendientes
- Documentar requisitos de compilación (Rust toolchain, Node.js, dependencias WinFSP/Dokany).
- Preparar scripts de desarrollo (`justfile` o `make.ps1`) que automaticen build/test.
- Diseñar pipeline de release con firmas digitales.

