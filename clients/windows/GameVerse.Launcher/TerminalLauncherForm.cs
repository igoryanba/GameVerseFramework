using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Text.Json;

internal static class LauncherWindow
{
    internal static int Run(string[] args)
    {
        if (!LauncherInstance.TryAcquire(out LauncherInstance? instance)) return 0;
        using (instance)
        {
            bool uiOnly = args.Any(value => value.Equals("--ui-only", StringComparison.OrdinalIgnoreCase));
            LauncherConfig? config = null;
            if (!uiOnly)
            {
                try { config = Launcher.Load(); }
                catch (Exception error)
                {
                    MessageBox.Show(error.Message, "GameVerse", MessageBoxButtons.OK, MessageBoxIcon.Error);
                    return 2;
                }
            }
            using TerminalLauncherForm form = new(config, args);
            instance!.Listen(form.RestoreWindow);
            Application.Run(form);
            return form.ExitCode;
        }
    }
}

internal sealed class TerminalLauncherForm : Form
{
    internal static readonly string[] SupportedCommands =
    {
        "help", "login", "register", "resume", "characters", "create", "play", "status",
        "chat", "inventory", "shop", "buy", "job", "logout", "reconnect", "clear", "exit"
    };
    private enum Prompt { None, LoginName, LoginPassword, RegisterInvite, RegisterName, RegisterPassword, CharacterFirst, CharacterLast }
    private readonly RichTextBox history = new();
    private readonly TextBox input = new();
    private readonly LauncherConfig? config;
    private readonly bool allowLowMemory;
    private readonly bool attachExisting;
    private readonly CancellationTokenSource stopping = new();
    private readonly GameLaunchGate launchGate = new();
    private readonly object logSync = new();
    private readonly string logPath;
    private UiBridgeClient? bridge;
    private Process? bridgeProcess;
    private Process? playProcess;
    private Process? gameProcess;
    private bool ownsGame;
    private bool allowClose;
    private bool busy;
    private Prompt prompt;
    private string? firstValue;
    private string stage = "checking";

    internal int ExitCode { get; private set; }

    internal static bool SelfTest()
    {
        using TerminalLauncherForm form = new(null, new[] { "start", "--ui-only" });
        GameLaunchGate gate = new();
        UiRequest request = UiBridgeClient.Request("ui.ready");
        byte[] encoded = JsonSerializer.SerializeToUtf8Bytes(request, UiJson.Options);
        return form.Size == new Size(720, 440)
            && form.MinimumSize == new Size(520, 300)
            && form.FormBorderStyle == FormBorderStyle.Sizable
            && form.MinimizeBox
            && !form.MaximizeBox
            && !form.TopMost
            && form.Controls.Find("history", true).Length == 1
            && form.Controls.Find("input", true).Length == 1
            && SupportedCommands.Distinct(StringComparer.Ordinal).Count() == SupportedCommands.Length
            && gate.TryBegin()
            && !gate.TryBegin()
            && encoded.Length is > 0 and <= UiBridgeClient.MaxMessageBytes;
    }

    internal static int RenderTest(string? output)
    {
        if (string.IsNullOrWhiteSpace(output)) return 1;
        using TerminalLauncherForm form = new(null, new[] { "start", "--ui-only" });
        form.Show();
        form.PerformLayout();
        Application.DoEvents();
        using Bitmap image = new(form.ClientSize.Width, form.ClientSize.Height);
        form.DrawToBitmap(image, new Rectangle(Point.Empty, form.ClientSize));
        image.Save(Path.GetFullPath(output), System.Drawing.Imaging.ImageFormat.Png);
        form.Hide();
        return 0;
    }

    internal TerminalLauncherForm(LauncherConfig? config, string[] args)
    {
        this.config = config;
        string configuredLogs = config?.LogDirectory
            ?? Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "GameVerse", "logs");
        string logDirectory = Launcher.ResolvePath(configuredLogs);
        Directory.CreateDirectory(logDirectory);
        logPath = Path.Combine(logDirectory, $"launcher-{DateTime.UtcNow:yyyyMMdd}.jsonl");
        allowLowMemory = args.Any(value => value.Equals("--allow-low-memory", StringComparison.OrdinalIgnoreCase));
        attachExisting = args.Any(value => value.Equals("--attach-existing", StringComparison.OrdinalIgnoreCase));
        Text = "GameVerse";
        Size = new Size(720, 440);
        MinimumSize = new Size(520, 300);
        StartPosition = FormStartPosition.CenterScreen;
        FormBorderStyle = FormBorderStyle.Sizable;
        MaximizeBox = false;
        MinimizeBox = true;
        TopMost = false;

