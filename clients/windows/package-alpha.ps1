param(
    [string]$Configuration = 'Release',
    [string]$Runtime = 'win-x64',
    [string]$OutputDirectory = '',
    [string]$Dotnet = 'dotnet',
    [string]$Cargo = 'cargo',
    [string]$SigningKeyPath = '',
    [string]$UpdateBaseUrl = '',
    [string]$Version = '0.1.0-alpha.1',
    [string]$MinimumLauncherVersion = '0.1.0-alpha.1'
)
$ErrorActionPreference = 'Stop'
$root = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$artifacts = if ($OutputDirectory) { [IO.Path]::GetFullPath($OutputDirectory) } else { Join-Path $root 'artifacts' }
$stage = Join-Path $artifacts 'GameVerse-alpha-win-x64'
$symbols = Join-Path $artifacts 'GameVerse-alpha-win-x64-symbols'
if (Test-Path -LiteralPath $stage) { Remove-Item -LiteralPath $stage -Recurse -Force }
if (Test-Path -LiteralPath $symbols) { Remove-Item -LiteralPath $symbols -Recurse -Force }
New-Item -ItemType Directory -Path $stage,$symbols -Force | Out-Null

function Invoke-Checked([string]$program, [string[]]$arguments) {
    & $program @arguments
    if ($LASTEXITCODE -ne 0) { throw "$program failed with exit code $LASTEXITCODE" }
}

$launcherPublish = Join-Path $artifacts 'publish-launcher'
$uiPublish = Join-Path $artifacts 'publish-ui'
Invoke-Checked $Dotnet @('publish', (Join-Path $PSScriptRoot 'GameVerse.Launcher'), '-c', $Configuration, '-r', $Runtime, '--self-contained', 'true', '-p:PublishSingleFile=true', '-p:IncludeNativeLibrariesForSelfExtract=true', '-o', $launcherPublish, '--nologo')
Invoke-Checked $Dotnet @('publish', (Join-Path $PSScriptRoot 'GameVerse.UI'), '-c', $Configuration, '-r', $Runtime, '--self-contained', 'true', '-p:PublishSingleFile=true', '-p:IncludeNativeLibrariesForSelfExtract=true', '-o', $uiPublish, '--nologo')
Invoke-Checked $Cargo @('build', '--locked', '--release', '-p', 'gameverse-client', '--bin', 'gameverse-gta-bridge-m2')
Invoke-Checked 'cmake' @('-S', (Join-Path $root 'native\GameVerse.NativeBootstrap'), '-B', (Join-Path $root '.build\native-bootstrap'), '-A', 'x64')
Invoke-Checked 'cmake' @('--build', (Join-Path $root '.build\native-bootstrap'), '--config', $Configuration)

New-Item -ItemType Directory -Path (Join-Path $stage 'ui'),(Join-Path $stage 'bridge'),(Join-Path $stage 'adapter'),(Join-Path $stage 'native'),(Join-Path $stage 'licenses') -Force | Out-Null
Copy-Item -LiteralPath (Join-Path $launcherPublish 'GameVerse.Launcher.exe') -Destination $stage
Copy-Item -Path (Join-Path $uiPublish '*') -Destination (Join-Path $stage 'ui') -Recurse -Force
Copy-Item -LiteralPath (Join-Path $root 'target\release\gameverse-gta-bridge-m2.exe') -Destination (Join-Path $stage 'bridge')
$adapterOutput = Join-Path $root "adapters\gta5\GameVerse.GtaAdapter\bin\$Configuration\net48"
if (-not (Test-Path -LiteralPath (Join-Path $adapterOutput 'GameVerse.GtaAdapter.dll'))) { throw 'Build the GTA adapter with adapters/gta5/setup.ps1 before packaging' }
Get-ChildItem -LiteralPath $adapterOutput -File | Where-Object Extension -ne '.pdb' | Copy-Item -Destination (Join-Path $stage 'adapter')
Copy-Item -LiteralPath (Join-Path $root ".build\native-bootstrap\$Configuration\GameVerse.NativeBootstrap.asi") -Destination (Join-Path $stage 'native')
Copy-Item -Path (Join-Path $root 'native\GameVerse.NativeBootstrap\compatibility\*') -Destination (Join-Path $stage 'native')
Copy-Item -LiteralPath (Join-Path $root 'native\GameVerse.NativeBootstrap\THIRD_PARTY_MINHOOK.txt') -Destination (Join-Path $stage 'licenses')
Copy-Item -LiteralPath (Join-Path $root 'adapters\gta5\THIRD_PARTY.md') -Destination (Join-Path $stage 'licenses')
Copy-Item -LiteralPath (Join-Path $root 'LICENSE') -Destination (Join-Path $stage 'licenses') -ErrorAction SilentlyContinue
Get-ChildItem -Path $launcherPublish,$uiPublish,$adapterOutput -Filter '*.pdb' -File -Recurse | Copy-Item -Destination $symbols -Force

