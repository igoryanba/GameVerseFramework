using System.Buffers.Binary;
using System.Collections.Concurrent;
using System.IO.Pipes;
using System.Text;
using System.Text.Json;

internal sealed record UiRequest(int SchemaVersion, string RequestId, string Command, JsonElement Payload);

internal sealed record UiResponse(
    int SchemaVersion,
    string RequestId,
    bool Ok,
    string? ErrorCode,
    string? Message,
    JsonElement Payload);

internal static class UiJson
{
    internal static readonly JsonSerializerOptions Options = new(JsonSerializerDefaults.Web)
    {
        PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower
    };
}

internal sealed class UiBridgeClient : IAsyncDisposable
{
    internal const int MaxMessageBytes = 64 * 1024;
    private readonly string pipeName;
    private readonly ConcurrentDictionary<string, TaskCompletionSource<UiResponse>> pending = new();
    private readonly SemaphoreSlim writeLock = new(1, 1);
    private readonly CancellationTokenSource stopping = new();
    private NamedPipeClientStream? stream;
    private Task? reader;

    internal bool Connected => stream?.IsConnected == true;
    internal event Action? Disconnected;
    internal event Action<UiResponse>? BridgeEvent;

    internal UiBridgeClient(string pipe) => pipeName = NormalizePipe(pipe);

    internal async Task ConnectWithRetryAsync(TimeSpan timeout)
    {
        using CancellationTokenSource deadline = CancellationTokenSource.CreateLinkedTokenSource(stopping.Token);
        deadline.CancelAfter(timeout);
        Exception? last = null;
        while (!deadline.IsCancellationRequested)
        {
            NamedPipeClientStream candidate = new(".", pipeName, PipeDirection.InOut, PipeOptions.Asynchronous);
            try
            {
                await candidate.ConnectAsync(1000, deadline.Token);
                stream = candidate;
                reader = ReadLoopAsync(candidate, stopping.Token);
                UiResponse hello = await SendAsync(Request("ui.hello", new { ui_build = Application.ProductVersion }), deadline.Token);
                if (!hello.Ok) throw new InvalidDataException(hello.Message ?? "Bridge rejected launcher handshake");
                return;
            }
            catch (Exception error) when (error is IOException or TimeoutException or OperationCanceledException or InvalidDataException)
            {
                last = error;
                candidate.Dispose();
                if (!deadline.IsCancellationRequested) await Task.Delay(250, deadline.Token);
            }
        }
        throw new TimeoutException("Не удалось подключиться к GameVerse bridge", last);
    }

    internal async Task<UiResponse> SendAsync(
        UiRequest request,
        CancellationToken cancellationToken = default,
        TimeSpan? responseTimeout = null)
    {
        NamedPipeClientStream target = stream is { IsConnected: true } value
            ? value
            : throw new IOException("Bridge ещё не подключён");
        TaskCompletionSource<UiResponse> completion = new(TaskCreationOptions.RunContinuationsAsynchronously);
        if (!pending.TryAdd(request.RequestId, completion)) throw new InvalidOperationException("Duplicate request ID");
        try
        {
            byte[] body = JsonSerializer.SerializeToUtf8Bytes(request, UiJson.Options);
            if (body.Length is 0 or > MaxMessageBytes) throw new InvalidDataException("UI message is too large");
            byte[] prefix = new byte[4];
            BinaryPrimitives.WriteUInt32BigEndian(prefix, (uint)body.Length);
            await writeLock.WaitAsync(cancellationToken);
            try
            {
                await target.WriteAsync(prefix, cancellationToken);
                await target.WriteAsync(body, cancellationToken);
                await target.FlushAsync(cancellationToken);
            }
            finally { writeLock.Release(); }
            return await completion.Task.WaitAsync(responseTimeout ?? TimeSpan.FromSeconds(8), cancellationToken);
        }
        finally { pending.TryRemove(request.RequestId, out _); }
    }

    internal static UiRequest Request(string command, object? payload = null) => new(
        1,
        Guid.NewGuid().ToString("N"),
        command,
        JsonSerializer.SerializeToElement(payload ?? new { }, UiJson.Options));

    private async Task ReadLoopAsync(Stream source, CancellationToken cancellationToken)
    {
        try
        {
            byte[] prefix = new byte[4];
            while (!cancellationToken.IsCancellationRequested)
            {
                await source.ReadExactlyAsync(prefix, cancellationToken);
                uint length = BinaryPrimitives.ReadUInt32BigEndian(prefix);
                if (length is 0 or > MaxMessageBytes) throw new InvalidDataException("Invalid bridge frame length");
                byte[] body = new byte[length];
                await source.ReadExactlyAsync(body, cancellationToken);
                UiResponse response = JsonSerializer.Deserialize<UiResponse>(body, UiJson.Options)
                    ?? throw new InvalidDataException("Empty bridge response");
                if (pending.TryRemove(response.RequestId, out TaskCompletionSource<UiResponse>? completion))
                    completion.TrySetResult(response);
                else if (response.RequestId == "bridge-stage")
                    BridgeEvent?.Invoke(response);
            }
        }
        catch (Exception error) when (error is IOException or EndOfStreamException or OperationCanceledException or InvalidDataException or JsonException)
        {
            foreach (TaskCompletionSource<UiResponse> completion in pending.Values)
                completion.TrySetException(new IOException("Связь с GameVerse bridge потеряна", error));
            pending.Clear();
            if (!stopping.IsCancellationRequested) Disconnected?.Invoke();
        }
    }

    public async ValueTask DisposeAsync()
    {
        stopping.Cancel();
        stream?.Dispose();
        if (reader is not null)
            try { await reader; } catch (OperationCanceledException) { }
        writeLock.Dispose();
        stopping.Dispose();
    }

    private static string NormalizePipe(string pipe)
    {
        const string prefix = @"\\.\pipe\";
        string value = pipe.StartsWith(prefix, StringComparison.OrdinalIgnoreCase) ? pipe[prefix.Length..] : pipe;
        if (string.IsNullOrWhiteSpace(value) || value.Contains('\\') || value.Contains('/'))
            throw new ArgumentException("Invalid UI pipe name", nameof(pipe));
        return value;
    }
}

internal static class TokenStore
{
    private static readonly byte[] Entropy = Encoding.UTF8.GetBytes("GameVerse.UI.RefreshToken.v1");
    private static readonly string TokenPath = Path.Combine(
        Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "GameVerse", "refresh-token.bin");

    internal static bool Exists => File.Exists(TokenPath);
    internal static void Save(string token)
    {
        Directory.CreateDirectory(Path.GetDirectoryName(TokenPath)!);
        byte[] protectedToken = System.Security.Cryptography.ProtectedData.Protect(
            Encoding.UTF8.GetBytes(token), Entropy, System.Security.Cryptography.DataProtectionScope.CurrentUser);
        File.WriteAllBytes(TokenPath, protectedToken);
    }
    internal static string? Load()
    {
        if (!File.Exists(TokenPath)) return null;
        byte[] plaintext = System.Security.Cryptography.ProtectedData.Unprotect(
            File.ReadAllBytes(TokenPath), Entropy, System.Security.Cryptography.DataProtectionScope.CurrentUser);
        return Encoding.UTF8.GetString(plaintext);
    }
    internal static void Clear() { if (File.Exists(TokenPath)) File.Delete(TokenPath); }
}
