$ErrorActionPreference = "Stop"

function Set-RepoContext {
    $script:repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
    Set-Location $script:repoRoot
}

function Initialize-Env {
    $env:PATH = "$Env:USERPROFILE\.cargo\bin;$env:PATH"
    $env:PROTOC = [Environment]::GetEnvironmentVariable("PROTOC","User")

    foreach ($name in @("PGCLIENTENCODING","PGOPTIONS","DATABASE_URL","REDIS_URL")) {
        $value = [Environment]::GetEnvironmentVariable($name,"User")
        if ($value) {
            Set-Item -Path Env:$name -Value $value
        }
    }

    if (-not $env:DATABASE_URL -or -not $env:REDIS_URL) {
        throw "Faltan variables de entorno DATABASE_URL/REDIS_URL. Ejecuta setup-postgres-redis primero."
    }
}

function Start-Worker {
    $logDir = Join-Path $script:repoRoot "logs\windows"
    New-Item -ItemType Directory -Force -Path $logDir | Out-Null
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $logFile = Join-Path $logDir "worker_$timestamp.log"
    New-Item -ItemType File -Path $logFile -Force | Out-Null
    Write-Host "Logs -> $logFile" -ForegroundColor DarkGray

    Write-Host "Iniciando worker Redis (containers:create)..." -ForegroundColor Cyan
    cargo run -p backend --bin worker 2>&1 | Tee-Object -FilePath $logFile -Append
}

Set-RepoContext
Initialize-Env
Start-Worker