if ($SigningKeyPath) {
    $privateKey = (Resolve-Path -LiteralPath $SigningKeyPath).Path
    $stagePrefix = $stage.TrimEnd([IO.Path]::DirectorySeparatorChar) + [IO.Path]::DirectorySeparatorChar
    if ($privateKey.StartsWith($stagePrefix, [StringComparison]::OrdinalIgnoreCase)) { throw 'Signing key must be stored outside the package' }
    $signer = [Security.Cryptography.ECDsa]::Create()
    try {
        $signer.ImportFromPem([IO.File]::ReadAllText($privateKey))
        if ($signer.KeySize -ne 256) { throw 'Signing key must use ECDSA P-256' }
        [IO.File]::WriteAllText((Join-Path $stage 'update-public-key.pem'), $signer.ExportSubjectPublicKeyInfoPem())
    } finally { $signer.Dispose() }
}

$example = [ordered]@{
    GameDirectory = 'C:\Games\Grand Theft Auto V Enhanced'
    UiPath = 'ui\GameVerse.UI.exe'
    BridgePath = 'bridge\gameverse-gta-bridge-m2.exe'
    UiPipe = '\\.\pipe\gameverse-ui-v1'
    AdapterPipe = '\\.\pipe\gameverse-gta-v1'
    BootstrapPipe = '\\.\pipe\gameverse-bootstrap-v1'
    ServerAddress = '127.0.0.1:30122'
    CertificatePath = 'server-cert.der'
    CertificateSha256 = ('0' * 64)
    UpdateChannel = 'alpha'
    LogLevel = 'info'
    RequireInstallManifest = $true
    LogDirectory = '%LOCALAPPDATA%\GameVerse\logs'
    UpdateManifestUrl = ''
    UpdateSignatureUrl = ''
    UpdatePublicKeyPath = 'update-public-key.pem'
}
$example | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $stage 'launcher.example.json') -Encoding utf8NoBOM

$files = Get-ChildItem -LiteralPath $stage -File -Recurse | Sort-Object FullName | ForEach-Object {
    [ordered]@{
        path = [IO.Path]::GetRelativePath($stage, $_.FullName).Replace('\','/')
        size = $_.Length
        sha256 = (Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
    }
}
$manifest = [ordered]@{
    schema_version = 1
    product = 'GameVerse'
    channel = 'alpha'
    runtime = $Runtime
    commit = (git -C $root rev-parse HEAD).Trim()
    generated_at = [DateTimeOffset]::UtcNow.ToString('O')
    files = @($files)
}
$manifest | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath (Join-Path $stage 'install-manifest.json') -Encoding utf8NoBOM

if ($SigningKeyPath) {
    if (-not $UpdateBaseUrl) { throw 'UpdateBaseUrl is required when SigningKeyPath is set' }
    & (Join-Path $PSScriptRoot 'create-update-manifest.ps1') -PackageRoot $stage -BaseUrl $UpdateBaseUrl -Version $Version -MinimumLauncherVersion $MinimumLauncherVersion -SigningKeyPath $SigningKeyPath
    if ($LASTEXITCODE -ne 0) { throw 'Update manifest signing failed' }
}

$clientZip = Join-Path $artifacts 'GameVerse-alpha-win-x64.zip'
$symbolsZip = Join-Path $artifacts 'GameVerse-alpha-win-x64-symbols.zip'
if (Test-Path -LiteralPath $clientZip) { Remove-Item -LiteralPath $clientZip -Force }
if (Test-Path -LiteralPath $symbolsZip) { Remove-Item -LiteralPath $symbolsZip -Force }
Compress-Archive -Path (Join-Path $stage '*') -DestinationPath $clientZip
Compress-Archive -Path (Join-Path $symbols '*') -DestinationPath $symbolsZip
Write-Output (@{ status='packaged'; client=$clientZip; symbols=$symbolsZip; files=$files.Count } | ConvertTo-Json -Compress)
