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
    private readonly DataGridView servers = new();
    private readonly TextBox search = new();
    private readonly Button playButton = new();
    private readonly Button loginButton = new();
    private readonly Button registerButton = new();
    private readonly Button characterButton = new();
    private readonly Button refreshButton = new();
    private readonly Button diagnosticsButton = new();
    private readonly Label statusLabel = new();
    private readonly List<ServerSummary> directory = new();
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
    private HashSet<int> previousGameIds = new();
    private bool reservationLaunchStarted;
    private string? launchBlockReason;

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
            && form.Controls.Find("servers", true).Length == 1
            && form.Controls.Find("play", true).Length == 1
            && SupportedCommands.Distinct(StringComparer.Ordinal).Count() == SupportedCommands.Length
            && gate.TryBegin()
            && !gate.TryBegin()
            && encoded.Length is > 0 and <= UiBridgeClient.MaxMessageBytes;
    }

    internal static int RenderTest(string? output)
    {
        if (string.IsNullOrWhiteSpace(output)) return 1;
        using TerminalLauncherForm form = new(null, new[] { "start", "--ui-only" });
        form.directory.Add(new ServerSummary(
            "preview", "GameVerse RP Alpha", "Закрытая RP-альфа", "Roleplay", "127.0.0.1:30122",
            7, 32, "online", new[] { "ru", "rp" }, "enhanced", "1.0.1158.13", new string('0', 64)));
        form.ApplyServerFilter();
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
        search.Name = "search";
        search.PlaceholderText = "Поиск серверов";
        search.Dock = DockStyle.Fill;
        search.TextChanged += (_, _) => ApplyServerFilter();

        servers.Name = "servers";
        servers.Dock = DockStyle.Fill;
        servers.ReadOnly = true;
        servers.AllowUserToAddRows = false;
        servers.AllowUserToDeleteRows = false;
        servers.AllowUserToResizeRows = false;
        servers.AutoGenerateColumns = false;
        servers.MultiSelect = false;
        servers.SelectionMode = DataGridViewSelectionMode.FullRowSelect;
        servers.RowHeadersVisible = false;
        servers.BackgroundColor = SystemColors.Window;
        servers.Columns.Add(new DataGridViewTextBoxColumn { HeaderText = "Сервер", DataPropertyName = nameof(ServerSummary.Name), AutoSizeMode = DataGridViewAutoSizeColumnMode.Fill });
        servers.Columns.Add(new DataGridViewTextBoxColumn { HeaderText = "Режим", DataPropertyName = nameof(ServerSummary.Mode), Width = 100 });
        servers.Columns.Add(new DataGridViewTextBoxColumn { HeaderText = "Игроки", DataPropertyName = "PlayersText", Width = 70 });
        servers.Columns.Add(new DataGridViewTextBoxColumn { HeaderText = "Статус", DataPropertyName = nameof(ServerSummary.Status), Width = 80 });
        servers.SelectionChanged += (_, _) => UpdateServerSelection();

        playButton.Name = "play";
        playButton.Text = "Играть";
        playButton.AutoSize = true;
        playButton.Enabled = false;
        playButton.Click += async (_, _) => await StartPreflightAsync();
        loginButton.Text = "Войти";
        loginButton.AutoSize = true;
        loginButton.Click += async (_, _) => await ShowLoginAsync();
        registerButton.Text = "Регистрация";
        registerButton.AutoSize = true;
        registerButton.Click += async (_, _) => await ShowRegistrationAsync();
        characterButton.Text = "Персонаж";
        characterButton.AutoSize = true;
        characterButton.Click += async (_, _) => await ShowCharacterAsync();
        refreshButton.Text = "Обновить";
        refreshButton.AutoSize = true;
        refreshButton.Click += async (_, _) => await RefreshServersAsync();
        diagnosticsButton.Text = "Диагностика";
        diagnosticsButton.AutoSize = true;
        diagnosticsButton.Click += (_, _) => new DiagnosticsForm(history.Text).Show(this);
        statusLabel.AutoEllipsis = true;
        statusLabel.Dock = DockStyle.Fill;
        statusLabel.TextAlign = ContentAlignment.MiddleLeft;

        FlowLayoutPanel actions = new() { Dock = DockStyle.Fill, AutoSize = true, FlowDirection = FlowDirection.LeftToRight, WrapContents = false };
        actions.Controls.AddRange(new Control[] { playButton, loginButton, registerButton, characterButton, refreshButton, diagnosticsButton });
        TableLayoutPanel layout = new()
        {
            Dock = DockStyle.Fill,
            ColumnCount = 1, RowCount = 4,
            Margin = new Padding(8), Padding = new Padding(8)
        };
        layout.ColumnStyles.Add(new ColumnStyle(SizeType.Percent, 100F));
        layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 30F));
        layout.RowStyles.Add(new RowStyle(SizeType.Percent, 100F));
        layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 40F));
        layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 28F));
        layout.Controls.Add(search, 0, 0);
        layout.Controls.Add(servers, 0, 1);
        layout.Controls.Add(actions, 0, 2);
        layout.Controls.Add(statusLabel, 0, 3);
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
        Log("GameVerse launcher готов.");
        if (config is null)
        {
            SetStage("auth_required", "Режим проверки интерфейса: bridge и GTA не запускаются.");
            return;
        }
        try
        {
            List<Check> checks = Launcher.Checks(config, allowLowMemory);
            foreach (Check check in checks.Where(value => !value.Passed)) Log($"Проверка {check.Name}: {check.Detail}", true);
            Check? bootstrap = checks.FirstOrDefault(value => value.Name == "bootstrap_compatibility");
            if (bootstrap is { Passed: false }) launchBlockReason = bootstrap.Detail;
            if (checks.Any(value => !value.Passed && value.Name != "bootstrap_compatibility"))
                throw new InvalidOperationException("Проверка установки не пройдена.");

            await RefreshServersAsync();
            SetStage("idle", launchBlockReason is null
                ? "Выберите сервер и нажмите «Играть»."
                : $"Автоматический запуск временно недоступен: {launchBlockReason}");
        }
        catch (Exception error)
        {
            ExitCode = 4;
            SetStage("failed", error.Message);
            RestoreWindow();
        }
    }

    private async Task RefreshServersAsync()
    {
        if (config is null) return;
        SetStage("refreshing_servers", "Обновление списка серверов…");
        try
        {
            IReadOnlyList<ServerSummary> loaded = await ServerDirectoryClient.LoadAsync(config, stopping.Token);
            directory.Clear(); directory.AddRange(loaded);
            ApplyServerFilter();
            SetStage("idle", loaded.Count == 0 ? "Доступных серверов нет."
                : launchBlockReason is null ? "Выберите сервер и нажмите «Играть»."
                : $"Автоматический запуск временно недоступен: {launchBlockReason}");
        }
        catch (Exception error)
        {
            directory.Clear(); directory.Add(ServerDirectoryClient.Local(config));
            ApplyServerFilter();
            SetStage("idle", $"Каталог недоступен; показан локальный сервер. {error.Message}");
        }
    }

    private void ApplyServerFilter()
    {
        string filter = search.Text.Trim();
        var rows = directory.Where(value => filter.Length == 0 || value.Name.Contains(filter, StringComparison.CurrentCultureIgnoreCase)
            || value.Mode.Contains(filter, StringComparison.CurrentCultureIgnoreCase) || value.Tags.Any(tag => tag.Contains(filter, StringComparison.OrdinalIgnoreCase)))
            .Select(value => new ServerRow(value)).ToList();
        servers.DataSource = rows;
        string? favorite = LauncherPreferences.FavoriteServerId;
        if (favorite is not null)
            foreach (DataGridViewRow row in servers.Rows)
                if (row.DataBoundItem is ServerRow item && item.Server.ServerId == favorite) { row.Selected = true; break; }
        UpdateServerSelection();
    }

    private void UpdateServerSelection()
    {
        ServerSummary? selected = SelectedServer();
        playButton.Enabled = selected is { Status: "online" } && bridgeProcess is null && launchBlockReason is null;
        if (selected is not null) statusLabel.Text = $"{selected.Description}  •  {selected.Players}/{selected.MaxPlayers}  •  GTA {selected.GtaBuild}";
    }

    private ServerSummary? SelectedServer() => servers.CurrentRow?.DataBoundItem is ServerRow row ? row.Server : null;

    private async Task StartPreflightAsync()
    {
        if (config is null || SelectedServer() is not ServerSummary selected || busy) return;
        busy = true; playButton.Enabled = false;
        try
        {
            if (!selected.Address.Equals(config.ServerAddress, StringComparison.OrdinalIgnoreCase)
                || !selected.CertificateSha256.Equals(config.CertificateSha256, StringComparison.OrdinalIgnoreCase))
                throw new InvalidOperationException("Для выбранного сервера отсутствует закреплённая конфигурация подключения.");
            Process[] existingGames = Process.GetProcessesByName("GTA5_Enhanced");
            Process[] existingLaunchers = Process.GetProcessesByName("PlayGTAV");
            if ((existingGames.Length > 0 || existingLaunchers.Length > 0) && !attachExisting)
                throw new InvalidOperationException("GTA уже запущена. Закройте её перед подключением к GameVerse.");
            previousGameIds = existingGames.Select(value => value.Id).ToHashSet();
            LauncherPreferences.SaveServer(selected.ServerId);
            SetStage("bridge_starting", "Подготовка подключения…");
            bridgeProcess = StartBridge(config, selected.Address);
            await WaitForReadyEventAsync(bridgeProcess, "m2_pipe_ready", TimeSpan.FromSeconds(15), stopping.Token);
            _ = DrainBridgeAsync(bridgeProcess.StandardError, stopping.Token);
            bridge = new UiBridgeClient(config.UiPipe);
            bridge.Disconnected += BridgeDisconnected;
            bridge.BridgeEvent += BridgeEvent;
            await bridge.ConnectWithRetryAsync(TimeSpan.FromSeconds(15));
            ShowResponse(await bridge.SendAsync(UiBridgeClient.Request("ui.ready"), stopping.Token));
            if (TokenStore.Exists)
            {
                string? token = TokenStore.Load();
                if (token is not null)
                {
                    UiResponse resumed = await bridge.SendAsync(UiBridgeClient.Request("auth.resume", new { refresh_token = token }), stopping.Token);
                    ShowResponse(resumed);
                    if (resumed.Ok) await RequestCharactersAsync();
                }
            }
            else SetStage("auth_required", "Войдите или зарегистрируйтесь. GTA ещё не запущена.");
        }
        catch (Exception error) { SetStage("failed", error.Message); }
        finally { busy = false; UpdateServerSelection(); }
    }

    private async Task ShowLoginAsync()
    {
        if (bridge?.Connected != true) { SetStage("failed", "Сначала выберите сервер и нажмите «Играть»."); return; }
        using LoginDialog dialog = new();
        if (dialog.ShowDialog(this) != DialogResult.OK) return;
        UiResponse? response = await SendAsync("auth.login", new { login = dialog.Login, password = dialog.Password });
        dialog.ClearPassword();
        if (response?.Ok == true) await RequestCharactersAsync();
    }

    private async Task ShowRegistrationAsync()
    {
        if (bridge?.Connected != true) { SetStage("failed", "Сначала выберите сервер и нажмите «Играть»."); return; }
        using RegistrationDialog dialog = new();
        if (dialog.ShowDialog(this) != DialogResult.OK) return;
        UiResponse? response = await SendAsync("auth.register", new { invite = dialog.Invite, login = dialog.Login, password = dialog.Password });
        dialog.ClearPassword();
        if (response?.Ok == true) await RequestCharactersAsync();
    }

    private async Task ShowCharacterAsync()
    {
        if (bridge?.Connected != true) { SetStage("failed", "Сначала подключитесь к серверу."); return; }
        using CharacterDialog dialog = new();
        if (dialog.ShowDialog(this) != DialogResult.OK) return;
        UiResponse? response = await SendAsync("characters.create", new { first_name = dialog.FirstName, last_name = dialog.LastName });
        if (response?.Ok == true) await RequestCharactersAsync();
    }

    private async Task RequestCharactersAsync()
    {
        UiResponse? response = await SendAsync("characters.list", new { });
        if (response?.Ok != true || response.Payload.ValueKind != JsonValueKind.Object
            || !response.Payload.TryGetProperty("characters", out JsonElement list)) return;
        JsonElement[] values = list.EnumerateArray().ToArray();
        if (values.Length == 0) { SetStage("character_required", "Создайте первого персонажа кнопкой «Персонаж»."); return; }
        ulong? preferred = LauncherPreferences.LastCharacterId;
        JsonElement selected = values.FirstOrDefault(value => preferred.HasValue && value.GetProperty("id").GetUInt64() == preferred.Value);
        if (selected.ValueKind == JsonValueKind.Undefined) selected = values[0];
        ulong id = selected.GetProperty("id").GetUInt64();
        LauncherPreferences.SaveCharacter(id);
        await SendAsync("characters.select", new { character_id = id });
    }

    private sealed record ServerRow(ServerSummary Server)
    {
        public string Name => Server.Name;
        public string Mode => Server.Mode;
        public string PlayersText => $"{Server.Players}/{Server.MaxPlayers}";
        public string Status => Server.Status == "online" ? "Доступен" : "Недоступен";
    }

    private static Process StartBridge(LauncherConfig config, string serverAddress)
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
            "--server", serverAddress,
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
        statusLabel.Text = message;
        Log(message, value == "failed");
        if (value is "auth_required" or "character_required")
        {
            MinimizeGameForInteraction();
            RestoreWindow();
        }
        else if (value is "failed" or "reconnecting") RestoreWindow();
        else if (value == "reserved" && !reservationLaunchStarted)
        {
            reservationLaunchStarted = true;
            _ = LaunchReservedGameAsync();
        }
        if (value == "active")
        {
            RestoreGameAfterInteraction();
            WindowState = FormWindowState.Minimized;
        }
    }

    private async Task LaunchReservedGameAsync()
    {
        if (config is null) return;
        try
        {
            if (attachExisting && Process.GetProcessesByName("GTA5_Enhanced").FirstOrDefault() is Process existing)
            {
                gameProcess = existing;
                ownsGame = false;
                SetStage("game_ready", "Подключение к запущенной GTA разрешено developer-флагом.");
            }
            else await LaunchGameOnceAsync(config, previousGameIds);
            WindowState = FormWindowState.Minimized;
            if (gameProcess is not null) _ = WatchGameAsync(gameProcess);
        }
        catch (Exception error)
        {
            reservationLaunchStarted = false;
            SetStage("failed", error.Message);
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
        "auth_required" => "Войдите или зарегистрируйтесь. GTA ещё не запущена.",
        "character_required" => "Подготовка персонажа…",
        "reserved" => "Персонаж выбран. Запуск GTA…",
        "world_loading" => "Загрузка игрового мира…",
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

internal sealed class LoginDialog : Form
{
    private readonly TextBox login = new();
    private readonly TextBox password = new();
    internal string Login => login.Text.Trim();
    internal string Password => password.Text;

    internal LoginDialog()
    {
        Text = "Вход в GameVerse";
        ClientSize = new Size(360, 145);
        FormBorderStyle = FormBorderStyle.FixedDialog;
        StartPosition = FormStartPosition.CenterParent;
        MaximizeBox = false;
        MinimizeBox = false;
        ShowInTaskbar = false;
        login.Dock = DockStyle.Fill;
        password.Dock = DockStyle.Fill;
        password.UseSystemPasswordChar = true;
        Button ok = new() { Text = "Войти", DialogResult = DialogResult.OK, AutoSize = true };
        Button cancel = new() { Text = "Отмена", DialogResult = DialogResult.Cancel, AutoSize = true };
        FlowLayoutPanel buttons = new() { Dock = DockStyle.Fill, FlowDirection = FlowDirection.RightToLeft };
        buttons.Controls.Add(cancel); buttons.Controls.Add(ok);
        TableLayoutPanel layout = new() { Dock = DockStyle.Fill, Padding = new Padding(10), ColumnCount = 2, RowCount = 3 };
        layout.ColumnStyles.Add(new ColumnStyle(SizeType.Absolute, 75));
        layout.ColumnStyles.Add(new ColumnStyle(SizeType.Percent, 100));
        layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 32));
        layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 32));
        layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 42));
        layout.Controls.Add(new Label { Text = "Логин", TextAlign = ContentAlignment.MiddleLeft, Dock = DockStyle.Fill }, 0, 0);
        layout.Controls.Add(login, 1, 0);
        layout.Controls.Add(new Label { Text = "Пароль", TextAlign = ContentAlignment.MiddleLeft, Dock = DockStyle.Fill }, 0, 1);
        layout.Controls.Add(password, 1, 1);
        layout.Controls.Add(buttons, 0, 2);
        layout.SetColumnSpan(buttons, 2);
        Controls.Add(layout);
        AcceptButton = ok;
        CancelButton = cancel;
    }

    internal void ClearPassword()
    {
        password.Clear();
    }
}

