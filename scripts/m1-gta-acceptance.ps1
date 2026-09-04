param(
    [Parameter(Mandatory)][string]$GamePath,
    [int]$Seconds = 300,
    [string]$TargetDir = $env:CARGO_TARGET_DIR,
    [string]$Output = '.m1/gta-acceptance',
    [int]$Port = 30121,
    [ValidateSet('PlayGTAV.exe','GTA5_Enhanced.exe')][string]$Launcher = 'PlayGTAV.exe',
    [switch]$LeaveGameRunning
)

$ErrorActionPreference = 'Stop'
if ($Seconds -lt 30) { throw 'At least 30 seconds required' }

$repo = Split-Path $PSScriptRoot -Parent
$GamePath = (Resolve-Path -LiteralPath $GamePath).Path
$gameExe = Join-Path $GamePath 'GTA5_Enhanced.exe'
if (-not (Test-Path -LiteralPath $gameExe -PathType Leaf)) { throw 'GTA5_Enhanced.exe not found' }
$gameFile = Get-Item -LiteralPath $gameExe
if ($gameFile.VersionInfo.FileVersion -ne '1.0.1158.13') { throw 'Only Enhanced 1.0.1158.13 is tested' }
$launcherExe = Join-Path $GamePath $Launcher
if (-not (Test-Path -LiteralPath $launcherExe -PathType Leaf)) { throw "Launcher not found: $Launcher" }
if (Get-Process -Name GTA5_Enhanced -ErrorAction SilentlyContinue) { throw 'Close the existing GTA5_Enhanced process before the test' }
$memory = Get-CimInstance Win32_OperatingSystem
if ($memory.TotalVirtualMemorySize * 1KB -lt 16GB) {
    Write-Warning 'Windows commit limit is below 16 GB. GTA Enhanced may fail with DirectX Out of memory.'
}

if (-not $TargetDir) { $TargetDir = Join-Path $repo 'target' }
$binaries = @{
    server = Join-Path $TargetDir 'debug/gameverse-presence-server.exe'
    bridge = Join-Path $TargetDir 'debug/gameverse-gta-bridge.exe'
    bot = Join-Path $TargetDir 'debug/gameverse-presence-bot.exe'
}
foreach ($entry in $binaries.GetEnumerator()) {
    if (-not (Test-Path -LiteralPath $entry.Value -PathType Leaf)) { throw "Missing binary: $($entry.Value)" }
}

$requiredHostFiles = @(
    'ScriptHookV.dll', 'xinput1_4.dll', 'ScriptHookVDotNet.asi',
    'ScriptHookVDotNet2.dll', 'ScriptHookVDotNet3.dll', 'MinHook.x64.dll',
    'scripts/GameVerse.GtaAdapter.dll', 'scripts/Newtonsoft.Json.dll'
)
$missingHostFiles = @($requiredHostFiles | Where-Object { -not (Test-Path -LiteralPath (Join-Path $GamePath $_) -PathType Leaf) })
if ($missingHostFiles.Count -gt 0) {
    throw ('Adapter is not installed. Missing: ' + ($missingHostFiles -join ', '))
}

$Output = [IO.Path]::GetFullPath((Join-Path $repo $Output))
New-Item -ItemType Directory -Path $Output -Force | Out-Null
$identity = Join-Path $Output 'identity'
New-Item -ItemType Directory -Path $identity -Force | Out-Null
$cert = Join-Path $identity 'cert.der'
$key = Join-Path $identity 'key.der'
$address = '127.0.0.1:' + $Port
$adapterLog = Join-Path $GamePath 'GameVerse.GtaAdapter.log'
$startedUtc = [DateTime]::UtcNow
$children = [Collections.Generic.List[Diagnostics.Process]]::new()
$game = $null
$launcherProcess = $null
$failure = $null

function Start-LoggedProcess([string]$Name, [string]$FilePath, [string[]]$Arguments) {
    $quoted = $Arguments | ForEach-Object { '"' + $_.Replace('"','\"') + '"' }
    $process = Start-Process -FilePath $FilePath -ArgumentList $quoted -WorkingDirectory $repo -WindowStyle Hidden -PassThru `
        -RedirectStandardOutput (Join-Path $Output "$Name.log") `
        -RedirectStandardError (Join-Path $Output "$Name.stderr.log")
    $children.Add($process)
    return $process
}

function Read-NewAdapterLog {
    if (-not (Test-Path -LiteralPath $adapterLog)) { return @() }
    return @(Get-Content -LiteralPath $adapterLog | Where-Object {
        if ($_ -notmatch '^(?<stamp>\S+) ') { return $false }
        try { [DateTime]::Parse($Matches.stamp).ToUniversalTime() -ge $startedUtc } catch { $false }
    })
}

