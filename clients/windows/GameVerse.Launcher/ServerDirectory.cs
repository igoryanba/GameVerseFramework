using System.Net;
using System.Net.Http.Json;
using System.Text.Json;

internal sealed record ServerSummary(
    string ServerId,
    string Name,
    string Description,
    string Mode,
    string Address,
    int Players,
    int MaxPlayers,
    string Status,
    string[] Tags,
    string GtaEdition,
    string GtaBuild,
    string CertificateSha256);

internal sealed record ServerDirectoryResponse(int SchemaVersion, ServerSummary[] Servers);

internal static class ServerDirectoryClient
{
    private static readonly HttpClient Http = new() { Timeout = TimeSpan.FromSeconds(5) };

    internal static async Task<IReadOnlyList<ServerSummary>> LoadAsync(LauncherConfig config, CancellationToken stopping)
    {
        if (string.IsNullOrWhiteSpace(config.DirectoryUrl)) return new[] { Local(config) };
        Uri uri = new(config.DirectoryUrl, UriKind.Absolute);
        if (uri.Scheme != Uri.UriSchemeHttps && !(uri.Scheme == Uri.UriSchemeHttp && IsLoopback(uri.Host)))
            throw new InvalidDataException("Каталог серверов должен использовать HTTPS (HTTP разрешён только для localhost)");
        ServerDirectoryResponse response = await Http.GetFromJsonAsync<ServerDirectoryResponse>(uri, UiJson.Options, stopping)
            ?? throw new InvalidDataException("Каталог серверов вернул пустой ответ");
        if (response.SchemaVersion != 1 || response.Servers.Length > 500)
            throw new InvalidDataException("Неподдерживаемая версия каталога серверов");
        foreach (ServerSummary server in response.Servers)
        {
            if (string.IsNullOrWhiteSpace(server.ServerId) || string.IsNullOrWhiteSpace(server.Name)
                || !System.Net.IPEndPoint.TryParse(server.Address, out _)
                || server.Players < 0 || server.MaxPlayers is < 1 or > 1024
                || server.Players > server.MaxPlayers
                || !System.Text.RegularExpressions.Regex.IsMatch(server.CertificateSha256, "^[A-Fa-f0-9]{64}$"))
                throw new InvalidDataException("Каталог содержит некорректную запись сервера");
        }
        return response.Servers;
    }

    internal static ServerSummary Local(LauncherConfig config) => new(
        "local-alpha", "GameVerse RP Alpha", "Локальный сервер разработки", "Roleplay",
        config.ServerAddress, 0, 32, "online", new[] { "ru", "rp", "alpha" },
        "enhanced", "1.0.1158.13", config.CertificateSha256.ToUpperInvariant());

    private static bool IsLoopback(string host) => host.Equals("localhost", StringComparison.OrdinalIgnoreCase)
        || (IPAddress.TryParse(host, out IPAddress? address) && IPAddress.IsLoopback(address));
}

internal static class LauncherPreferences
{
    private sealed record State(string? FavoriteServerId, ulong? LastCharacterId);
    private static readonly string PathName = Path.Combine(
        Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "GameVerse", "launcher-state.json");

    internal static string? FavoriteServerId => Load().FavoriteServerId;
    internal static ulong? LastCharacterId => Load().LastCharacterId;
    internal static void SaveServer(string serverId) => Save(Load() with { FavoriteServerId = serverId });
    internal static void SaveCharacter(ulong characterId) => Save(Load() with { LastCharacterId = characterId });

    private static State Load()
    {
        try { return File.Exists(PathName) ? JsonSerializer.Deserialize<State>(File.ReadAllText(PathName), UiJson.Options) ?? new(null, null) : new(null, null); }
        catch (JsonException) { return new(null, null); }
        catch (IOException) { return new(null, null); }
    }

    private static void Save(State state)
    {
        Directory.CreateDirectory(System.IO.Path.GetDirectoryName(PathName)!);
        string temporary = PathName + ".tmp";
        File.WriteAllText(temporary, JsonSerializer.Serialize(state, UiJson.Options));
        File.Move(temporary, PathName, true);
    }
}
