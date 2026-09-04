using System;
using System.Collections.Concurrent;
using System.Diagnostics;
using System.IO;
using System.IO.Pipes;
using System.Threading;
using System.Threading.Tasks;
using Newtonsoft.Json.Linq;

namespace GameVerse.AdapterProtocol
{
    // Background I/O only. No GTA object or native call is allowed in this class.
    public sealed class PipeLink : IDisposable
    {
        public readonly BlockingCollection<JObject> Commands = new BlockingCollection<JObject>(128);
        private readonly BlockingCollection<JObject> reports = new BlockingCollection<JObject>(64);
        private readonly CancellationTokenSource stop = new CancellationTokenSource();
        private readonly object gate = new object();
        private readonly Action<string> log;
        private readonly string pipeName, build, backend;
        private PlayerState latest;
        private NamedPipeClientStream pipe;
        private Task worker;
        private long receivedAt;
        private bool gameReady;
        private long sampledAt;
        public bool Connected { get; private set; }
        public PipeLink(Action<string> log, string build, string backend = "shvdne-1.1.0.6", string pipeName = Wire.PipeName)
        { this.log = log; this.build = build; this.backend = backend; this.pipeName = pipeName; }
        public void Start() { worker = Task.Run(Run); }
        public void Publish(PlayerState state, bool ready) { lock (gate) { latest = state; gameReady = ready; sampledAt = Stopwatch.GetTimestamp(); } }
        // A live I/O thread must not keep a stalled game thread's pose alive forever.
        private bool FreshGame() => gameReady && (Stopwatch.GetTimestamp() - sampledAt) / (double)Stopwatch.Frequency < 2;
        public void Report(string name, EntityId id = null)
        {
            var message = Wire.Message("adapter_status"); message["event"] = name; message["id"] = id == null ? JValue.CreateNull() : JToken.FromObject(id);
            reports.TryAdd(message);
        }
        public void ReportBootstrapFailure(string code, string message)
        {
            var report = Wire.Message("bootstrap_failure"); report["code"] = code; report["message"] = message;
            reports.TryAdd(report);
        }
        private async Task Run()
        {
            while (!stop.IsCancellationRequested)
            {
                try
                {
                    bool ready; lock (gate) ready = FreshGame();
                    if (!ready) { await Task.Delay(250, stop.Token); continue; }
                    using (var stream = new NamedPipeClientStream(".", pipeName, PipeDirection.InOut, PipeOptions.Asynchronous))
                    using (var session = CancellationTokenSource.CreateLinkedTokenSource(stop.Token))
                    {
                        lock (gate) pipe = stream;
                        await stream.ConnectAsync(1500, stop.Token).ConfigureAwait(false);
                        var hello = Wire.Message("adapter_hello"); hello["version"] = Wire.Version; hello["backend"] = backend;
                        await Wire.Write(stream, hello, session.Token);
                        var info = Wire.Message("game_info"); info["edition"] = "enhanced"; info["build"] = build;
                        await Wire.Write(stream, info, session.Token);
                        Interlocked.Exchange(ref receivedAt, Stopwatch.GetTimestamp()); Connected = true; log("IPC_CONNECTED=true");
                        Task read = Receive(stream, session.Token), write = Send(stream, session.Token);
                        await Task.WhenAny(read, write).ConfigureAwait(false);
                        session.Cancel(); stream.Dispose();
                        try { await Task.WhenAll(read, write).ConfigureAwait(false); } catch (Exception e) { log("IPC_SESSION_END " + e.Message); }
                    }
                }
                catch (Exception e) { if (!stop.IsCancellationRequested) log("IPC_RETRY " + e.Message); }
                finally
                {
                    Connected = false; lock (gate) pipe = null;
                    // Drop stale commands before resetting, so a delayed create cannot resurrect a ped.
                    while (Commands.TryTake(out _)) { }
                    Commands.TryAdd(Wire.Message("reset"));
                    while (reports.TryTake(out _)) { }
                }
                try { await Task.Delay(1000, stop.Token).ConfigureAwait(false); } catch (OperationCanceledException) { }
            }
        }
        private async Task Receive(Stream stream, CancellationToken cancel)
        {
            while (!cancel.IsCancellationRequested)
            {
                JObject message = await Wire.Read(stream, cancel).ConfigureAwait(false);
                Interlocked.Exchange(ref receivedAt, Stopwatch.GetTimestamp());
                if ((string)message["type"] == "adapter_heartbeat") continue;
                if (!Commands.TryAdd(message)) throw new IOException("Command queue overflow");
            }
        }
        private async Task Send(Stream stream, CancellationToken cancel)
        {
            ulong sequence = 0; int count = 0;
            while (!cancel.IsCancellationRequested)
            {
                if ((Stopwatch.GetTimestamp() - Interlocked.Read(ref receivedAt)) / (double)Stopwatch.Frequency > 5) throw new IOException("Bridge heartbeat timeout");
                PlayerState state; bool ready; lock (gate) { state = latest; ready = FreshGame(); }
                if (state != null && ready)
                {
                    var message = Wire.Message("local_player_state"); message["sequence"] = ++sequence; message["state"] = JToken.FromObject(state);
                    await Wire.Write(stream, message, cancel).ConfigureAwait(false);
                }
                for (int i = 0; i < 8 && reports.TryTake(out JObject report); i++) await Wire.Write(stream, report, cancel).ConfigureAwait(false);
                if (count++ % 20 == 0)
                {
                    var heartbeat = Wire.Message("adapter_heartbeat"); heartbeat["game_ready"] = ready;
                    await Wire.Write(stream, heartbeat, cancel).ConfigureAwait(false);
                }
                await Task.Delay(50, cancel).ConfigureAwait(false);
            }
        }
        public void Dispose()
        {
            stop.Cancel(); lock (gate) { pipe?.Dispose(); }
            // Script abort must not block the game thread waiting for network shutdown.
        }
    }
}
