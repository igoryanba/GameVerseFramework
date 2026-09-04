using System.Text.Json;

namespace GameVerse.UI;

internal static class Program
{
    [STAThread]
    private static int Main(string[] args)
    {
        if (args.Contains("--self-test", StringComparer.OrdinalIgnoreCase))
            return UiMessageValidator.RunSelfTest();

        string pipe = Argument(args, "--pipe") ?? "gameverse-ui-v1";
        ApplicationConfiguration.Initialize();
        Application.Run(new MainForm(pipe));
        return 0;
    }

    private static string? Argument(string[] args, string name)
    {
        int index = Array.FindIndex(args, value => value.Equals(name, StringComparison.OrdinalIgnoreCase));
        return index >= 0 && index + 1 < args.Length ? args[index + 1] : null;
    }
}

internal static class UiJson
{
    internal static readonly JsonSerializerOptions Options = new(JsonSerializerDefaults.Web)
    {
        PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower
    };
}
