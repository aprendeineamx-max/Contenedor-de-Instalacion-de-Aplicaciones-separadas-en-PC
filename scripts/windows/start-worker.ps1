$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
Set-Location $repoRoot

$env:PATH = "$Env:USERPROFILE\.cargo\bin;$env:PATH"
$env:PROTOC = [Environment]::GetEnvironmentVariable("PROTOC","User")
$env:DATABASE_URL = [Environment]::GetEnvironmentVariable("DATABASE_URL","User")
$env:REDIS_URL = [Environment]::GetEnvironmentVariable("REDIS_URL","User")

Write-Host "Iniciando worker Redis (containers:create)..." -ForegroundColor Cyan
cargo run -p backend --bin worker
