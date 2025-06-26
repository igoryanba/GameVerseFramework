#requires -Version 5.0
<##
GameVerse Windows installer script
Usage:
  powershell -ExecutionPolicy Bypass -File install.ps1 [tag]
If tag omitted, installs latest release.
##>
param([string]$Tag = "latest")

$ErrorActionPreference = 'Stop'

$Repo = "igoryanba/GameVerseFramework"
$Arch = "x86_64"
$Platform = "windows"
$BinDir = "$env:USERPROFILE\.gameverse\bin"
New-Item -ItemType Directory -Force -Path $BinDir | Out-Null

function Need-Cmd([string]$name) {
  if (-not (Get-Command $name -ErrorAction SilentlyContinue)) {
    Write-Error "❌ '$name' not found. Install it first."; exit 1 }
}

Need-Cmd curl
Need-Cmd Expand-Archive

if ($Tag -eq "latest") {
  Write-Host "🛰️  Fetching latest release tag…"
  $json = curl -s "https://api.github.com/repos/$Repo/releases/latest" | ConvertFrom-Json
  $Tag = $json.tag_name
}

$AssetName = "gameverse-$Platform-$Arch.zip"
$DownloadUrl = "https://github.com/$Repo/releases/download/$Tag/$AssetName"
$TempFile = "$env:TEMP\$AssetName"

Write-Host "📥 Downloading $DownloadUrl"
curl -L $DownloadUrl -o $TempFile

Write-Host "📦 Extracting to $BinDir"
Expand-Archive -LiteralPath $TempFile -DestinationPath $BinDir -Force
Remove-Item $TempFile

$profilePath = "$HOME\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1"
if (-not ($env:PATH -split ';' | Where-Object { $_ -eq $BinDir })) {
  Write-Host "➕ Adding $BinDir to PATH via profile ($profilePath)"
  if (-not (Test-Path $profilePath)) { New-Item -ItemType File -Force -Path $profilePath | Out-Null }
  Add-Content -Path $profilePath -Value "`n# GameVerse CLI`n`$Env:Path += ';$BinDir'`n"
}

Write-Host "✅ GameVerse installed. Open new terminal and run 'gameverse --help'" 