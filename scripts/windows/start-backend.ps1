$ErrorActionPreference = "Stop"

function Set-RepoContext {
    $script:repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
    Set-Location $script:repoRoot
}

function Initialize-Env {
    $env:PATH = "$Env:USERPROFILE\.cargo\bin;$env:PATH"
    $env:PROTOC = [Environment]::GetEnvironmentVariable("PROTOC","User")

    foreach ($name in @("PGCLIENTENCODING","PGOPTIONS","DATABASE_URL","REDIS_URL","CONTAINERS_API_KEY")) {
        $value = [Environment]::GetEnvironmentVariable($name,"User")
        if ($value) {
            Set-Item -Path Env:$name -Value $value
        }
    }

    if (-not $env:DATABASE_URL -or -not $env:REDIS_URL) {
        throw "Faltan variables de entorno DATABASE_URL/REDIS_URL. Ejecuta setup-postgres-redis primero."
    }
}

function Assert-PortFree {
    param(
        [int[]]$Ports
    )

    foreach ($port in $Ports) {
        $conns = Get-NetTCPConnection -LocalPort $port -ErrorAction SilentlyContinue
        if ($null -ne $conns) {
            $procIds = $conns | Select-Object -ExpandProperty OwningProcess -Unique
            foreach ($procId in $procIds) {
                $proc = Get-Process -Id $procId -ErrorAction SilentlyContinue
                if ($proc -and $proc.ProcessName -eq "backend") {
                    Write-Warning "Puerto $port ocupado por proceso backend (PID $procId). Deteniendo..."
                    Stop-Process -Id $procId -Force
                } else {
                    $name = if ($proc) { $proc.ProcessName } else { "desconocido" }
                    throw "El puerto $port está en uso por PID $procId ($name). Ajusta BACKEND_HTTP_ADDR/BACKEND_GRPC_ADDR y reintenta."
                }
            }
        }
    }
}

function Ensure-Tooling {
    Write-Host "Formateando cA?digo (cargo fmt)..." -ForegroundColor DarkCyan
    cargo fmt --all

    Write-Host "Preparando sentencias SQLX..." -ForegroundColor DarkCyan
    try {
        cargo sqlx prepare -- --bin backend | Out-Null
    } catch {
        Write-Warning "No se pudo ejecutar 'cargo sqlx prepare'. Verifica que sqlx-cli estA? instalado: cargo install sqlx-cli --no-default-features --features native-tls,postgres"
    }
}

function Start-Backend {
    $logDir = Join-Path $script:repoRoot "logs\windows"
    New-Item -ItemType Directory -Force -Path $logDir | Out-Null
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $logFile = Join-Path $logDir "backend_$timestamp.log"
    New-Item -ItemType File -Path $logFile -Force | Out-Null
    Write-Host "Logs -> $logFile" -ForegroundColor DarkGray

    Write-Host "Iniciando backend (REST + gRPC)..." -ForegroundColor Cyan
    cargo run -p backend --bin backend 2>&1 | Tee-Object -FilePath $logFile -Append
}

Set-RepoContext
Initialize-Env
Assert-PortFree -Ports @(8080,50051)
Ensure-Tooling
Start-Backend
