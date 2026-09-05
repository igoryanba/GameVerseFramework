using System.Diagnostics;
using System.IO.Compression;
using System.Runtime.InteropServices;
using System.Security.Cryptography;
using System.Text.Json;
using System.Text.RegularExpressions;
internal static class Program
{
    [STAThread]
    private static int Main(string[] args)
    {
        string command = args.FirstOrDefault()?.ToLowerInvariant() ?? "start";
        if (command == "start")
        {
            ApplicationConfiguration.Initialize();
            return LauncherWindow.Run(args);
        }
        return Launcher.RunAsync(args).GetAwaiter().GetResult();
    }
}

internal sealed record LauncherConfig(
    string GameDirectory,
    string BridgePath,
    string UiPipe,
    string AdapterPipe,
    string BootstrapPipe,
    string ServerAddress,
    string? DirectoryUrl,
    string CertificatePath,
    string CertificateSha256,
    string UpdateChannel,
    string LogLevel,
    bool RequireInstallManifest,
    bool DeveloperManualStory,
    bool DeveloperTelemetryStory,
    string? LogDirectory,
    string? UpdateManifestUrl,
    string? UpdateSignatureUrl,
    string? UpdatePublicKeyPath);

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
        if (command == "__render-ui") return TerminalLauncherForm.RenderTest(args.ElementAtOrDefault(1));
        if (command == "__generate-update-test-key") return GenerateUpdateTestKey(args);
        if (command == "__apply-update") return await ApplyUpdateAsync(args);
        if (command == "verify-update") return VerifyUpdate(args);
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
                "start" => throw new InvalidOperationException("Interactive startup must be hosted by the launcher window"),
                "logs" => OpenLogs(config),
                "diagnostics" => Diagnostics(config, args.ElementAtOrDefault(1)),
                "update" => await UpdateAsync(config),
                _ => Usage()
            };
        }
        catch (Exception error)
        {
            Console.Error.WriteLine(JsonSerializer.Serialize(new { status = "failed", error = error.Message }));
            return 4;
        }
    }

    internal static LauncherConfig Load()
    {
        string path = Path.Combine(AppContext.BaseDirectory, ConfigName);
        if (!File.Exists(path)) throw new FileNotFoundException($"Create {ConfigName} with 'init' first", path);
        LauncherConfig config = JsonSerializer.Deserialize<LauncherConfig>(File.ReadAllText(path), new JsonSerializerOptions
        {
            PropertyNameCaseInsensitive = true
        }) ?? throw new InvalidDataException("Launcher configuration is empty");
        if (new[] { config.GameDirectory, config.BridgePath, config.UiPipe, config.AdapterPipe, config.BootstrapPipe, config.ServerAddress, config.CertificatePath, config.CertificateSha256, config.UpdateChannel, config.LogLevel }
            .Any(string.IsNullOrWhiteSpace))
            throw new InvalidDataException("Launcher configuration contains an empty required value");
        if (!config.UiPipe.StartsWith(@"\\.\pipe\", StringComparison.OrdinalIgnoreCase))
            throw new InvalidDataException("UiPipe must be a local Windows named-pipe path");
        if (!config.AdapterPipe.StartsWith(@"\\.\pipe\", StringComparison.OrdinalIgnoreCase) || config.AdapterPipe == config.UiPipe)
            throw new InvalidDataException("AdapterPipe must be a distinct local Windows named-pipe path");
        if (!config.BootstrapPipe.StartsWith(@"\\.\pipe\", StringComparison.OrdinalIgnoreCase)
            || config.BootstrapPipe == config.UiPipe || config.BootstrapPipe == config.AdapterPipe)
            throw new InvalidDataException("BootstrapPipe must be a distinct local Windows named-pipe path");
        if (!Regex.IsMatch(config.CertificateSha256, "^[A-Fa-f0-9]{64}$"))
            throw new InvalidDataException("CertificateSha256 must contain 64 hexadecimal characters");
        if (config.DeveloperManualStory && config.DeveloperTelemetryStory)
            throw new InvalidDataException("DeveloperManualStory and DeveloperTelemetryStory are mutually exclusive");
        return config;
    }

    private static int WriteExample()
    {
        string path = Path.Combine(AppContext.BaseDirectory, ConfigName);
        if (File.Exists(path)) throw new IOException($"Refusing to replace {path}");
        var example = new LauncherConfig(
            @"C:\Games\Grand Theft Auto V Enhanced",
            @"bridge\gameverse-gta-bridge-m2.exe",
            @"\\.\pipe\gameverse-ui-v1",
            @"\\.\pipe\gameverse-gta-v1",
            @"\\.\pipe\gameverse-bootstrap-v1",
            "127.0.0.1:30122",
            "http://127.0.0.1:30123/v1/servers",
            @"server-cert.der",
            new string('0', 64),
            "alpha",
            "info",
            false,
            false,
            false,
            @"%LOCALAPPDATA%\GameVerse\logs",
            null,
            null,
            @"update-public-key.pem");
        File.WriteAllText(path, JsonSerializer.Serialize(example, new JsonSerializerOptions { WriteIndented = true }));
        Console.WriteLine(JsonSerializer.Serialize(new { status = "created", path }));
        return 0;
    }

    internal static List<Check> Checks(LauncherConfig config, bool allowLowMemory = false)
    {
        string game = ResolvePath(config.GameDirectory);
        string bridge = ResolvePath(config.BridgePath);
        string cert = ResolvePath(config.CertificatePath);
        string enhanced = Path.Combine(game, "GTA5_Enhanced.exe");
        string play = Path.Combine(game, "PlayGTAV.exe");
        string compatibility = Path.Combine(game, "enhanced-1.0.1158.13.json");
        MemoryStatus memory = new();
        GlobalMemoryStatusEx(memory);
        ulong freeGiB = memory.AvailablePhysical / 1024 / 1024 / 1024;
        Check installManifest = CheckInstallManifest(config.RequireInstallManifest);
        return new List<Check>
        {
            installManifest,
            new("windows", OperatingSystem.IsWindowsVersionAtLeast(10), Environment.OSVersion.VersionString),
            new("game_directory", Directory.Exists(game), game),
            new("gta_enhanced", File.Exists(enhanced), enhanced),
            new("play_gtav", File.Exists(play), play),
            new("bridge", File.Exists(bridge), bridge),
            new("server_certificate", File.Exists(cert), cert),
            new("server_certificate_fingerprint", File.Exists(cert) && CertificateHash(cert).Equals(config.CertificateSha256, StringComparison.OrdinalIgnoreCase), File.Exists(cert) ? CertificateHash(cert) : "certificate unavailable"),
            new("free_memory", freeGiB >= 4 || allowLowMemory,
                freeGiB >= 4
                    ? $"{freeGiB} GiB available"
                    : allowLowMemory
                        ? $"{freeGiB} GiB available; explicit low-memory override enabled"
                        : $"{freeGiB} GiB available; 4 GiB recommended (start with --allow-low-memory to continue at your own risk)"),
            new("adapter", File.Exists(Path.Combine(game, "scripts", "GameVerse.GtaAdapter.dll")), Path.Combine(game, "scripts", "GameVerse.GtaAdapter.dll")),
            new("native_bootstrap", config.DeveloperManualStory || File.Exists(Path.Combine(game, "GameVerse.NativeBootstrap.asi")), config.DeveloperManualStory ? "developer manual Story diagnostic" : Path.Combine(game, "GameVerse.NativeBootstrap.asi")),
            CheckBootstrapCompatibility(config, compatibility, Path.Combine(game, "enhanced-1.0.1158.13.sig")),
            new("scripthook", File.Exists(Path.Combine(game, "ScriptHookV.dll")), Path.Combine(game, "ScriptHookV.dll")),
            new("scripthookdotnet", File.Exists(Path.Combine(game, "ScriptHookVDotNet.asi")), Path.Combine(game, "ScriptHookVDotNet.asi"))
        };
    }

    private static Check CheckBootstrapCompatibility(LauncherConfig config, string manifestPath, string signaturePath)
    {
        if (config.DeveloperManualStory)
            return new("bootstrap_compatibility", true, "developer manual Story diagnostic");
        if (!File.Exists(manifestPath) || !File.Exists(signaturePath))
            return new("bootstrap_compatibility", false, "compatibility manifest or signature is missing");
        try
        {
            using JsonDocument document = JsonDocument.Parse(File.ReadAllBytes(manifestPath));
            string? mode = document.RootElement.TryGetProperty("mode", out JsonElement value) ? value.GetString() : null;
            bool telemetryAllowed = config.DeveloperTelemetryStory && mode == "telemetry_only";
            bool productionReady = !config.DeveloperTelemetryStory && mode == "world_loader"
                && document.RootElement.TryGetProperty("signatures", out JsonElement signatures)
                && signatures.ValueKind == JsonValueKind.Array && signatures.GetArrayLength() > 0;
            return new(
                "bootstrap_compatibility",
                telemetryAllowed || productionReady,
                productionReady ? "verified automatic world loader"
                    : telemetryAllowed ? "developer telemetry Story diagnostic"
                    : "automatic world loader is not verified for this GTA build");
        }
        catch (JsonException error)
        {
            return new("bootstrap_compatibility", false, $"invalid compatibility manifest: {error.Message}");
        }
    }

    private static int Verify(LauncherConfig config)
    {
        List<Check> checks = Checks(config);
        Console.WriteLine(JsonSerializer.Serialize(new { status = checks.All(value => value.Passed) ? "ready" : "failed", checks }));
        return checks.All(value => value.Passed) ? 0 : 3;
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
        {
            string managedEntry = Environment.GetCommandLineArgs()[0];
            if (!Path.GetExtension(managedEntry).Equals(".dll", StringComparison.OrdinalIgnoreCase))
                throw new InvalidOperationException("Managed launcher entry path is unavailable");
            info.ArgumentList.Add(Path.GetFullPath(managedEntry));
        }
        info.ArgumentList.Add("__ready-child");
        info.ArgumentList.Add("self_test_ready");
        using Process child = Process.Start(info) ?? throw new InvalidOperationException("Self-test child did not start");
        await WaitForReadyEventAsync(child, "self_test_ready", TimeSpan.FromSeconds(3), "Self-test child");
        await child.WaitForExitAsync();
        bool passed = child.ExitCode == 0;
        bool updateSecurity = UpdateSecuritySelfTest();
        bool atomicUpdate = AtomicUpdateSelfTest();
        bool terminalUi = TerminalLauncherForm.SelfTest();
        bool bootstrapGate = BootstrapCompatibilitySelfTest();
        passed = passed && updateSecurity && atomicUpdate && terminalUi && bootstrapGate;
        Console.WriteLine(JsonSerializer.Serialize(new { status = passed ? "passed" : "failed", readiness_event = child.ExitCode == 0, child_cleaned_up = child.HasExited, update_signature = updateSecurity, atomic_update_rollback = atomicUpdate, terminal_ui = terminalUi, bootstrap_gate = bootstrapGate }));
        return passed ? 0 : 1;
    }

    private static bool BootstrapCompatibilitySelfTest()
    {
        string directory = Path.Combine(Path.GetTempPath(), "gameverse-bootstrap-gate-" + Guid.NewGuid().ToString("N"));
        Directory.CreateDirectory(directory);
        string manifest = Path.Combine(directory, "compatibility.json");
        string signature = Path.Combine(directory, "compatibility.sig");
        File.WriteAllBytes(signature, new byte[] { 1 });
        try
        {
            LauncherConfig production = new("game", "bridge", @"\\.\pipe\ui", @"\\.\pipe\adapter", @"\\.\pipe\bootstrap",
                "127.0.0.1:30122", null, "cert", new string('0', 64), "alpha", "info", false, false, false, null, null, null, null);
            File.WriteAllText(manifest, "{\"mode\":\"telemetry_only\",\"signatures\":[]}");
            if (CheckBootstrapCompatibility(production, manifest, signature).Passed) return false;
            LauncherConfig telemetry = production with { DeveloperTelemetryStory = true };
            if (!CheckBootstrapCompatibility(telemetry, manifest, signature).Passed) return false;
            File.WriteAllText(manifest, "{\"mode\":\"world_loader\",\"signatures\":[{\"name\":\"world_request\"}]}");
            return CheckBootstrapCompatibility(production, manifest, signature).Passed;
        }
        finally { Directory.Delete(directory, true); }
    }

    private static int VerifyUpdate(string[] args)
    {
        if (args.Length != 4) throw new ArgumentException("verify-update requires manifest, signature, and public key paths");
        VerifiedUpdate update = UpdateSecurity.Verify(File.ReadAllBytes(args[1]), File.ReadAllBytes(args[2]), File.ReadAllText(args[3]));
        Console.WriteLine(JsonSerializer.Serialize(new { status = "verified", update.Version, update.Channel, files = update.Files.Count }));
        return 0;
    }

    private static int GenerateUpdateTestKey(string[] args)
    {
        if (args.Length != 3) throw new ArgumentException("test key generation requires private and public output paths");
        using ECDsa key = ECDsa.Create(ECCurve.NamedCurves.nistP256);
        File.WriteAllText(args[1], key.ExportECPrivateKeyPem());
        File.WriteAllText(args[2], key.ExportSubjectPublicKeyInfoPem());
        return 0;
    }

    private static bool UpdateSecuritySelfTest()
    {
        using ECDsa key = ECDsa.Create(ECCurve.NamedCurves.nistP256);
        string pem = key.ExportSubjectPublicKeyInfoPem();
        byte[] manifest = JsonSerializer.SerializeToUtf8Bytes(new
        {
            schema_version = 1,
            version = "0.1.0",
            channel = "alpha",
            minimum_launcher_version = "0.1.0",
            signature = new { algorithm = "ECDSA_P256_SHA256", key_id = "self-test" },
            files = new[] { new { path = "bridge/gameverse-gta-bridge-m2.exe", size = 1, sha256 = new string('a', 64), url = "https://updates.gameverse.invalid/bridge.exe" } }
        });
        byte[] signature = key.SignData(manifest, HashAlgorithmName.SHA256);
        VerifiedUpdate verified = UpdateSecurity.Verify(manifest, signature, pem);
        manifest[^1] ^= 1;
        try
        {
            UpdateSecurity.Verify(manifest, signature, pem);
            return false;
        }
        catch (CryptographicException)
        {
            return verified.Files.Count == 1;
        }
    }

    private static bool AtomicUpdateSelfTest()
    {
        string parent = Path.Combine(Path.GetTempPath(), "gameverse-update-test-" + Guid.NewGuid().ToString("N"));
        string install = Path.Combine(parent, "GameVerse");
        string backup = install + ".previous";
        string stage = Path.Combine(parent, ".gameverse-update-" + Guid.NewGuid().ToString("N"));
        try
        {
            Directory.CreateDirectory(install);
            File.WriteAllText(Path.Combine(install, "version.txt"), "old");
            WriteTestInstall(stage, "new");
            if (!AtomicUpdate.Apply(install, stage, backup, out _)) return false;
            if (File.ReadAllText(Path.Combine(install, "version.txt")) != "new" || File.ReadAllText(Path.Combine(backup, "version.txt")) != "old") return false;
            if (!AtomicUpdate.Rollback(install, backup, out _)) return false;
            if (File.ReadAllText(Path.Combine(install, "version.txt")) != "old") return false;

            string broken = Path.Combine(parent, ".gameverse-update-" + Guid.NewGuid().ToString("N"));
            WriteTestInstall(broken, "broken");
            File.WriteAllText(Path.Combine(broken, "version.txt"), "tampered");
            bool applied = AtomicUpdate.Apply(install, broken, backup, out _);
            return !applied && File.ReadAllText(Path.Combine(install, "version.txt")) == "old";
        }
        finally
        {
            if (Directory.Exists(parent)) Directory.Delete(parent, true);
        }
    }

    private static void WriteTestInstall(string directory, string version)
    {
        Directory.CreateDirectory(directory);
        string file = Path.Combine(directory, "version.txt");
        File.WriteAllText(file, version);
        File.WriteAllText(Path.Combine(directory, "install-manifest.json"), JsonSerializer.Serialize(new
        {
            schema_version = 1,
            files = new[] { new { path = "version.txt", size = new FileInfo(file).Length, sha256 = Convert.ToHexString(SHA256.HashData(File.ReadAllBytes(file))) } }
        }));
    }

    private static async Task<int> UpdateAsync(LauncherConfig config)
    {
        if (!Uri.TryCreate(config.UpdateManifestUrl, UriKind.Absolute, out Uri? manifestUrl) || manifestUrl.Scheme != Uri.UriSchemeHttps
            || !Uri.TryCreate(config.UpdateSignatureUrl, UriKind.Absolute, out Uri? signatureUrl) || signatureUrl.Scheme != Uri.UriSchemeHttps
            || string.IsNullOrWhiteSpace(config.UpdatePublicKeyPath))
            throw new InvalidDataException("UpdateManifestUrl, UpdateSignatureUrl and UpdatePublicKeyPath must configure signed HTTPS updates");
        using HttpClient client = new() { Timeout = TimeSpan.FromMinutes(10) };
        byte[] manifest = await AtomicUpdate.DownloadSmallAsync(client, manifestUrl, 1024 * 1024, CancellationToken.None);
        byte[] signature = await AtomicUpdate.DownloadSmallAsync(client, signatureUrl, 1024, CancellationToken.None);
        VerifiedUpdate update = UpdateSecurity.Verify(manifest, signature, File.ReadAllText(ResolvePath(config.UpdatePublicKeyPath)));
        if (!update.Channel.Equals(config.UpdateChannel, StringComparison.OrdinalIgnoreCase)) throw new InvalidDataException("update channel does not match launcher configuration");
        string launcherVersion = typeof(Launcher).Assembly.GetName().Version?.ToString(3) ?? "0.0.0";
        if (!VersionAtLeast(launcherVersion, update.MinimumLauncherVersion)) throw new InvalidDataException($"update requires launcher {update.MinimumLauncherVersion} or newer");

        string install = Path.GetFullPath(AppContext.BaseDirectory).TrimEnd(Path.DirectorySeparatorChar);
        string staging = await AtomicUpdate.StageAsync(update, install, client, CancellationToken.None);
        string? relativeCertificate = Path.IsPathRooted(config.CertificatePath) ? null : config.CertificatePath;
        AtomicUpdate.PreserveLocalFiles(install, staging, ConfigName, relativeCertificate);

        string helperDirectory = Path.Combine(Path.GetTempPath(), "GameVerse-Updater-" + Guid.NewGuid().ToString("N"));
        Directory.CreateDirectory(helperDirectory);
        string executable = Environment.ProcessPath ?? throw new InvalidOperationException("launcher executable path is unavailable");
        if (Path.GetFileNameWithoutExtension(executable).Equals("dotnet", StringComparison.OrdinalIgnoreCase))
            throw new InvalidOperationException("atomic update requires the packaged launcher executable");
        string helper = Path.Combine(helperDirectory, Path.GetFileName(executable));
        File.Copy(executable, helper);
        ProcessStartInfo info = new() { FileName = helper, UseShellExecute = false, CreateNoWindow = true };
        info.ArgumentList.Add("__apply-update");
        info.ArgumentList.Add(install);
        info.ArgumentList.Add(staging);
        info.ArgumentList.Add(install + ".previous");
        info.ArgumentList.Add(Environment.ProcessId.ToString());
        Process.Start(info)?.Dispose();
        Console.WriteLine(JsonSerializer.Serialize(new { status = "staged", update.Version, action = "launcher_will_restart_after_atomic_install" }));
        return 0;
    }

    private static async Task<int> ApplyUpdateAsync(string[] args)
    {
        if (args.Length != 5 || !int.TryParse(args[4], out int parentPid)) throw new ArgumentException("invalid update transaction arguments");
        await AtomicUpdate.WaitForExitAsync(parentPid, TimeSpan.FromMinutes(2));
        bool applied = AtomicUpdate.Apply(args[1], args[2], args[3], out string detail);
        if (applied)
        {
            string launcher = Path.Combine(args[1], "GameVerse.Launcher.exe");
            applied = await UpdatedLauncherSelfTestAsync(launcher);
            if (!applied) AtomicUpdate.Rollback(args[1], args[3], out detail);
            else Process.Start(new ProcessStartInfo { FileName = launcher, UseShellExecute = true, WorkingDirectory = args[1] })?.Dispose();
        }
        return applied ? 0 : 1;
    }

    private static async Task<bool> UpdatedLauncherSelfTestAsync(string launcher)
    {
        if (!File.Exists(launcher)) return false;
        using Process? process = Process.Start(new ProcessStartInfo
        {
            FileName = launcher,
            Arguments = "self-test",
            WorkingDirectory = Path.GetDirectoryName(launcher)!,
            UseShellExecute = false,
            CreateNoWindow = true,
            RedirectStandardOutput = true,
            RedirectStandardError = true
        });
        if (process is null) return false;
        using CancellationTokenSource deadline = new(TimeSpan.FromSeconds(30));
        try { await process.WaitForExitAsync(deadline.Token); }
        catch (OperationCanceledException) { try { process.Kill(entireProcessTree: true); } catch (InvalidOperationException) { } return false; }
        return process.ExitCode == 0;
    }

    private static bool VersionAtLeast(string actual, string required)
    {
        static int[] Parts(string value)
        {
            string core = value.Split('-', 2)[0];
            string[] pieces = core.Split('.');
            if (pieces.Length is < 1 or > 4 || pieces.Any(piece => !int.TryParse(piece, out _))) throw new InvalidDataException($"invalid version: {value}");
            return pieces.Select(int.Parse).Concat(Enumerable.Repeat(0, 4)).Take(4).ToArray();
        }
        int[] left = Parts(actual);
        int[] right = Parts(required);
        for (int index = 0; index < 4; index++)
        {
            if (left[index] != right[index]) return left[index] > right[index];
        }
        return true;
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

    internal static string ResolvePath(string value)
    {
        string expanded = Environment.ExpandEnvironmentVariables(value);
        return Path.GetFullPath(expanded, AppContext.BaseDirectory);
    }
    private static string CertificateHash(string path) => Convert.ToHexString(SHA256.HashData(File.ReadAllBytes(path)));
    private static Check CheckInstallManifest(bool required)
    {
        string manifestPath = Path.Combine(AppContext.BaseDirectory, "install-manifest.json");
        if (!File.Exists(manifestPath))
            return new Check("install_manifest", !required, required ? "install-manifest.json is missing" : "development build");
        try
        {
            using JsonDocument document = JsonDocument.Parse(File.ReadAllText(manifestPath));
            if (document.RootElement.GetProperty("schema_version").GetInt32() != 1)
                throw new InvalidDataException("unsupported schema");
            string root = Path.GetFullPath(AppContext.BaseDirectory);
            string rootPrefix = root.TrimEnd(Path.DirectorySeparatorChar) + Path.DirectorySeparatorChar;
            int count = 0;
            foreach (JsonElement file in document.RootElement.GetProperty("files").EnumerateArray())
            {
                string relative = file.GetProperty("path").GetString() ?? throw new InvalidDataException("missing path");
                string full = Path.GetFullPath(relative.Replace('/', Path.DirectorySeparatorChar), root);
                if (!full.StartsWith(rootPrefix, StringComparison.OrdinalIgnoreCase) || !File.Exists(full))
                    throw new InvalidDataException($"invalid packaged path: {relative}");
                long size = file.GetProperty("size").GetInt64();
                string hash = file.GetProperty("sha256").GetString() ?? "";
                if (new FileInfo(full).Length != size || !CertificateHash(full).Equals(hash, StringComparison.OrdinalIgnoreCase))
                    throw new InvalidDataException($"package integrity check failed: {relative}");
                count++;
            }
            return new Check("install_manifest", count > 0, $"{count} packaged files verified");
        }
        catch (Exception error) when (error is IOException or JsonException or InvalidDataException or KeyNotFoundException)
        {
            return new Check("install_manifest", false, error.Message);
        }
    }
    private static string LogDirectory(LauncherConfig config) => ResolvePath(config.LogDirectory ?? Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "GameVerse", "logs"));
    private static int Usage()
    {
        Console.Error.WriteLine("Usage: GameVerse.Launcher init|verify|start [--allow-low-memory]|update|logs|diagnostics [output.zip]|self-test|verify-update <manifest> <signature> <public-key.pem>");
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
