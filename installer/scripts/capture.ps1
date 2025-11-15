#requires -version 7
param(
    [Parameter(Mandatory = $true)]
    [string]$InstallerPath,

    [string]$Arguments = "/quiet",

    [Parameter(Mandatory = $true)]
    [string]$ContainerRoot
)

$ErrorActionPreference = "Stop"

Write-Host "[capture] Preparando entorno para $InstallerPath"

$env:CONTAINER_ROOT = $ContainerRoot
$env:APPDATA = Join-Path $ContainerRoot "user/AppData/Roaming"
$env:LOCALAPPDATA = Join-Path $ContainerRoot "user/LocalAppData"
$env:PROGRAMFILES = Join-Path $ContainerRoot "rootfs/ProgramFiles"
$env:TEMP = Join-Path $ContainerRoot "temp"
$env:TMP = $env:TEMP

New-Item -ItemType Directory -Force -Path @(
    $env:APPDATA,
    $env:LOCALAPPDATA,
    $env:PROGRAMFILES,
    $env:TEMP
) | Out-Null

$logDir = Join-Path $ContainerRoot "installer-logs"
New-Item -ItemType Directory -Force -Path $logDir | Out-Null
$timestamp = Get-Date -Format 'yyyyMMdd-HHmmss'
$stdoutLog = Join-Path $logDir "$timestamp.out.log"
$stderrLog = Join-Path $logDir "$timestamp.err.log"

Write-Host "[capture] Ejecutando instalador..."
Start-Process -FilePath $InstallerPath -ArgumentList $Arguments `
    -Wait -NoNewWindow `
    -RedirectStandardOutput $stdoutLog `
    -RedirectStandardError $stderrLog

Write-Host "[capture] Instalación finalizada. Registros en:"
Write-Host "  * STDOUT: $stdoutLog"
Write-Host "  * STDERR: $stderrLog"