try {
    if (-not (Test-Path -LiteralPath $cert) -or -not (Test-Path -LiteralPath $key)) {
        & $binaries.server --cert $cert --key $key --init-identity
        if ($LASTEXITCODE -ne 0) { throw 'Identity generation failed' }
    }

    $server = Start-LoggedProcess 'server' $binaries.server @('--bind',$address,'--cert',$cert,'--key',$key,'--duration',($Seconds+15).ToString())
    Start-Sleep -Milliseconds 500
    $bridge = Start-LoggedProcess 'bridge' $binaries.bridge @('--server',$address,'--cert',$cert,'--duration',($Seconds+10).ToString())
    Start-Sleep -Milliseconds 500
    $bot = Start-LoggedProcess 'bot' $binaries.bot @('--server',$address,'--cert',$cert,'--duration',$Seconds.ToString(),'--report',(Join-Path $Output 'bot.json'))

    # This distribution starts reliably through PlayGTAV.exe. Direct executable
    # launch remains available as an explicit diagnostic option.
    $launcherProcess = Start-Process -FilePath $launcherExe -WorkingDirectory $GamePath -PassThru
    $gameDeadline = [DateTime]::UtcNow.AddSeconds(60)
    while (-not $game -and [DateTime]::UtcNow -lt $gameDeadline) {
        Start-Sleep -Milliseconds 500
        $game = Get-Process -Name GTA5_Enhanced -ErrorAction SilentlyContinue | Select-Object -First 1
    }
    if (-not $game) { throw "$Launcher did not start GTA5_Enhanced.exe within 60 seconds" }
    $timer = [Diagnostics.Stopwatch]::StartNew()
    while ($timer.Elapsed.TotalSeconds -lt $Seconds) {
        Start-Sleep -Seconds 1
        $game.Refresh()
        $newLog = Read-NewAdapterLog
        $loaded = [bool]($newLog -match 'GTA_ADAPTER_LOADED=true.+SUPPORTED=True')
        $ipc = [bool]($newLog -match 'IPC_CONNECTED=true')
        $serverState = if (Test-Path (Join-Path $Output 'server.log')) { Get-Content (Join-Path $Output 'server.log') -Tail 1 -ErrorAction SilentlyContinue } else { $null }
        $localStateSeen = [bool]($serverState -match '"players":2')
        if ($loaded -and $ipc -and $localStateSeen -and $bot.HasExited) { break }
        if ($game.HasExited) { throw "GTA5_Enhanced exited before acceptance completed: $($game.ExitCode)" }
    }
}
catch {
    $failure = $_.Exception.Message
}
finally {
    $newLog = Read-NewAdapterLog
    if ($newLog.Count -gt 0) { $newLog | Set-Content -LiteralPath (Join-Path $Output 'adapter.log') -Encoding utf8 }
    foreach ($process in $children) {
        $process.Refresh()
        if (-not $process.HasExited) { Stop-Process -Id $process.Id }
        $process.WaitForExit()
    }
    if ($game) {
        $game.Refresh()
        if (-not $LeaveGameRunning -and -not $game.HasExited) { Stop-Process -Id $game.Id; $game.WaitForExit() }
    }
    if ($launcherProcess) {
        $launcherProcess.Refresh()
        if (-not $LeaveGameRunning -and -not $launcherProcess.HasExited) { Stop-Process -Id $launcherProcess.Id }
    }
}

$adapterLines = Read-NewAdapterLog
$loaded = [bool]($adapterLines -match 'GTA_ADAPTER_LOADED=true.+SUPPORTED=True')
$ipc = [bool]($adapterLines -match 'IPC_CONNECTED=true')
$remoteCreated = [bool]($adapterLines -match 'remote_ped_created')
$botReport = if (Test-Path -LiteralPath (Join-Path $Output 'bot.json')) { Get-Content -LiteralPath (Join-Path $Output 'bot.json') -Raw | ConvertFrom-Json } else { $null }
$g1 = $loaded -and $ipc -and $botReport -and $botReport.received_remote_states -gt 0
$g3 = $botReport -and $botReport.published -gt 0
$report = [ordered]@{
    gta_executable = $gameExe
    launcher = $Launcher
    playgtav_used = $Launcher -eq 'PlayGTAV.exe'
    game_build = $gameFile.VersionInfo.FileVersion
    duration_seconds = $Seconds
    adapter_loaded = $loaded
    ipc_connected = $ipc
    g1_telemetry = [bool]$g1
    g2_remote_ped_created = $remoteCreated
    g2_visual_confirmation_required = $true
    g3_bidirectional_state = [bool]$g3
    passed_automated_checks = [bool]($g1 -and $remoteCreated -and $g3)
    failure = $failure
}
$report | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $Output 'acceptance.json') -Encoding utf8
$report | ConvertTo-Json -Depth 6
if ($failure) { throw $failure }
if (-not $report.passed_automated_checks) { throw 'GTA acceptance evidence is incomplete; inspect acceptance.json and logs' }