        history.Name = "history";
        history.Dock = DockStyle.Fill;
        history.ReadOnly = true;
        history.BackColor = SystemColors.Window;
        history.ForeColor = SystemColors.WindowText;
        history.BorderStyle = BorderStyle.FixedSingle;
        history.Font = new Font("Consolas", 10F);
        history.DetectUrls = false;
        history.TabStop = false;

        input.Name = "input";
        input.Dock = DockStyle.Fill;
        input.PlaceholderText = "Введите команду";
        input.Font = new Font("Consolas", 10F);
        input.BorderStyle = BorderStyle.FixedSingle;
        input.KeyDown += InputKeyDown;
        TableLayoutPanel layout = new()
        {
            Dock = DockStyle.Fill,
            ColumnCount = 1,
            RowCount = 2,
            Margin = Padding.Empty,
            Padding = Padding.Empty
        };
        layout.ColumnStyles.Add(new ColumnStyle(SizeType.Percent, 100F));
        layout.RowStyles.Add(new RowStyle(SizeType.Percent, 100F));
        layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 26F));
        layout.Controls.Add(history, 0, 0);
        layout.Controls.Add(input, 0, 1);
        Controls.Add(layout);
        Shown += (_, _) => _ = InitializeAsync();
        FormClosing += OnClosing;
    }

    internal void RestoreWindow()
    {
        if (InvokeRequired) { BeginInvoke(RestoreWindow); return; }
        Show();
        WindowState = FormWindowState.Normal;
        TopMost = true;
        BringToFront();
        Activate();
        input.Focus();
        BeginInvoke(() => TopMost = false);
    }

    private async Task InitializeAsync()
    {
        Log("GameVerse готов. Команда help показывает доступные команды.");
        if (config is null)
        {
            SetStage("auth_required", "Режим проверки интерфейса: bridge и GTA не запускаются.");
            return;
        }
        try
        {
            List<Check> checks = Launcher.Checks(config, allowLowMemory);
            foreach (Check check in checks.Where(value => !value.Passed)) Log($"Проверка {check.Name}: {check.Detail}", true);
            if (checks.Any(value => !value.Passed)) throw new InvalidOperationException("Проверка установки не пройдена.");

            Process[] existingGames = Process.GetProcessesByName("GTA5_Enhanced");
            Process[] existingLaunchers = Process.GetProcessesByName("PlayGTAV");
            if ((existingGames.Length > 0 || existingLaunchers.Length > 0) && !attachExisting)
                throw new InvalidOperationException("GTA уже запущена. GameVerse не будет запускать второй экземпляр.");

            SetStage("bridge_starting", "Запуск bridge…");
            bridgeProcess = StartBridge(config);
            await WaitForReadyEventAsync(bridgeProcess, "m2_pipe_ready", TimeSpan.FromSeconds(15), stopping.Token);
            _ = DrainBridgeAsync(bridgeProcess.StandardError, stopping.Token);
            SetStage("bridge_ready", "Bridge готов.");

            bridge = new UiBridgeClient(config.UiPipe);
            bridge.Disconnected += BridgeDisconnected;
            bridge.BridgeEvent += BridgeEvent;
            await bridge.ConnectWithRetryAsync(TimeSpan.FromSeconds(15));

            if (existingGames.Length > 0 && attachExisting)
            {
                gameProcess = existingGames[0];
                ownsGame = false;
                SetStage("game_ready", "Подключение к уже запущенной GTA разрешено режимом разработчика.");
            }
            else
            {
                await LaunchGameOnceAsync(config, existingGames.Select(value => value.Id).ToHashSet());
            }
            WindowState = FormWindowState.Minimized;
            if (gameProcess is not null) _ = WatchGameAsync(gameProcess);
            UiResponse ready = await bridge.SendAsync(
                UiBridgeClient.Request("ui.ready"),
                stopping.Token,
                config.DeveloperTelemetryStory || config.DeveloperManualStory
                    ? TimeSpan.FromMinutes(16)
                    : TimeSpan.FromMinutes(3));
            ShowResponse(ready);
        }
        catch (Exception error)
        {
            ExitCode = 4;
            SetStage("failed", error.Message);
            RestoreWindow();
        }
    }

    private static Process StartBridge(LauncherConfig config)
    {
        string bridgePath = Launcher.ResolvePath(config.BridgePath);
        ProcessStartInfo info = new()
        {
            FileName = bridgePath,
            UseShellExecute = false,
            CreateNoWindow = true,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            WorkingDirectory = Path.GetDirectoryName(bridgePath)!
        };
        foreach (string value in new[]
        {
            "--server", config.ServerAddress,
            "--cert", Launcher.ResolvePath(config.CertificatePath),
            "--ui-pipe", config.UiPipe,
            "--pipe", config.AdapterPipe,
            "--bootstrap-pipe", config.BootstrapPipe
        }) info.ArgumentList.Add(value);
        if (config.DeveloperManualStory) info.ArgumentList.Add("--manual-story");
        if (config.DeveloperTelemetryStory) info.ArgumentList.Add("--telemetry-story");
        return Process.Start(info) ?? throw new InvalidOperationException("Bridge не запустился");
    }

    private async Task LaunchGameOnceAsync(LauncherConfig value, HashSet<int> previousGameIds)
    {
        if (!launchGate.TryBegin()) throw new InvalidOperationException("Повторный запуск GTA заблокирован.");
        string gameDirectory = Launcher.ResolvePath(value.GameDirectory);
        SetStage("game_starting", "Запущен загрузчик GTA.");
        playProcess = Process.Start(new ProcessStartInfo
        {
            FileName = Path.Combine(gameDirectory, "PlayGTAV.exe"),
            WorkingDirectory = gameDirectory,
            UseShellExecute = true
        }) ?? throw new InvalidOperationException("PlayGTAV.exe не запустился");
        gameProcess = await WaitForNewGameAsync(previousGameIds, TimeSpan.FromMinutes(2), stopping.Token);
        ownsGame = true;
        SetStage("game_ready", "GTA V Enhanced готова.");
    }

    private static async Task<Process> WaitForNewGameAsync(HashSet<int> previous, TimeSpan timeout, CancellationToken stopping)
    {
        using CancellationTokenSource deadline = CancellationTokenSource.CreateLinkedTokenSource(stopping);
        deadline.CancelAfter(timeout);
        while (!deadline.IsCancellationRequested)
        {
            Process? process = Process.GetProcessesByName("GTA5_Enhanced").FirstOrDefault(value => !previous.Contains(value.Id));
            if (process is not null) return process;
            await Task.Delay(500, deadline.Token);
        }
        throw new TimeoutException("GTA V Enhanced не запустилась за две минуты");
    }

    private static async Task WaitForReadyEventAsync(Process process, string expected, TimeSpan timeout, CancellationToken stopping)
    {
        using CancellationTokenSource deadline = CancellationTokenSource.CreateLinkedTokenSource(stopping);
        deadline.CancelAfter(timeout);
        while (!deadline.IsCancellationRequested)
        {
            string? line = await process.StandardOutput.ReadLineAsync(deadline.Token);
            if (line is null) throw new InvalidOperationException("Bridge завершился до готовности");
            using JsonDocument message = JsonDocument.Parse(line);
            if (message.RootElement.TryGetProperty("event", out JsonElement value) && value.GetString() == expected) return;
        }
        throw new TimeoutException("Bridge не сообщил о готовности");
    }

    private async Task DrainBridgeAsync(StreamReader reader, CancellationToken stopping)
    {
        try
        {
            while (await reader.ReadLineAsync(stopping) is string line)
                WriteStructuredLog("bridge", line.Length <= 512 ? line : "Bridge diagnostic was truncated");
        }
        catch (OperationCanceledException) { }
    }

    private async Task WatchGameAsync(Process process)
    {
        try { await process.WaitForExitAsync(stopping.Token); }
        catch (OperationCanceledException) { return; }
        if (!IsDisposed) BeginInvoke(() => { Log("GTA завершена."); _ = StopAndCloseAsync(); });
    }

    private void InputKeyDown(object? sender, KeyEventArgs args)
    {
        if (args.KeyCode != Keys.Enter) return;
        args.SuppressKeyPress = true;
        if (busy) return;
        string value = input.Text.Trim();
        input.Clear();
        _ = HandleInputAsync(value);
    }

    private async Task HandleInputAsync(string value)
    {
        if (prompt != Prompt.None) { await HandlePromptAsync(value); return; }
        if (string.IsNullOrWhiteSpace(value)) return;
        string[] parts = value.Split(' ', 3, StringSplitOptions.RemoveEmptyEntries);
        string command = parts[0].ToLowerInvariant();
        switch (command)
        {
            case "help": Log("login | register | resume | characters | create | play <id> | status | chat <текст> | inventory | shop | buy <id> [количество] | job start | job finish | logout | reconnect | clear | exit"); break;
            case "clear": history.Clear(); break;
            case "status": Log($"Состояние: {stage}. Bridge: {(bridge?.Connected == true ? "подключён" : "нет")}. GTA: {(gameProcess is { HasExited: false } ? "запущена" : "нет")}."); break;
            case "exit": Close(); break;
            case "login": BeginPrompt(Prompt.LoginName, "Логин:"); break;
            case "register": BeginPrompt(Prompt.RegisterInvite, "Инвайт:"); break;
            case "create": BeginPrompt(Prompt.CharacterFirst, "Имя персонажа:"); break;
            case "resume": await ResumeAsync(); break;
            case "reconnect": await ResumeAsync(); break;
            case "characters": await SendAsync("characters.list", new { }); break;
            case "play" when parts.Length >= 2 && ulong.TryParse(parts[1], out ulong id): await SendAsync("characters.select", new { character_id = id }); break;
            case "chat" when parts.Length >= 2: await SendAsync("chat.send", new { message = value[(value.IndexOf(' ') + 1)..] }); break;
            case "inventory": await SendAsync("inventory.request", new { }); break;
            case "shop": await SendAsync("shop.catalog", new { }); break;
            case "buy" when parts.Length >= 2 && uint.TryParse(parts[1], out uint item):
                uint quantity = parts.Length >= 3 && uint.TryParse(parts[2], out uint parsed) ? parsed : 1;
                await SendAsync("shop.buy", new { item_id = item, quantity }); break;
            case "job" when parts.Length >= 2 && parts[1].Equals("start", StringComparison.OrdinalIgnoreCase): await SendAsync("job.start", new { }); break;
            case "job" when parts.Length >= 2 && parts[1].Equals("finish", StringComparison.OrdinalIgnoreCase): await SendAsync("job.finish", new { }); break;
            case "logout":
                if ((await SendAsync("auth.logout", new { }))?.Ok == true) TokenStore.Clear();
                break;
            default: Log("Неизвестная команда. Введите help.", true); break;
        }
    }

    private void BeginPrompt(Prompt next, string message)
    {
        prompt = next;
        firstValue = null;
        input.UseSystemPasswordChar = false;
        Log(message);
        input.Focus();
    }

    private async Task HandlePromptAsync(string value)
    {
        switch (prompt)
        {
            case Prompt.LoginName: firstValue = value; prompt = Prompt.LoginPassword; input.UseSystemPasswordChar = true; Log("Пароль:"); return;
            case Prompt.LoginPassword:
                input.UseSystemPasswordChar = false; prompt = Prompt.None;
                string login = firstValue ?? ""; firstValue = null;
                await SendAsync("auth.login", new { login, password = value }); return;
            case Prompt.RegisterInvite: firstValue = value; prompt = Prompt.RegisterName; Log("Логин:"); return;
            case Prompt.RegisterName:
                string invite = firstValue ?? ""; firstValue = invite + "\n" + value; prompt = Prompt.RegisterPassword; input.UseSystemPasswordChar = true; Log("Пароль:"); return;
            case Prompt.RegisterPassword:
                input.UseSystemPasswordChar = false; prompt = Prompt.None;
                string[] registration = (firstValue ?? "\n").Split('\n', 2); firstValue = null;
                await SendAsync("auth.register", new { invite = registration[0], login = registration[1], password = value }); return;
            case Prompt.CharacterFirst: firstValue = value; prompt = Prompt.CharacterLast; Log("Фамилия персонажа:"); return;
            case Prompt.CharacterLast:
                prompt = Prompt.None; string firstName = firstValue ?? ""; firstValue = null;
                await SendAsync("characters.create", new { first_name = firstName, last_name = value }); return;
        }
    }

    private async Task ResumeAsync()
    {
        string? token = TokenStore.Load();
        if (token is null) { Log("Сохранённая сессия отсутствует.", true); return; }
        await SendAsync("auth.resume", new { refresh_token = token });
    }

    private async Task<UiResponse?> SendAsync(string command, object payload)
    {
        if (bridge?.Connected != true) { Log("Bridge не подключён.", true); return null; }
        busy = true; input.Enabled = false;
        try
        {
            UiResponse response = await bridge.SendAsync(UiBridgeClient.Request(command, payload), stopping.Token);
            ShowResponse(response);
            return response;
        }
        catch (Exception error) { SetStage("failed", error.Message); return null; }
        finally { busy = false; input.Enabled = true; input.Focus(); }
    }

    private void ShowResponse(UiResponse response)
    {
        if (!response.Ok) { Log($"{response.ErrorCode}: {response.Message}", true); return; }
        JsonElement payload = response.Payload;
        if (payload.ValueKind == JsonValueKind.Object && payload.TryGetProperty("refresh_token", out JsonElement token) && token.GetString() is string refresh)
            TokenStore.Save(refresh);
        if (payload.ValueKind == JsonValueKind.Object && payload.TryGetProperty("stage", out JsonElement nextStage))
            SetStage(nextStage.GetString() ?? stage, StageText(nextStage.GetString()));
        if (payload.ValueKind == JsonValueKind.Object && payload.TryGetProperty("characters", out JsonElement characters))
        {
            foreach (JsonElement character in characters.EnumerateArray())
                Log($"Персонаж {character.GetProperty("id")}: {character.GetProperty("first_name").GetString()} {character.GetProperty("last_name").GetString()}");
            return;
        }
        if (payload.ValueKind == JsonValueKind.Object && payload.TryGetProperty("items", out JsonElement items))
        {
            foreach (JsonElement item in items.EnumerateArray()) Log(item.ToString());
            return;
        }
        if (response.Message is not null) Log(response.Message);
        else if (payload.ValueKind == JsonValueKind.Object && payload.EnumerateObject().Any())
            Log(Redact(payload));
    }

    private void BridgeEvent(UiResponse response)
    {
        if (InvokeRequired) { BeginInvoke(() => BridgeEvent(response)); return; }
        if (response.Payload.ValueKind != JsonValueKind.Object) return;
        string next = response.Payload.TryGetProperty("stage", out JsonElement value) ? value.GetString() ?? stage : stage;
        string message = response.Payload.TryGetProperty("message", out JsonElement text) ? text.GetString() ?? StageText(next) : StageText(next);
        SetStage(next, message);
    }

    private void BridgeDisconnected()
    {
        if (InvokeRequired) { BeginInvoke(BridgeDisconnected); return; }
        SetStage("reconnecting", "Связь с bridge потеряна.");
        RestoreWindow();
    }

    private void SetStage(string value, string message)
    {
        if (InvokeRequired) { BeginInvoke(() => SetStage(value, message)); return; }
        stage = value;
        Log(message, value == "failed");
        if (value is "auth_required" or "character_required")
        {
            MinimizeGameForInteraction();
            RestoreWindow();
        }
        else if (value is "failed" or "reconnecting") RestoreWindow();
        if (value == "active")
        {
            RestoreGameAfterInteraction();
            WindowState = FormWindowState.Minimized;
        }
    }

    private void MinimizeGameForInteraction()
    {
        try
        {
            if (gameProcess is null || gameProcess.HasExited) return;
            gameProcess.Refresh();
            if (gameProcess.MainWindowHandle != IntPtr.Zero)
                NativeWindow.ShowWindowAsync(gameProcess.MainWindowHandle, NativeWindow.SwMinimize);
        }
        catch (InvalidOperationException) { }
        catch (System.ComponentModel.Win32Exception) { }
    }

    private void RestoreGameAfterInteraction()
    {
        try
        {
            if (gameProcess is null || gameProcess.HasExited) return;
            gameProcess.Refresh();
            if (gameProcess.MainWindowHandle != IntPtr.Zero)
                NativeWindow.ShowWindowAsync(gameProcess.MainWindowHandle, NativeWindow.SwRestore);
        }
        catch (InvalidOperationException) { }
        catch (System.ComponentModel.Win32Exception) { }
    }

    private static string StageText(string? value) => value switch
    {
        "auth_required" => "Войдите в аккаунт командой login или register.",
        "character_required" => "Выберите персонажа командой play <id>.",
        "spawning" => "Загрузка персонажа…",
        "active" => "Сессия активна.",
        _ => value ?? "Готово"
    };

    private static string Redact(JsonElement payload)
    {
        Dictionary<string, JsonElement> safe = payload.EnumerateObject()
            .Where(value => value.Name is not "refresh_token" and not "access_token" and not "password")
            .ToDictionary(value => value.Name, value => value.Value.Clone());
        return JsonSerializer.Serialize(safe, UiJson.Options);
    }

    private void Log(string message, bool error = false)
    {
        if (InvokeRequired) { BeginInvoke(() => Log(message, error)); return; }
        history.AppendText($"[{DateTime.Now:HH:mm:ss}] {(error ? "ОШИБКА" : "INFO")} {message}{Environment.NewLine}");
        history.SelectionStart = history.TextLength;
        history.ScrollToCaret();
        WriteStructuredLog(error ? "error" : "info", message);
    }

    private void WriteStructuredLog(string level, string message)
    {
        string safe = System.Text.RegularExpressions.Regex.Replace(
            message,
            "(?i)(password|refresh_token|access_token|authorization)(\\s*[:=]\\s*)[^\\s,;]+",
            "$1$2[REDACTED]");
        string line = JsonSerializer.Serialize(new
        {
            timestamp = DateTimeOffset.UtcNow,
            component = "launcher",
            level,
            stage,
            message = safe
        }, UiJson.Options);
        try { lock (logSync) File.AppendAllText(logPath, line + Environment.NewLine); }
        catch (IOException) { }
        catch (UnauthorizedAccessException) { }
    }

    private void OnClosing(object? sender, FormClosingEventArgs args)
    {
        if (allowClose) return;
        bool active = bridgeProcess is { HasExited: false } || gameProcess is { HasExited: false };
        if (active && MessageBox.Show("Завершить GameVerse и запущенную им GTA?", "GameVerse", MessageBoxButtons.YesNo, MessageBoxIcon.Question) != DialogResult.Yes)
        {
            args.Cancel = true;
            return;
        }
        args.Cancel = true;
        _ = StopAndCloseAsync();
    }

    private async Task StopAndCloseAsync()
    {
        if (allowClose) return;
        allowClose = true;
        SetStage("stopping", "Завершение GameVerse…");
        if (bridge?.Connected == true)
            try { await bridge.SendAsync(UiBridgeClient.Request("session.end", new { reason = "launcher_closed" })); } catch (Exception) { }
        stopping.Cancel();
        if (ownsGame) KillOwned(gameProcess);
        KillOwned(playProcess);
        KillOwned(bridgeProcess);
        if (bridge is not null) await bridge.DisposeAsync();
        BeginInvoke(Close);
    }

    private static void KillOwned(Process? process)
    {
        if (process is null) return;
        try { if (!process.HasExited) process.Kill(entireProcessTree: true); }
        catch (InvalidOperationException) { }
        catch (System.ComponentModel.Win32Exception) { }
        finally { process.Dispose(); }
    }
}

internal static class NativeWindow
{
    internal const int SwMinimize = 6;
    internal const int SwRestore = 9;

    [DllImport("user32.dll")]
    [return: MarshalAs(UnmanagedType.Bool)]
    internal static extern bool ShowWindowAsync(IntPtr window, int command);
}
