using System.Text.Json;
using System.Text.RegularExpressions;

namespace GameVerse.UI;

internal sealed record UiRequest(int SchemaVersion, string RequestId, string Command, JsonElement Payload);

internal sealed record UiResponse(
    int SchemaVersion,
    string RequestId,
    bool Ok,
    string? ErrorCode,
    string? Message,
    object? Payload);

internal static partial class UiMessageValidator
{
    internal const int MaxMessageBytes = 64 * 1024;
    internal static readonly HashSet<string> Commands = new(StringComparer.Ordinal)
    {
        "ui.ready",
        "auth.resume",
        "auth.logout",
        "auth.login",
        "auth.register",
        "characters.list",
        "characters.create",
        "characters.select",
        "chat.send",
        "inventory.request",
        "shop.catalog",
        "shop.buy",
        "job.start",
        "job.finish",
        "session.reconnect"
    };

    [GeneratedRegex("^[A-Za-z0-9][A-Za-z0-9._:-]{0,95}$", RegexOptions.CultureInvariant)]
    private static partial Regex RequestIdPattern();

    internal static bool TryParse(string json, out UiRequest? request, out string error)
    {
        request = null;
        error = "invalid_message";
        if (System.Text.Encoding.UTF8.GetByteCount(json) > MaxMessageBytes)
        {
            error = "message_too_large";
            return false;
        }

        try
        {
            using JsonDocument document = JsonDocument.Parse(json, new JsonDocumentOptions
            {
                AllowTrailingCommas = false,
                CommentHandling = JsonCommentHandling.Disallow,
                MaxDepth = 16
            });
            JsonElement root = document.RootElement;
            if (root.ValueKind != JsonValueKind.Object
                || !root.TryGetProperty("schema_version", out JsonElement schema)
                || schema.ValueKind != JsonValueKind.Number
                || !schema.TryGetInt32(out int version)
                || version != 1)
            {
                error = "unsupported_schema";
                return false;
            }
            if (!root.TryGetProperty("request_id", out JsonElement idValue)
                || idValue.ValueKind != JsonValueKind.String
                || idValue.GetString() is not string requestId
                || !RequestIdPattern().IsMatch(requestId))
            {
                error = "invalid_request_id";
                return false;
            }
            if (!root.TryGetProperty("command", out JsonElement commandValue)
                || commandValue.ValueKind != JsonValueKind.String
                || commandValue.GetString() is not string command
                || !Commands.Contains(command))
            {
                error = "unsupported_command";
                return false;
            }
            JsonElement payload = root.TryGetProperty("payload", out JsonElement payloadValue)
                ? payloadValue.Clone()
                : JsonDocument.Parse("{}").RootElement.Clone();
            if (payload.ValueKind != JsonValueKind.Object)
            {
                error = "invalid_payload";
                return false;
            }
            request = new UiRequest(version, requestId, command, payload);
            return true;
        }
        catch (JsonException)
        {
            return false;
        }
    }

    internal static int RunSelfTest()
    {
        const string valid = "{\"schema_version\":1,\"request_id\":\"self-test-1\",\"command\":\"ui.ready\",\"payload\":{}}";
        bool accepted = TryParse(valid, out UiRequest? request, out _)
            && request?.Command == "ui.ready";
        bool rejectedOriginlessCommand = !TryParse("{\"schema_version\":1,\"request_id\":\"x\",\"command\":\"process.start\",\"payload\":{}}", out _, out _);
        bool rejectedSchema = !TryParse("{\"schema_version\":2,\"request_id\":\"x\",\"command\":\"ui.ready\",\"payload\":{}}", out _, out _);
        bool rejectedOversize = !TryParse(valid + new string(' ', MaxMessageBytes), out _, out _);
        bool assetsPresent = new[] { "index.html", "app.css", "app.js", Path.Combine("locales", "ru-RU.json") }
            .All(file => File.Exists(Path.Combine(AppContext.BaseDirectory, "assets", file)));
        bool tokenRoundtrip = TokenStore.SelfTest();
        bool passed = accepted && rejectedOriginlessCommand && rejectedSchema && rejectedOversize && assetsPresent && tokenRoundtrip;
        Console.WriteLine(JsonSerializer.Serialize(new
        {
            status = passed ? "passed" : "failed",
            accepted,
            rejected_unsupported_command = rejectedOriginlessCommand,
            rejected_schema = rejectedSchema,
            rejected_oversize = rejectedOversize,
            assets_present = assetsPresent,
            dpapi_roundtrip = tokenRoundtrip
        }, UiJson.Options));
        return passed ? 0 : 1;
    }
}
