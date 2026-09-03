param([string]$Dotnet = 'dotnet')
$ErrorActionPreference = 'Stop'
$vendor = Join-Path $PSScriptRoot 'vendor'
New-Item -ItemType Directory -Path $vendor -Force | Out-Null
$archive = Join-Path $vendor 'shvdne-v1.1.0.6.zip'
$uri = 'https://github.com/Chiheb-Bacha/ScriptHookVDotNetEnhanced/releases/download/v1.1.0.6/ScriptHookVDotNetEnhanced-v1.1.0.6.zip'
$expected = 'F10DA8819FA6814FB0A04553567D2516934EAC96B16AF06AD8F315F0DA00473A'
if (-not (Test-Path -LiteralPath $archive)) { Invoke-WebRequest $uri -OutFile $archive }
if ((Get-FileHash -LiteralPath $archive).Hash -ne $expected) { throw 'SHVDNE archive checksum mismatch' }
Expand-Archive -LiteralPath $archive -DestinationPath (Join-Path $vendor 'shvdne') -Force
& $Dotnet build (Join-Path $PSScriptRoot 'GameVerse.GtaAdapter') -c Release --nologo
if ($LASTEXITCODE -ne 0) { throw 'Adapter build failed' }
& $Dotnet build (Join-Path $PSScriptRoot 'tests') -c Release --nologo
if ($LASTEXITCODE -ne 0) { throw 'Harness build failed' }
& $Dotnet run --project (Join-Path $PSScriptRoot 'tests') -c Release -f net48 --no-build -- --self-test
if ($LASTEXITCODE -ne 0) { throw 'IPC self-test failed' }
