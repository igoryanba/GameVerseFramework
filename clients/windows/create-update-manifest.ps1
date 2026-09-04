param(
    [Parameter(Mandatory=$true)][string]$PackageRoot,
    [Parameter(Mandatory=$true)][string]$BaseUrl,
    [Parameter(Mandatory=$true)][string]$Version,
    [Parameter(Mandatory=$true)][string]$MinimumLauncherVersion,
    [Parameter(Mandatory=$true)][string]$SigningKeyPath,
    [string]$Channel = 'alpha',
    [string]$KeyId = 'gameverse-alpha-1'
)
$ErrorActionPreference = 'Stop'
$root = (Resolve-Path -LiteralPath $PackageRoot).Path
$signingKey = (Resolve-Path -LiteralPath $SigningKeyPath).Path
$rootPrefix = $root.TrimEnd([IO.Path]::DirectorySeparatorChar) + [IO.Path]::DirectorySeparatorChar
if ($signingKey.StartsWith($rootPrefix, [StringComparison]::OrdinalIgnoreCase)) { throw 'Signing key must be stored outside the package' }
$base = [Uri]::new($BaseUrl)
if ($base.Scheme -ne 'https') { throw 'BaseUrl must use HTTPS' }
if (-not (Test-Path -LiteralPath $SigningKeyPath -PathType Leaf)) { throw 'Signing key does not exist' }

$files = @(Get-ChildItem -LiteralPath $root -File -Recurse |
    Where-Object { $_.Name -notin @('update-manifest.json','update-manifest.sig') } |
    Sort-Object FullName |
    ForEach-Object {
        $relative = [IO.Path]::GetRelativePath($root, $_.FullName).Replace('\','/')
        [ordered]@{
            path = $relative
            size = $_.Length
            sha256 = (Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
            url = [Uri]::new($base, $relative).AbsoluteUri
        }
    })
if ($files.Count -eq 0) { throw 'Package does not contain update files' }

$manifest = [ordered]@{
    schema_version = 1
    version = $Version
    channel = $Channel
    minimum_launcher_version = $MinimumLauncherVersion
    signature = [ordered]@{ algorithm = 'ECDSA_P256_SHA256'; key_id = $KeyId }
    files = @($files)
}
$manifestPath = Join-Path $root 'update-manifest.json'
$signaturePath = Join-Path $root 'update-manifest.sig'
$manifest | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath $manifestPath -Encoding utf8NoBOM
$bytes = [IO.File]::ReadAllBytes($manifestPath)
$key = [Security.Cryptography.ECDsa]::Create()
try {
    $key.ImportFromPem([IO.File]::ReadAllText($signingKey))
    if ($key.KeySize -ne 256) { throw 'Signing key must use ECDSA P-256' }
    $signature = $key.SignData($bytes, [Security.Cryptography.HashAlgorithmName]::SHA256)
    [IO.File]::WriteAllBytes($signaturePath, $signature)
} finally {
    $key.Dispose()
}
Write-Output (@{ status='signed'; manifest=$manifestPath; signature=$signaturePath; files=$files.Count } | ConvertTo-Json -Compress)