internal sealed class DiagnosticsForm : Form
{
    internal DiagnosticsForm(string text)
    {
        Text = "GameVerse — диагностика";
        Size = new Size(720, 360);
        StartPosition = FormStartPosition.CenterParent;
        RichTextBox output = new()
        {
            Dock = DockStyle.Fill,
            ReadOnly = true,
            Font = new Font("Consolas", 9F),
            Text = text,
            BackColor = SystemColors.Window,
            ForeColor = SystemColors.WindowText
        };
        Controls.Add(output);
    }
}

internal sealed class RegistrationDialog : Form
{
    private readonly TextBox invite = new();
    private readonly TextBox login = new();
    private readonly TextBox password = new() { UseSystemPasswordChar = true };
    internal string Invite => invite.Text.Trim();
    internal string Login => login.Text.Trim();
    internal string Password => password.Text;

    internal RegistrationDialog()
    {
        Text = "Регистрация в GameVerse";
        ClientSize = new Size(390, 185);
        FormBorderStyle = FormBorderStyle.FixedDialog;
        StartPosition = FormStartPosition.CenterParent;
        MaximizeBox = false; MinimizeBox = false; ShowInTaskbar = false;
        Button ok = new() { Text = "Создать аккаунт", DialogResult = DialogResult.OK, AutoSize = true };
        Button cancel = new() { Text = "Отмена", DialogResult = DialogResult.Cancel, AutoSize = true };
        TableLayoutPanel layout = DialogLayout(new[] { ("Инвайт", invite), ("Логин", login), ("Пароль", password) }, ok, cancel);
        Controls.Add(layout); AcceptButton = ok; CancelButton = cancel;
    }

