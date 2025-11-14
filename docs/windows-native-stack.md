# Stack nativo en Windows (sin Docker)

Este documento describe cómo levantar Postgres y Redis compatibles usando instaladores nativos para que el backend funcione en un Windows Server sin soporte de virtualización.

## 1. Requisitos
- Windows Server 2022/2025 (con privilegios de administrador).
- PowerShell 7+ (recomendado).
- Conexión a internet para usar `winget`.

## 2. Instalación automatizada
El repositorio incluye `scripts/windows/setup-postgres-redis.ps1`. Ejemplo:

```powershell
cd C:\Users\Administrator\Desktop\Contenedor de Instalación de Aplicaciones separadas en PC
.\scripts\windows\setup-postgres-redis.ps1 `
  -PostgresSuperuserPassword "TuPasswordDePostgres" `
  -AppUser "containers" `
  -AppPassword "containers" `
  -Database "containers" `
  -ApiKey "super-secreto"
```

El script realiza lo siguiente:
1. Instala PostgreSQL 16 y Memurai Developer (Redis compatible) si no existen.
2. Inicia los servicios `postgresql-x64-16` y `Memurai`.
3. Añade `C:\Program Files\PostgreSQL\16\bin` al `PATH`.
4. Crea la base de datos/usuario para la app.
5. Define `DATABASE_URL`, `REDIS_URL` y `CONTAINERS_API_KEY` en el perfil del usuario.

> **Nota:** Debes recordar la contraseña que elegiste para el superusuario `postgres` durante la instalación; el script la necesita para ejecutar `psql`.

## 3. Iniciar backend y worker
Después de ejecutar el script (y abrir una nueva consola para tomar las variables):

```powershell
# Consola A
$env:DATABASE_URL = [Environment]::GetEnvironmentVariable("DATABASE_URL","User")
$env:REDIS_URL = [Environment]::GetEnvironmentVariable("REDIS_URL","User")
$env:CONTAINERS_API_KEY = [Environment]::GetEnvironmentVariable("CONTAINERS_API_KEY","User")
cargo run -p backend

# Consola B (worker)
$env:DATABASE_URL = [Environment]::GetEnvironmentVariable("DATABASE_URL","User")
$env:REDIS_URL = [Environment]::GetEnvironmentVariable("REDIS_URL","User")
cargo run -p backend --bin worker
```

## 4. Verificación rápida
- `psql -U containers -d containers -c "\dt"` — comprueba la base.
- `redis-cli ping` (instalado junto a Memurai) — debe responder `PONG`.
- `http://localhost:8080/healthz` — backend arriba.

## 5. Troubleshooting
- **Contraseña incorrecta de postgres:** Ejecuta `psql` con el usuario `postgres` y cambia la contraseña (`\password postgres`).
- **Servicios no inician:** abre `services.msc` y revisa logs de `PostgreSQL`/`Memurai`, o ejecuta `Get-EventLog -LogName Application -Newest 50`.
- **Variables de entorno no cargan:** cierra la terminal y vuelve a abrir; también puedes usar `setx` manualmente.
