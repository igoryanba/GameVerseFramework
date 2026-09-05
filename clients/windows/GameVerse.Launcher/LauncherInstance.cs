using System.IO.Pipes;
using System.Text;

internal sealed class LauncherInstance : IDisposable
{
    private const string MutexName = @"Local\GameVerse.Launcher.Alpha";
    private const string PipeName = "gameverse-launcher-control-v1";
    private readonly Mutex mutex;
    private readonly CancellationTokenSource stopping = new();
    private Task? listener;

    private LauncherInstance(Mutex mutex) => this.mutex = mutex;

    internal static bool TryAcquire(out LauncherInstance? instance)
    {
        Mutex mutex = new(true, MutexName, out bool created);
        if (!created)
        {
            mutex.Dispose();
            instance = null;
            SignalExisting();
            return false;
        }
        instance = new LauncherInstance(mutex);
        return true;
    }

    internal void Listen(Action show)
    {
        listener = Task.Run(async () =>
        {
            while (!stopping.IsCancellationRequested)
            {
                using NamedPipeServerStream pipe = new(PipeName, PipeDirection.In, 1,
                    PipeTransmissionMode.Byte, PipeOptions.Asynchronous);
                try
                {
                    await pipe.WaitForConnectionAsync(stopping.Token);
                    byte[] data = new byte[4];
                    int read = await pipe.ReadAsync(data, stopping.Token);
                    if (read == 4 && Encoding.ASCII.GetString(data) == "show") show();
                }
                catch (OperationCanceledException) { break; }
                catch (IOException) { }
            }
        });
    }

    private static void SignalExisting()
    {
        try
        {
            using NamedPipeClientStream pipe = new(".", PipeName, PipeDirection.Out);
            pipe.Connect(750);
            pipe.Write(Encoding.ASCII.GetBytes("show"));
        }
        catch (IOException) { }
        catch (TimeoutException) { }
    }

    public void Dispose()
    {
        stopping.Cancel();
        try { listener?.Wait(TimeSpan.FromSeconds(1)); } catch (AggregateException) { }
        stopping.Dispose();
        mutex.ReleaseMutex();
        mutex.Dispose();
    }
}

internal sealed class GameLaunchGate
{
    private int started;
    internal bool TryBegin() => Interlocked.CompareExchange(ref started, 1, 0) == 0;
    internal bool Started => Volatile.Read(ref started) != 0;
}
