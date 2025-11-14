$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
Set-Location $repoRoot

$env:PATH = "$Env:USERPROFILE\.cargo\bin;$env:PATH"
$env:PROTOC = [Environment]::GetEnvironmentVariable("PROTOC","User")
$pgEncoding = [Environment]::GetEnvironmentVariable("PGCLIENTENCODING","User")
if ($pgEncoding) {
    $env:PGCLIENTENCODING = $pgEncoding
}
$env:DATABASE_URL = [Environment]::GetEnvironmentVariable("DATABASE_URL","User")
$env:REDIS_URL = [Environment]::GetEnvironmentVariable("REDIS_URL","User")
$env:CONTAINERS_API_KEY = [Environment]::GetEnvironmentVariable("CONTAINERS_API_KEY","User")

Write-Host "Iniciando backend (REST + gRPC)..." -ForegroundColor Cyan
cargo run -p backend --bin backend
