param(
    [int]$Seconds = 30,
    [string]$TargetDir = $env:CARGO_TARGET_DIR,
    [string]$Dotnet = 'dotnet',
    [string]$Output = '.m1/smoke',
    [int]$Port = 30131,
    [ValidateSet('net48','net8.0')][string]$Framework = 'net48'
)
$ErrorActionPreference = 'Stop'
if ($Seconds -lt 20) { throw 'At least 20 seconds required' }
$repo = Split-Path $PSScriptRoot -Parent
if (-not $TargetDir) { $TargetDir = Join-Path $repo 'target' }
$Output = [IO.Path]::GetFullPath((Join-Path $repo $Output))
New-Item -ItemType Directory -Path $Output -Force | Out-Null
$identity = Join-Path $repo ('.m1/identity-' + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $identity -Force | Out-Null
$cert = Join-Path $identity 'cert.der'; $key = Join-Path $identity 'key.der'
$bin = Join-Path $identity 'bin'
New-Item -ItemType Directory -Path $bin -Force | Out-Null
foreach ($exe in @('gameverse-presence-server.exe','gameverse-gta-bridge.exe','gameverse-presence-bot.exe')) {
    Copy-Item -LiteralPath (Join-Path $TargetDir "debug/$exe") -Destination $bin
}
$harnessSource=Join-Path $repo "adapters/gta5/tests/bin/Release/$Framework"
$harnessDir=Join-Path $identity 'harness'
Copy-Item -LiteralPath $harnessSource -Destination $harnessDir -Recurse
$harness = Join-Path $harnessDir 'GameVerse.AdapterHarness.dll'
$pipeName = 'gameverse-m1-test-' + [guid]::NewGuid().ToString('N')
$address = '127.0.0.1:' + $Port
$children = [Collections.Generic.List[Diagnostics.Process]]::new()
function Launch([string]$Name, [string]$Exe, [string[]]$Arguments) {
    # Start-Process joins arguments on Windows; quote every path/argument explicitly.
    $quoted = $Arguments | ForEach-Object { '"' + $_.Replace('"','\"') + '"' }
    $p = Start-Process -FilePath $Exe -ArgumentList $quoted -WorkingDirectory $repo -WindowStyle Hidden -PassThru -RedirectStandardOutput (Join-Path $Output "$Name.log") -RedirectStandardError (Join-Path $Output "$Name.stderr.log")
    $children.Add($p); return $p
}
try {
    & (Join-Path $bin 'gameverse-presence-server.exe') --cert $cert --key $key --init-identity
    if ($LASTEXITCODE -ne 0) { throw 'Identity generation failed' }
    $server = Launch 'server' (Join-Path $bin 'gameverse-presence-server.exe') @('--bind',$address,'--cert',$cert,'--key',$key,'--duration',($Seconds+8).ToString())
    Start-Sleep -Milliseconds 400
    $bridge = Launch 'bridge' (Join-Path $bin 'gameverse-gta-bridge.exe') @('--server',$address,'--cert',$cert,'--pipe',("\\.\pipe\"+$pipeName),'--duration',($Seconds+4).ToString())
    $adapterArgs=@('--pipe',$pipeName,'--duration',$Seconds.ToString(),'--reconnect-after',([int]($Seconds/3)).ToString(),'--stall-after',([int]($Seconds/2)).ToString(),'--report',(Join-Path $Output 'adapter.json'))
    if ($Framework -eq 'net48') { $adapter=Launch 'adapter-harness' ([IO.Path]::ChangeExtension($harness,'.exe')) $adapterArgs }
    else { $adapter=Launch 'adapter-harness' $Dotnet (@($harness)+$adapterArgs) }
    Start-Sleep -Milliseconds 700
    $bot = Launch 'bot' (Join-Path $bin 'gameverse-presence-bot.exe') @('--server',$address,'--cert',$cert,'--duration',($Seconds-4).ToString(),'--report',(Join-Path $Output 'bot.json'))
    $timer = [Diagnostics.Stopwatch]::StartNew()
    while (-not $server.HasExited) {
        Start-Sleep -Seconds 1
        if ([int]$timer.Elapsed.TotalSeconds % 30 -eq 0) { Write-Output ("M1 synthetic pipeline elapsed {0:n0}s / {1}s" -f $timer.Elapsed.TotalSeconds,$Seconds) }
        if ($timer.Elapsed.TotalSeconds -gt $Seconds+30) { throw 'Scenario timed out' }
    }
    foreach ($p in $children) { $p.WaitForExit(); if ($p.ExitCode -ne 0) { throw "Child process $($p.Id) failed: $($p.ExitCode)" } }
    $a=Get-Content (Join-Path $Output 'adapter.json') -Raw | ConvertFrom-Json
    $b=Get-Content (Join-Path $Output 'bot.json') -Raw | ConvertFrom-Json
    $metrics=(Get-Content (Join-Path $Output 'server.log') | Select-Object -Last 1) | ConvertFrom-Json
    if ($a.sessions.Count -lt 2 -or $a.activations -lt 2 -or $a.creates -lt 2 -or $a.updates -lt 20 -or $a.destroys -lt 1 -or $b.received_remote_states -lt 10 -or $metrics.players -ne 0 -or $metrics.max_input_depth -gt 128) { throw 'Acceptance checks failed' }
    [ordered]@{ backend='synthetic-adapter-harness'; gta_loaded=$false; seconds=$Seconds; passed=$true; csharp_named_pipe_rust_quic=$true; reconnect=$true; remote_create_update_destroy=$true; bidirectional_3d_state=$true; metrics=$metrics } | ConvertTo-Json -Depth 8 | Set-Content (Join-Path $Output 'acceptance.json')
    Get-Content (Join-Path $Output 'acceptance.json')
}
finally { foreach ($p in $children) { if (-not $p.HasExited) { Stop-Process -Id $p.Id } } }
