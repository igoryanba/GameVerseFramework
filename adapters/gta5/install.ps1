param(
    [Parameter(Mandatory)][string]$GamePath,
    [Parameter(Mandatory)][string]$ScriptHookPath,
    [string]$ShvdnePath = (Join-Path $PSScriptRoot 'vendor/shvdne')
)
$ErrorActionPreference = 'Stop'
$GamePath = (Resolve-Path -LiteralPath $GamePath).Path
$exe = Join-Path $GamePath 'GTA5_Enhanced.exe'
if (-not (Test-Path -LiteralPath $exe)) { throw 'GTA5_Enhanced.exe not found' }
if ((Get-Item -LiteralPath $exe).VersionInfo.FileVersion -ne '1.0.1158.13') { throw 'Only Enhanced 1.0.1158.13 is tested as the target build' }
if (Get-Process GTA5_Enhanced -ErrorAction SilentlyContinue) { throw 'Close GTA before installing the adapter' }
$adapter = Join-Path $PSScriptRoot 'GameVerse.GtaAdapter/bin/Release/net48'
$files = @(
    @{Source=(Join-Path $ScriptHookPath 'bin/ScriptHookV.dll');Relative='ScriptHookV.dll'},
    @{Source=(Join-Path $ScriptHookPath 'bin/xinput1_4.dll');Relative='xinput1_4.dll'},
    @{Source=(Join-Path $ShvdnePath 'ScriptHookVDotNet.asi');Relative='ScriptHookVDotNet.asi'},
    @{Source=(Join-Path $ShvdnePath 'ScriptHookVDotNet.ini');Relative='ScriptHookVDotNet.ini'},
    @{Source=(Join-Path $ShvdnePath 'ScriptHookVDotNet2.dll');Relative='ScriptHookVDotNet2.dll'},
    @{Source=(Join-Path $ShvdnePath 'ScriptHookVDotNet3.dll');Relative='ScriptHookVDotNet3.dll'},
    @{Source=(Join-Path $ShvdnePath 'MinHook.x64.dll');Relative='MinHook.x64.dll'},
    @{Source=(Join-Path $adapter 'GameVerse.GtaAdapter.dll');Relative='scripts/GameVerse.GtaAdapter.dll'},
    @{Source=(Join-Path $adapter 'Newtonsoft.Json.dll');Relative='scripts/Newtonsoft.Json.dll'},
    @{Source=(Join-Path $PSScriptRoot '..\..\.build\native-bootstrap\Release\GameVerse.NativeBootstrap.asi');Relative='GameVerse.NativeBootstrap.asi'},
    @{Source=(Join-Path $PSScriptRoot '..\..\native\GameVerse.NativeBootstrap\compatibility\enhanced-1.0.1158.13.json');Relative='enhanced-1.0.1158.13.json'},
    @{Source=(Join-Path $PSScriptRoot '..\..\native\GameVerse.NativeBootstrap\compatibility\enhanced-1.0.1158.13.sig');Relative='enhanced-1.0.1158.13.sig'},
    @{Source=(Join-Path $PSScriptRoot '..\..\native\GameVerse.NativeBootstrap\compatibility\telemetry-candidates-v1.json');Relative='telemetry-candidates-v1.json'},
    @{Source=(Join-Path $PSScriptRoot '..\..\native\GameVerse.NativeBootstrap\compatibility\telemetry-candidates-v1.sig');Relative='telemetry-candidates-v1.sig'}
)
# Validate the entire installation first. Existing mods are not overwritten.
foreach ($file in $files) {
    if (-not (Test-Path -LiteralPath $file.Source)) { throw "Missing build/dependency: $($file.Source)" }
    if (Test-Path -LiteralPath (Join-Path $GamePath $file.Relative)) { throw "Existing file: $($file.Relative); inspect it before installing" }
}
$manifestPath = Join-Path $GamePath 'GameVerse.install.json'
if (Test-Path -LiteralPath $manifestPath) { throw 'An installation manifest already exists' }
New-Item -ItemType Directory -Path (Join-Path $GamePath 'scripts') -Force | Out-Null
$manifest = [Collections.Generic.List[object]]::new()
$copied = [Collections.Generic.List[string]]::new()
try {
    foreach ($file in $files) {
        $target = Join-Path $GamePath $file.Relative
        Copy-Item -LiteralPath $file.Source -Destination $target
        $copied.Add($target)
        $manifest.Add([pscustomobject]@{RelativePath=$file.Relative;SHA256=(Get-FileHash -LiteralPath $target).Hash})
    }
    $manifestTemporary = $manifestPath + '.tmp'
    ConvertTo-Json -InputObject $manifest.ToArray() | Set-Content -LiteralPath $manifestTemporary
    Move-Item -LiteralPath $manifestTemporary -Destination $manifestPath
}
catch {
    foreach ($target in $copied) { Remove-Item -LiteralPath $target -Force -ErrorAction SilentlyContinue }
    Remove-Item -LiteralPath ($manifestPath + '.tmp') -Force -ErrorAction SilentlyContinue
    throw
}
Write-Output 'Installed adapter and native bootstrap files. Start GameVerse with the launcher.'
# No args.txt, launcher changes, entitlement changes or anti-cheat settings are applied.
