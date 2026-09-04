[CmdletBinding()]
param([string]$Destination = ".research/upstreams")
$ErrorActionPreference = "Stop"
$root = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$target = [IO.Path]::GetFullPath((Join-Path $root $Destination))
if (-not $target.StartsWith($root + [IO.Path]::DirectorySeparatorChar, [StringComparison]::OrdinalIgnoreCase)) { throw "Destination must stay inside the repository" }
$sources = @(
    @{ Name="ragecoop"; Url="https://github.com/RAGECOOP/RAGECOOP-V.git"; Commit="fcd7e18d9b14c7cda95783e5a7ade4b4a20f97d2"; License="MIT" },
    @{ Name="gta-network"; Url="https://github.com/GTANetworkDev/platform.git"; Commit="f0ee9f391a0ec9a557c32df549baa6cde4ba0f53"; License="MIT" }
)
New-Item -ItemType Directory -Force -Path $target | Out-Null
foreach ($source in $sources) {
    $path = Join-Path $target $source.Name
    if (-not (Test-Path (Join-Path $path ".git"))) { git clone --filter=blob:none --no-checkout $source.Url $path }
    git -C $path fetch --depth 1 origin $source.Commit
    git -C $path checkout --detach $source.Commit
    $actual = (git -C $path rev-parse HEAD).Trim()
    if ($actual -ne $source.Commit) { throw "Revision mismatch for $($source.Name)" }
    [pscustomobject]@{ name=$source.Name; commit=$actual; license=$source.License; path=$path }
}
