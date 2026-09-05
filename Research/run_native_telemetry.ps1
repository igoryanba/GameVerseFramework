param(
    [Parameter(Mandatory = $true)]
    [string]$OutputPath,

    [ValidateRange(10, 1200)]
    [int]$TimeoutSeconds = 900
)

$ErrorActionPreference = 'Stop'
$maximumFrame = 64KB
$pipeName = 'gameverse-bootstrap-v1'
$utf8 = [System.Text.UTF8Encoding]::new($false, $true)
$outputFile = [System.IO.Path]::GetFullPath($OutputPath)
$outputDirectory = [System.IO.Path]::GetDirectoryName($outputFile)
[System.IO.Directory]::CreateDirectory($outputDirectory) | Out-Null

function Read-Exact {
    param(
        [Parameter(Mandatory = $true)]
        [System.IO.Stream]$Stream,
        [Parameter(Mandatory = $true)]
        [byte[]]$Buffer,
        [Parameter(Mandatory = $true)]
        [System.Threading.CancellationToken]$CancellationToken
    )

    $offset = 0
    while ($offset -lt $Buffer.Length) {
        $read = $Stream.ReadAsync(
            $Buffer,
            $offset,
            $Buffer.Length - $offset,
            $CancellationToken
        ).GetAwaiter().GetResult()
        if ($read -eq 0) {
            throw 'bootstrap_pipe_closed'
        }
        $offset += $read
    }
}

function Read-Frame {
    param(
        [Parameter(Mandatory = $true)][System.IO.Stream]$Stream,
        [Parameter(Mandatory = $true)]
        [System.Threading.CancellationToken]$CancellationToken
    )

    $prefix = [byte[]]::new(4)
    Read-Exact -Stream $Stream -Buffer $prefix -CancellationToken $CancellationToken
    $length = ([uint32]$prefix[0] -shl 24) -bor
              ([uint32]$prefix[1] -shl 16) -bor
              ([uint32]$prefix[2] -shl 8) -bor
              [uint32]$prefix[3]
    if ($length -eq 0 -or $length -gt $maximumFrame) {
        throw "invalid_bootstrap_frame_length:$length"
    }
    $body = [byte[]]::new($length)
    Read-Exact -Stream $Stream -Buffer $body -CancellationToken $CancellationToken
    return $utf8.GetString($body)
}

function Write-Frame {
    param(
        [Parameter(Mandatory = $true)][System.IO.Stream]$Stream,
        [Parameter(Mandatory = $true)][string]$Json
    )

    $body = $utf8.GetBytes($Json)
    if ($body.Length -eq 0 -or $body.Length -gt $maximumFrame) {
        throw "invalid_outbound_frame_length:$($body.Length)"
    }
    $prefix = [byte[]]@(
        (($body.Length -shr 24) -band 0xff),
        (($body.Length -shr 16) -band 0xff),
        (($body.Length -shr 8) -band 0xff),
        ($body.Length -band 0xff)
    )
    $Stream.Write($prefix, 0, $prefix.Length)
    $Stream.Write($body, 0, $body.Length)
    $Stream.Flush()
}

$pipe = [System.IO.Pipes.NamedPipeServerStream]::new(
    $pipeName,
    [System.IO.Pipes.PipeDirection]::InOut,
    1,
    [System.IO.Pipes.PipeTransmissionMode]::Byte,
    [System.IO.Pipes.PipeOptions]::Asynchronous,
    $maximumFrame,
    $maximumFrame
)
$writer = $null
$cancellation = [System.Threading.CancellationTokenSource]::new(
    [TimeSpan]::FromSeconds($TimeoutSeconds)
)

try {
    $writer = [System.IO.StreamWriter]::new($outputFile, $false, $utf8)
    $writer.AutoFlush = $true
    $pipe.WaitForConnectionAsync($cancellation.Token).GetAwaiter().GetResult()

    $telemetryStarted = $false
    while ($pipe.IsConnected -and -not $cancellation.IsCancellationRequested) {
        $json = Read-Frame -Stream $pipe -CancellationToken $cancellation.Token
        $message = $json | ConvertFrom-Json -Depth 32
        $writer.WriteLine($json)

        if ($message.type -eq 'bootstrap_hello' -and -not $telemetryStarted) {
            if ($message.schema_version -ne 1 -or $message.capabilities -notcontains 'telemetry') {
                throw 'incompatible_bootstrap_hello'
            }
            Write-Frame -Stream $pipe -Json '{"type":"bootstrap_command","schema_version":1,"command":"start_telemetry"}'
            $telemetryStarted = $true
        }

        if ($message.type -eq 'bootstrap_failure' -or
            ($message.type -eq 'bootstrap_stage' -and $message.stage -eq 'adapter_ready')) {
            break
        }
    }
}
catch [System.OperationCanceledException] {
    exit 2
}
finally {
    if ($null -ne $writer) { $writer.Dispose() }
    $pipe.Dispose()
    $cancellation.Dispose()
}
