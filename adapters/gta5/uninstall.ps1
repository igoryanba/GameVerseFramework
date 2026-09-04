param(
    [Parameter(Mandatory)][string]$GamePath
)

$ErrorActionPreference = 'Stop'
$GamePath = (Resolve-Path -LiteralPath $GamePath).Path
if (Get-Process -Name GTA5_Enhanced -ErrorAction SilentlyContinue) { throw 'Close GTA before removing the adapter' }
$manifestPath = Join-Path $GamePath 'GameVerse.install.json'
if (-not (Test-Path -LiteralPath $manifestPath -PathType Leaf)) { throw 'GameVerse.install.json not found' }
$manifest = @(Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json)
if ($manifest.Count -eq 0) { throw 'Installation manifest is empty' }

$validated = [Collections.Generic.List[string]]::new()
foreach ($item in $manifest) {
    if (-not $item.RelativePath -or $item.RelativePath -match '(^|[\\/])\.\.([\\/]|$)' -or [IO.Path]::IsPathRooted([string]$item.RelativePath)) {
        throw 'Installation manifest contains an unsafe relative path'
    }
    $target = [IO.Path]::GetFullPath((Join-Path $GamePath ([string]$item.RelativePath)))
    $root = $GamePath.TrimEnd([IO.Path]::DirectorySeparatorChar) + [IO.Path]::DirectorySeparatorChar
    if (-not $target.StartsWith($root,[StringComparison]::OrdinalIgnoreCase)) { throw 'Installation manifest points outside the game directory' }
    if (-not (Test-Path -LiteralPath $target -PathType Leaf)) { throw "Installed file is missing: $($item.RelativePath)" }
    if ((Get-FileHash -LiteralPath $target).Hash -ne [string]$item.SHA256) { throw "Installed file changed; refusing removal: $($item.RelativePath)" }
    $validated.Add($target)
}

# All entries are validated before anything is removed.
foreach ($target in $validated) { Remove-Item -LiteralPath $target -Force }
Remove-Item -LiteralPath $manifestPath -Force
$scriptsPath = Join-Path $GamePath 'scripts'
if ((Test-Path -LiteralPath $scriptsPath) -and -not (Get-ChildItem -LiteralPath $scriptsPath -Force)) {
    Remove-Item -LiteralPath $scriptsPath
}
Write-Output 'Removed the files recorded by GameVerse.install.json.'
