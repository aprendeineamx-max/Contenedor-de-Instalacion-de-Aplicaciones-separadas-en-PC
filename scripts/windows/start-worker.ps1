$ErrorActionPreference = "Stop"

Set-Location "C:\Users\Administrator\Desktop\Contenedor de Instalación de Aplicaciones separadas en PC"

$env:PATH = "$Env:USERPROFILE\.cargo\bin;$env:PATH"
$env:DATABASE_URL = [Environment]::GetEnvironmentVariable("DATABASE_URL","User")
$env:REDIS_URL = [Environment]::GetEnvironmentVariable("REDIS_URL","User")

Write-Host "Iniciando worker Redis (containers:create)..." -ForegroundColor Cyan
cargo run -p backend --bin worker
