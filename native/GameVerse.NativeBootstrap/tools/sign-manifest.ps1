param(
    [Parameter(Mandatory)][string]$ManifestPath,
    [Parameter(Mandatory)][string]$PrivateKeyPath,
    [Parameter(Mandatory)][string]$PublicHeaderPath,
    [switch]$GenerateKey
)
$ErrorActionPreference = 'Stop'
$manifest = (Resolve-Path -LiteralPath $ManifestPath).Path
$signature = [IO.Path]::ChangeExtension($manifest, '.sig')
$private = [IO.Path]::GetFullPath($PrivateKeyPath)
$header = [IO.Path]::GetFullPath($PublicHeaderPath)
if ($private.StartsWith((Resolve-Path (Join-Path $PSScriptRoot '..\..\..')).Path, [StringComparison]::OrdinalIgnoreCase)) {
    throw 'Private key must remain outside the repository'
}
$key = [Security.Cryptography.ECDsa]::Create([Security.Cryptography.ECCurve+NamedCurves]::nistP256)
try {
    if ($GenerateKey) {
        if (Test-Path -LiteralPath $private) { throw 'Refusing to replace the existing private key' }
        New-Item -ItemType Directory -Path ([IO.Path]::GetDirectoryName($private)) -Force | Out-Null
        [IO.File]::WriteAllText($private, $key.ExportPkcs8PrivateKeyPem())
    } else {
        if (-not (Test-Path -LiteralPath $private)) { throw 'Private signing key is missing' }
        $key.ImportFromPem([IO.File]::ReadAllText($private))
    }
    if ($key.KeySize -ne 256) { throw 'Manifest key must use ECDSA P-256' }
    $bytes = [IO.File]::ReadAllBytes($manifest)
    $signed = $key.SignData($bytes, [Security.Cryptography.HashAlgorithmName]::SHA256,
        [Security.Cryptography.DSASignatureFormat]::IeeeP1363FixedFieldConcatenation)
    if ($signed.Length -ne 64) { throw 'Unexpected signature format' }
    [IO.File]::WriteAllBytes($signature, $signed)
    $parameters = $key.ExportParameters($false)
    $coordinates = @($parameters.Q.X) + @($parameters.Q.Y)
    $values = ($coordinates | ForEach-Object { '0x{0:X2}' -f $_ }) -join ', '
    $source = "#pragma once`n#include <array>`n#include <cstdint>`nnamespace gameverse { inline constexpr std::array<std::uint8_t, 64> kManifestPublicKey{$values}; }`n"
    [IO.File]::WriteAllText($header, $source, [Text.UTF8Encoding]::new($false))
    Write-Output (@{ status='signed'; manifest=[IO.Path]::GetFileName($manifest); algorithm='ECDSA_P256_SHA256' } | ConvertTo-Json -Compress)
} finally {
    $key.Dispose()
}
