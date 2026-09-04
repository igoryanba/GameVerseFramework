using System.Text.Json;
using Microsoft.Web.WebView2.Core;
using Microsoft.Web.WebView2.WinForms;

namespace GameVerse.UI;

internal sealed class MainForm : Form
{
    private const string HostName = "gameverse.local";
    private const string AppOrigin = "https://gameverse.local";
    private readonly WebView2 view = new() { Dock = DockStyle.Fill };
    private readonly UiBridgeClient bridge;

    internal MainForm(string pipe)
    {
        bridge = new UiBridgeClient(pipe);
        bridge.ConnectedToBridge += BridgeConnected;
        bridge.DisconnectedFromBridge += BridgeDisconnected;
        Text = "GameVerse";
        MinimumSize = new Size(960, 640);
        Size = new Size(1280, 800);
        StartPosition = FormStartPosition.CenterScreen;
        BackColor = Color.FromArgb(9, 13, 22);
        Controls.Add(view);
        Shown += InitializeAsync;
        FormClosed += async (_, _) => await bridge.DisposeAsync();
    }

    private async void InitializeAsync(object? sender, EventArgs args)
    {
        try
        {
            string profile = Path.Combine(
                Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData),
                "GameVerse", "WebView2");
            CoreWebView2Environment environment = await CoreWebView2Environment.CreateAsync(null, profile);
            await view.EnsureCoreWebView2Async(environment);
            Configure(view.CoreWebView2);
            view.Source = new Uri($"{AppOrigin}/index.html");
            Console.WriteLine(JsonSerializer.Serialize(new { @event = "ui_ready", origin = AppOrigin }, UiJson.Options));
            Console.Out.Flush();
            _ = ConnectBridgeAsync();
        }
        catch (WebView2RuntimeNotFoundException error)
        {
            MessageBox.Show(
                "Для GameVerse нужен Microsoft Edge WebView2 Runtime. Установите официальный Evergreen Runtime и повторите запуск.\n\n" + error.Message,
                "GameVerse — требуется WebView2",
                MessageBoxButtons.OK,
                MessageBoxIcon.Error);
            Close();
        }
        catch (Exception error)
        {
            MessageBox.Show(error.Message, "GameVerse — ошибка запуска", MessageBoxButtons.OK, MessageBoxIcon.Error);
            Close();
        }
    }

    private async Task ConnectBridgeAsync()
    {
        try { await bridge.ConnectWithRetryAsync(TimeSpan.FromSeconds(30)); }
        catch (Exception error) { ShowBridgeState("failed", error.Message); }
    }

    private void BridgeConnected()
    {
        Console.WriteLine(JsonSerializer.Serialize(new { @event = "ui_bridge_ready" }, UiJson.Options));
        Console.Out.Flush();
        ShowBridgeState("bridge_ready", "Bridge подключён");
    }

    private void BridgeDisconnected()
    {
        ShowBridgeState("reconnecting", "Связь с bridge потеряна");
        _ = ConnectBridgeAsync();
    }

    private void ShowBridgeState(string stage, string message)
    {
        if (InvokeRequired) { BeginInvoke(() => ShowBridgeState(stage, message)); return; }
        if (view.CoreWebView2 is null) return;
        view.CoreWebView2.PostWebMessageAsJson(JsonSerializer.Serialize(new
        {
            schema_version = 1,
            request_id = "bridge-stage",
            ok = true,
            payload = new { stage, message, has_saved_session = TokenStore.Exists }
        }, UiJson.Options));
    }

    private void Configure(CoreWebView2 core)
    {
        string assets = Path.Combine(AppContext.BaseDirectory, "assets");
        core.SetVirtualHostNameToFolderMapping(HostName, assets, CoreWebView2HostResourceAccessKind.DenyCors);
        core.Settings.AreDefaultContextMenusEnabled = false;
        core.Settings.AreDevToolsEnabled = false;
        core.Settings.IsStatusBarEnabled = false;
        core.Settings.IsZoomControlEnabled = false;
        core.Settings.IsBuiltInErrorPageEnabled = false;
        core.Settings.IsGeneralAutofillEnabled = false;
        core.Settings.IsPasswordAutosaveEnabled = false;
        core.NavigationStarting += (_, eventArgs) => eventArgs.Cancel = !IsTrustedUri(eventArgs.Uri);
        core.FrameNavigationStarting += (_, eventArgs) => eventArgs.Cancel = !IsTrustedUri(eventArgs.Uri);
        core.NewWindowRequested += (_, eventArgs) => eventArgs.Handled = true;
        core.DownloadStarting += (_, eventArgs) => eventArgs.Cancel = true;
        core.WebMessageReceived += OnWebMessageReceived;
    }

    private static bool IsTrustedUri(string source) =>
        Uri.TryCreate(source, UriKind.Absolute, out Uri? uri)
        && uri.Scheme == Uri.UriSchemeHttps
        && uri.Host.Equals(HostName, StringComparison.OrdinalIgnoreCase)
        && uri.Port == 443;

    private async void OnWebMessageReceived(object? sender, CoreWebView2WebMessageReceivedEventArgs args)
    {
        if (!IsTrustedUri(args.Source)) return;
        string json;
        try { json = args.WebMessageAsJson; }
        catch (ArgumentException) { return; }

        if (!UiMessageValidator.TryParse(json, out UiRequest? request, out string validationError))
        {
            Send(new UiResponse(1, "invalid", false, validationError, "Некорректный запрос интерфейса", null));
            return;
        }
        try
        {
            UiRequest outbound = request!;
            if (outbound.Command == "session.reconnect")
            {
                string? storedToken = TokenStore.Load();
                if (storedToken is null)
                {
                    Send(new UiResponse(1, outbound.RequestId, false, "no_saved_session", "Сохранённая сессия отсутствует", null));
                    return;
                }
                outbound = outbound with
                {
                    Command = "auth.resume",
                    Payload = JsonSerializer.SerializeToElement(new { refresh_token = storedToken }, UiJson.Options)
                };
            }
            UiResponse response = await bridge.SendAsync(outbound);
            if (response.Ok && response.Payload is JsonElement payload
                && payload.ValueKind == JsonValueKind.Object
                && payload.TryGetProperty("refresh_token", out JsonElement token)
                && token.GetString() is string newRefreshToken)
            {
                TokenStore.Save(newRefreshToken);
                Dictionary<string, JsonElement> safePayload = payload.EnumerateObject()
                    .Where(property => property.Name != "refresh_token" && property.Name != "access_token")
                    .ToDictionary(property => property.Name, property => property.Value.Clone());
                response = response with { Payload = safePayload };
            }
            if (outbound.Command == "auth.logout" && response.Ok) TokenStore.Clear();
            Send(response);
        }
        catch (Exception error)
        {
            Send(new UiResponse(1, request!.RequestId, false, "bridge_unavailable", error.Message, null));
        }
    }

    private void Send(UiResponse response) =>
        view.CoreWebView2.PostWebMessageAsJson(JsonSerializer.Serialize(response, UiJson.Options));
}
