param([string]$Destination = ".research/resources")
$ErrorActionPreference = "Stop"
$items = @(
    @{ Name="tj_utils"; Url="https://github.com/Tj0t1/tj_utils.git"; Commit="23b4796896d01a73a61c570d44a6a564d88ad78a"; License="GPL" },
    @{ Name="qb-smallresources"; Url="https://github.com/qbcore-framework/qb-smallresources.git"; Commit="e69a95786ef29e53d962e4e0ec57eef36e195561"; License="GPL-3.0" }
)
$destinationPath = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot "..\$Destination"))
New-Item -ItemType Directory -Path $destinationPath -Force | Out-Null
foreach ($item in $items) {
    $path = Join-Path $destinationPath $item.Name
    if (Test-Path -LiteralPath $path) { throw "Canary path already exists: $path" }
    git clone --filter=blob:none --no-checkout $item.Url $path
    git -C $path checkout --detach $item.Commit
    $hashLines = git -C $path ls-files | Sort-Object | ForEach-Object {
        $file = Join-Path $path $_
        "$_ $((Get-FileHash -Algorithm SHA256 -LiteralPath $file).Hash.ToLowerInvariant())"
    }
    $aggregate = [System.Security.Cryptography.SHA256]::HashData([Text.Encoding]::UTF8.GetBytes(($hashLines -join "`n")))
    $metadata = [ordered]@{ name=$item.Name; url=$item.Url; commit=$item.Commit; license=$item.License; sha256=[Convert]::ToHexString($aggregate).ToLowerInvariant() }
    $metadata | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $path "gameverse-canary.json") -Encoding utf8
}