    internal void ClearPassword() => password.Clear();

    internal static TableLayoutPanel DialogLayout((string Label, TextBox Input)[] fields, Button ok, Button cancel)
    {
        TableLayoutPanel layout = new() { Dock = DockStyle.Fill, Padding = new Padding(10), ColumnCount = 2, RowCount = fields.Length + 1 };
        layout.ColumnStyles.Add(new ColumnStyle(SizeType.Absolute, 90));
        layout.ColumnStyles.Add(new ColumnStyle(SizeType.Percent, 100));
        for (int index = 0; index < fields.Length; index++)
        {
            fields[index].Input.Dock = DockStyle.Fill;
            layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 32));
            layout.Controls.Add(new Label { Text = fields[index].Label, TextAlign = ContentAlignment.MiddleLeft, Dock = DockStyle.Fill }, 0, index);
            layout.Controls.Add(fields[index].Input, 1, index);
        }
        FlowLayoutPanel buttons = new() { Dock = DockStyle.Fill, FlowDirection = FlowDirection.RightToLeft };
        buttons.Controls.Add(cancel); buttons.Controls.Add(ok);
        layout.RowStyles.Add(new RowStyle(SizeType.Absolute, 42));
        layout.Controls.Add(buttons, 0, fields.Length); layout.SetColumnSpan(buttons, 2);
        return layout;
    }
}

internal sealed class CharacterDialog : Form
{
    private readonly TextBox firstName = new();
    private readonly TextBox lastName = new();
    internal string FirstName => firstName.Text.Trim();
    internal string LastName => lastName.Text.Trim();

    internal CharacterDialog()
    {
        Text = "Новый персонаж";
        ClientSize = new Size(390, 150);
        FormBorderStyle = FormBorderStyle.FixedDialog;
        StartPosition = FormStartPosition.CenterParent;
        MaximizeBox = false; MinimizeBox = false; ShowInTaskbar = false;
        Button ok = new() { Text = "Создать", DialogResult = DialogResult.OK, AutoSize = true };
        Button cancel = new() { Text = "Отмена", DialogResult = DialogResult.Cancel, AutoSize = true };
        Controls.Add(RegistrationDialog.DialogLayout(new[] { ("Имя", firstName), ("Фамилия", lastName) }, ok, cancel));
        AcceptButton = ok; CancelButton = cancel;
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
