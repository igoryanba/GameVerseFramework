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
    @{Source=(Join-Path $adapter 'Newtonsoft.Json.dll');Relative='scripts/Newtonsoft.Json.dll'}
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
try {
    foreach ($file in $files) {
        $target = Join-Path $GamePath $file.Relative
        Copy-Item -LiteralPath $file.Source -Destination $target
        $manifest.Add([pscustomobject]@{Path=$target;SHA256=(Get-FileHash -LiteralPath $target).Hash})
    }
}
finally { ConvertTo-Json -InputObject $manifest.ToArray() | Set-Content -LiteralPath $manifestPath }
Write-Output 'Installed adapter files. Start the presence server, bridge and bot, then enter Story Mode.'
# No args.txt, launcher changes, entitlement changes or anti-cheat settings are applied.
