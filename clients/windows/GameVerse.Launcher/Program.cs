using System.Diagnostics;
using System.IO.Compression;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Text.RegularExpressions;
using Microsoft.Win32;

return await Launcher.RunAsync(args);

internal sealed record LauncherConfig(
    string GameDirectory,
    string UiPath,
    string BridgePath,
    string UiPipe,
    string ServerAddress,
    string CertificatePath,
    string? LogDirectory);

internal sealed record Check(string Name, bool Passed, string Detail);

internal static class Launcher
{
    private const string ConfigName = "launcher.json";

    internal static async Task<int> RunAsync(string[] args)
    {
        string command = args.FirstOrDefault()?.ToLowerInvariant() ?? "verify";
        if (command == "init") return WriteExample();
        if (command == "__ready-child")
        {
            Console.WriteLine(JsonSerializer.Serialize(new { @event = args.ElementAtOrDefault(1) ?? "self_test_ready" }));
            Console.Out.Flush();
            await Task.Delay(500);
            return 0;
        }
        if (command == "self-test") return await SelfTestAsync();
        LauncherConfig config;
        try { config = Load(); }
        catch (Exception error)
        {
            Console.Error.WriteLine(JsonSerializer.Serialize(new { status = "configuration_error", error = error.Message }));
            return 2;
        }
        try
        {
            return command switch
            {
                "verify" => Verify(config),
                "start" => await StartAsync(config),
                "logs" => OpenLogs(config),
                "diagnostics" => Diagnostics(config, args.ElementAtOrDefault(1)),
                _ => Usage()
            };
        }
        catch (Exception error)
        {
            Console.Error.WriteLine(JsonSerializer.Serialize(new { status = "failed", error = error.Message }));
            return 4;
        }
    }

