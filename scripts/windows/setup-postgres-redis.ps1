<#
.SYNOPSIS
  Instala y configura PostgreSQL + Memurai (Redis compatible) en Windows sin depender de Docker.

.DESCRIPTION
  - Verifica si PostgreSQL y Memurai están instalados; de lo contrario usa winget para instalarlos.
  - Inicia los servicios requeridos.
  - Crea una base de datos/usuario para la aplicación.
  - Configura las variables de entorno `DATABASE_URL`, `REDIS_URL` y `CONTAINERS_API_KEY`.

.PARAMETER PostgresVersion
  Versión principal de PostgreSQL instalada (por defecto 16).

.PARAMETER PostgresSuperuser
  Usuario super administrador, normalmente `postgres`.

.PARAMETER PostgresSuperuserPassword
  Contraseña del super usuario (la que definiste durante la instalación).

.PARAMETER AppUser
  Usuario que se creará para la aplicación (por defecto `containers`).

.PARAMETER AppPassword
  Contraseña para el usuario de la aplicación (por defecto `containers`).

.PARAMETER Database
  Nombre de la base de datos que se creará (por defecto `containers`).

.PARAMETER ApiKey
  API key que expondrá el backend (por defecto `super-secreto`).
#>

[CmdletBinding()]
param(
    [string]$PostgresVersion = "16",
    [string]$PostgresSuperuser = "postgres",
    [Parameter(Mandatory = $true)]
    [string]$PostgresSuperuserPassword,
    [string]$AppUser = "containers",
    [string]$AppPassword = "containers",
    [string]$Database = "containers",
    [string]$ApiKey = "super-secreto",
    [switch]$SkipInstall
)

function Invoke-WingetInstall {
    param(
        [string]$Id
    )

    Write-Host "Instalando $Id mediante winget..." -ForegroundColor Cyan
    $process = Start-Process winget -ArgumentList @("install", "-e", "--id", $Id, "--accept-package-agreements", "--accept-source-agreements") -Wait -PassThru
    if ($process.ExitCode -ne 0) {
        throw "Winget devolvió código $($process.ExitCode) al instalar $Id."
    }
}

function Ensure-ServiceRunning {
    param(
        [string]$ServiceName
    )

    $service = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
    if (-not $service) {
        throw "No se encontró el servicio $ServiceName."
    }

    if ($service.Status -ne "Running") {
        Write-Host "Iniciando servicio $ServiceName..." -ForegroundColor Cyan
        Start-Service -Name $ServiceName
        $service.WaitForStatus("Running", (New-TimeSpan -Seconds 30))
    }
}

function Invoke-PostgresQuery {
    param(
        [string]$Query
    )

    $pgBin = "C:\Program Files\PostgreSQL\$PostgresVersion\bin"
    $psql = Join-Path $pgBin "psql.exe"
    if (-not (Test-Path $psql)) {
        throw "No se encontró psql en $psql. Verifica la versión instalada."
    }

    $env:PGPASSWORD = $PostgresSuperuserPassword
    & $psql -U $PostgresSuperuser -d postgres -tAc $Query
    if ($LASTEXITCODE -ne 0) {
        throw "psql devolvió código $LASTEXITCODE para la consulta: $Query"
    }
}

if (-not $SkipInstall) {
    $pgService = "postgresql-x64-$PostgresVersion"
    $pgInstalled = Get-Service -Name $pgService -ErrorAction SilentlyContinue
    if (-not $pgInstalled) {
        Invoke-WingetInstall -Id "PostgreSQL.PostgreSQL.$PostgresVersion"
    }

    $memuraiInstalled = Get-Service -Name "Memurai" -ErrorAction SilentlyContinue
    if (-not $memuraiInstalled) {
        Invoke-WingetInstall -Id "Memurai.MemuraiDeveloper"
    }
}

Ensure-ServiceRunning -ServiceName "postgresql-x64-$PostgresVersion"
Ensure-ServiceRunning -ServiceName "Memurai"

# Añadir binarios de PostgreSQL al PATH del sistema si no existen
$pgBinPath = "C:\Program Files\PostgreSQL\$PostgresVersion\bin"
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "Machine")
if ($currentPath -notlike "*$pgBinPath*") {
    Write-Host "Agregando $pgBinPath al PATH del sistema..." -ForegroundColor Cyan
    [Environment]::SetEnvironmentVariable("PATH", "$pgBinPath;$currentPath", "Machine")
}

# Crear usuario/base de datos si no existen
$roleExists = Invoke-PostgresQuery -Query "SELECT 1 FROM pg_roles WHERE rolname = '$AppUser';"
if ([string]::IsNullOrWhiteSpace($roleExists)) {
    Invoke-PostgresQuery -Query "CREATE ROLE $AppUser WITH LOGIN PASSWORD '$AppPassword';"
    Write-Host "Usuario $AppUser creado." -ForegroundColor Green
}

$dbExists = Invoke-PostgresQuery -Query "SELECT 1 FROM pg_database WHERE datname = '$Database';"
if ([string]::IsNullOrWhiteSpace($dbExists)) {
    Invoke-PostgresQuery -Query "CREATE DATABASE $Database OWNER $AppUser;"
    Write-Host "Base de datos $Database creada." -ForegroundColor Green
}

# Configurar variables de entorno de la app
[Environment]::SetEnvironmentVariable("DATABASE_URL", "postgres://${AppUser}:${AppPassword}@localhost:5432/${Database}", "User")
[Environment]::SetEnvironmentVariable("REDIS_URL", "redis://localhost:6379", "User")
[Environment]::SetEnvironmentVariable("CONTAINERS_API_KEY", $ApiKey, "User")

Write-Host "`nPostgres + Memurai configurados correctamente." -ForegroundColor Green
Write-Host "Reinicia las consolas para heredar las nuevas variables de entorno." -ForegroundColor Yellow