    private static LauncherConfig Load()
    {
        string path = Path.Combine(AppContext.BaseDirectory, ConfigName);
        if (!File.Exists(path)) throw new FileNotFoundException($"Create {ConfigName} with 'init' first", path);
        LauncherConfig config = JsonSerializer.Deserialize<LauncherConfig>(File.ReadAllText(path), new JsonSerializerOptions
        {
            PropertyNameCaseInsensitive = true
        }) ?? throw new InvalidDataException("Launcher configuration is empty");
        if (new[] { config.GameDirectory, config.UiPath, config.BridgePath, config.UiPipe, config.ServerAddress, config.CertificatePath }
            .Any(string.IsNullOrWhiteSpace))
            throw new InvalidDataException("Launcher configuration contains an empty required value");
        if (!config.UiPipe.StartsWith(@"\\.\pipe\", StringComparison.OrdinalIgnoreCase))
            throw new InvalidDataException("UiPipe must be a local Windows named-pipe path");
        return config;
    }

    private static int WriteExample()
    {
        string path = Path.Combine(AppContext.BaseDirectory, ConfigName);
        if (File.Exists(path)) throw new IOException($"Refusing to replace {path}");
        var example = new LauncherConfig(
            @"C:\Games\Grand Theft Auto V Enhanced",
            @"C:\GameVerse\GameVerse.UI.exe",
            @"C:\GameVerse\gameverse-gta-bridge-m2.exe",
            @"\\.\pipe\gameverse-ui-v1",
            "127.0.0.1:30122",
            @"C:\GameVerse\server-cert.der",
            @"C:\GameVerse\logs");
        File.WriteAllText(path, JsonSerializer.Serialize(example, new JsonSerializerOptions { WriteIndented = true }));
        Console.WriteLine(JsonSerializer.Serialize(new { status = "created", path }));
        return 0;
    }

    private static List<Check> Checks(LauncherConfig config)
    {
        string game = Path.GetFullPath(Environment.ExpandEnvironmentVariables(config.GameDirectory));
        string ui = Path.GetFullPath(Environment.ExpandEnvironmentVariables(config.UiPath));
        string bridge = Path.GetFullPath(Environment.ExpandEnvironmentVariables(config.BridgePath));
        string cert = Path.GetFullPath(Environment.ExpandEnvironmentVariables(config.CertificatePath));
        string enhanced = Path.Combine(game, "GTA5_Enhanced.exe");
        string play = Path.Combine(game, "PlayGTAV.exe");
        MemoryStatus memory = new();
        GlobalMemoryStatusEx(memory);
        ulong freeGiB = memory.AvailablePhysical / 1024 / 1024 / 1024;
        return new List<Check>
        {
            new("windows", OperatingSystem.IsWindowsVersionAtLeast(10), Environment.OSVersion.VersionString),
            new("game_directory", Directory.Exists(game), game),
            new("gta_enhanced", File.Exists(enhanced), enhanced),
            new("play_gtav", File.Exists(play), play),
            new("gameverse_ui", File.Exists(ui), ui),
            new("webview2_runtime", WebView2Version() is not null, WebView2Version() ?? "Install Microsoft Edge WebView2 Evergreen Runtime"),
            new("bridge", File.Exists(bridge), bridge),
            new("server_certificate", File.Exists(cert), cert),
            new("free_memory", freeGiB >= 4, $"{freeGiB} GiB available"),
            new("adapter", File.Exists(Path.Combine(game, "scripts", "GameVerse.GtaAdapter.dll")), Path.Combine(game, "scripts", "GameVerse.GtaAdapter.dll")),
            new("scripthook", File.Exists(Path.Combine(game, "ScriptHookV.dll")), Path.Combine(game, "ScriptHookV.dll")),
            new("scripthookdotnet", File.Exists(Path.Combine(game, "ScriptHookVDotNet.asi")), Path.Combine(game, "ScriptHookVDotNet.asi"))
        };
    }

    private static int Verify(LauncherConfig config)
    {
        List<Check> checks = Checks(config);
        Console.WriteLine(JsonSerializer.Serialize(new { status = checks.All(value => value.Passed) ? "ready" : "failed", checks }));
        return checks.All(value => value.Passed) ? 0 : 3;
    }

    private static async Task<int> StartAsync(LauncherConfig config)
    {
        if (Verify(config) != 0) return 3;
        string game = Path.GetFullPath(Environment.ExpandEnvironmentVariables(config.GameDirectory));
        string ui = Path.GetFullPath(Environment.ExpandEnvironmentVariables(config.UiPath));
        string bridge = Path.GetFullPath(Environment.ExpandEnvironmentVariables(config.BridgePath));
        string cert = Path.GetFullPath(Environment.ExpandEnvironmentVariables(config.CertificatePath));
        ProcessStartInfo uiInfo = new()
        {
            FileName = ui,
            UseShellExecute = false,
            CreateNoWindow = false,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            WorkingDirectory = Path.GetDirectoryName(ui)!
        };
        uiInfo.ArgumentList.Add("--pipe");
        uiInfo.ArgumentList.Add(config.UiPipe);
        Process uiProcess = Process.Start(uiInfo) ?? throw new InvalidOperationException("GameVerse UI did not start");
        try { await WaitForReadyEventAsync(uiProcess, "ui_ready", TimeSpan.FromSeconds(20), "GameVerse UI"); }
        catch { uiProcess.Dispose(); throw; }
        ProcessStartInfo bridgeInfo = new()
        {
            FileName = bridge,
            UseShellExecute = false,
            CreateNoWindow = true,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            WorkingDirectory = Path.GetDirectoryName(bridge)!
        };
        bridgeInfo.ArgumentList.Add("--server");
        bridgeInfo.ArgumentList.Add(config.ServerAddress);
        bridgeInfo.ArgumentList.Add("--cert");
        bridgeInfo.ArgumentList.Add(cert);
        bridgeInfo.ArgumentList.Add("--ui-pipe");
        bridgeInfo.ArgumentList.Add(config.UiPipe);
        using Process bridgeProcess = Process.Start(bridgeInfo) ?? throw new InvalidOperationException("Bridge did not start");
        try { await WaitForReadyEventAsync(bridgeProcess, "m2_pipe_ready", TimeSpan.FromSeconds(15), "Bridge"); }
        catch
        {
            if (!uiProcess.HasExited) uiProcess.Kill(entireProcessTree: true);
            uiProcess.Dispose();
            throw;
        }
        Process.Start(new ProcessStartInfo
        {
            FileName = Path.Combine(game, "PlayGTAV.exe"),
            WorkingDirectory = game,
            UseShellExecute = true
        });
        Console.WriteLine(JsonSerializer.Serialize(new { status = "started", ui_pid = uiProcess.Id, bridge_pid = bridgeProcess.Id, stage = "waiting_for_adapter" }));
        try
        {
            using Process gameProcess = await WaitForGameAsync(TimeSpan.FromMinutes(2));
            Console.WriteLine(JsonSerializer.Serialize(new { status = "active", game_pid = gameProcess.Id }));
            await gameProcess.WaitForExitAsync();
            return 0;
        }
        finally
        {
            if (!bridgeProcess.HasExited) bridgeProcess.Kill(entireProcessTree: true);
            if (!uiProcess.HasExited) uiProcess.Kill(entireProcessTree: true);
            uiProcess.Dispose();
        }
    }

    private static async Task<Process> WaitForGameAsync(TimeSpan timeout)
    {
        using CancellationTokenSource deadline = new(timeout);
        try
        {
            while (true)
            {
                Process? game = Process.GetProcessesByName("GTA5_Enhanced").FirstOrDefault();
                if (game is not null) return game;
                await Task.Delay(500, deadline.Token);
            }
        }
        catch (OperationCanceledException) when (deadline.IsCancellationRequested)
        {
            throw new TimeoutException("GTA V Enhanced did not start within two minutes");
        }
    }

    private static async Task WaitForReadyEventAsync(Process process, string expectedEvent, TimeSpan timeout, string component)
    {
        using CancellationTokenSource deadline = new(timeout);
        try
        {
            while (true)
            {
                string? line = await process.StandardOutput.ReadLineAsync(deadline.Token);
                if (line is null)
                {
                    string error = await process.StandardError.ReadToEndAsync(deadline.Token);
                    throw new InvalidOperationException($"Bridge stopped before readiness: {error.Trim()}");
                }
                using JsonDocument message = JsonDocument.Parse(line);
                if (message.RootElement.TryGetProperty("event", out JsonElement value)
                    && value.GetString() == expectedEvent) return;
            }
        }
        catch (OperationCanceledException) when (deadline.IsCancellationRequested)
        {
            if (!process.HasExited) process.Kill(entireProcessTree: true);
            throw new TimeoutException($"{component} did not report readiness within {timeout.TotalSeconds:0} seconds");
        }
    }

    private static async Task<int> SelfTestAsync()
    {
        string executable = Environment.ProcessPath ?? throw new InvalidOperationException("Launcher executable path is unavailable");
        ProcessStartInfo info = new()
        {
            FileName = executable,
            UseShellExecute = false,
            CreateNoWindow = true,
            RedirectStandardOutput = true,
            RedirectStandardError = true
        };
        if (Path.GetFileNameWithoutExtension(executable).Equals("dotnet", StringComparison.OrdinalIgnoreCase))
            info.ArgumentList.Add(System.Reflection.Assembly.GetExecutingAssembly().Location);
        info.ArgumentList.Add("__ready-child");
        info.ArgumentList.Add("self_test_ready");
        using Process child = Process.Start(info) ?? throw new InvalidOperationException("Self-test child did not start");
        await WaitForReadyEventAsync(child, "self_test_ready", TimeSpan.FromSeconds(3), "Self-test child");
        await child.WaitForExitAsync();
        bool passed = child.ExitCode == 0;
        Console.WriteLine(JsonSerializer.Serialize(new { status = passed ? "passed" : "failed", readiness_event = passed, child_cleaned_up = child.HasExited }));
        return passed ? 0 : 1;
    }

    private static int OpenLogs(LauncherConfig config)
    {
        string logs = LogDirectory(config);
        Directory.CreateDirectory(logs);
        Process.Start(new ProcessStartInfo { FileName = logs, UseShellExecute = true });
        return 0;
    }

    private static int Diagnostics(LauncherConfig config, string? output)
    {
        string target = Path.GetFullPath(output ?? Path.Combine(Environment.CurrentDirectory, $"gameverse-diagnostics-{DateTime.UtcNow:yyyyMMdd-HHmmss}.zip"));
        string staging = Path.Combine(Path.GetTempPath(), "gameverse-diagnostics-" + Guid.NewGuid().ToString("N"));
        Directory.CreateDirectory(staging);
        try
        {
            File.WriteAllText(Path.Combine(staging, "checks.json"), JsonSerializer.Serialize(new
            {
                generated_at = DateTimeOffset.UtcNow,
                checks = Checks(config),
                processes = Process.GetProcesses().Where(process => new[] { "PlayGTAV", "GTA5_Enhanced", "gameverse-gta-bridge-m2" }.Contains(process.ProcessName, StringComparer.OrdinalIgnoreCase)).Select(process => new { process.ProcessName, process.Id })
            }));
            string logs = LogDirectory(config);
            if (Directory.Exists(logs))
                foreach (string file in Directory.EnumerateFiles(logs, "*.log").Take(20))
                {
                    string content = File.ReadAllText(file);
                    content = Regex.Replace(content, "(?i)(password|token|authorization|cookie)(\\s*[:=]\\s*)[^\\s,;]+", "$1$2[REDACTED]");
                    File.WriteAllText(Path.Combine(staging, Path.GetFileName(file)), content);
                }
            if (File.Exists(target)) File.Delete(target);
            ZipFile.CreateFromDirectory(staging, target);
            Console.WriteLine(JsonSerializer.Serialize(new { status = "created", path = target }));
            return 0;
        }
        finally { Directory.Delete(staging, true); }
    }

    private static string LogDirectory(LauncherConfig config) => Path.GetFullPath(Environment.ExpandEnvironmentVariables(config.LogDirectory ?? Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "GameVerse", "logs")));
    private static string? WebView2Version()
    {
        const string client = @"SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}";
        foreach ((RegistryHive hive, RegistryView view) in new[]
        {
            (RegistryHive.CurrentUser, RegistryView.Default),
            (RegistryHive.LocalMachine, RegistryView.Registry32),
            (RegistryHive.LocalMachine, RegistryView.Registry64)
        })
        {
            using RegistryKey root = RegistryKey.OpenBaseKey(hive, view);
            using RegistryKey? key = root.OpenSubKey(client);
            if (key?.GetValue("pv") is string version && !string.IsNullOrWhiteSpace(version)) return version;
        }
        return null;
    }
    private static int Usage()
    {
        Console.Error.WriteLine("Usage: GameVerse.Launcher init|verify|start|logs|diagnostics [output.zip]|self-test");
        return 1;
    }

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern bool GlobalMemoryStatusEx([In, Out] MemoryStatus buffer);

    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Auto)]
    private sealed class MemoryStatus
    {
        public uint Length = (uint)Marshal.SizeOf<MemoryStatus>();
        public uint MemoryLoad;
        public ulong TotalPhysical;
        public ulong AvailablePhysical;
        public ulong TotalPageFile;
        public ulong AvailablePageFile;
        public ulong TotalVirtual;
        public ulong AvailableVirtual;
        public ulong AvailableExtendedVirtual;
    }
}
