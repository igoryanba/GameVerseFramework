using System.Security.Cryptography;
using System.Text.Json;
using System.Text.RegularExpressions;

internal sealed record UpdateFile(string Path, long Size, string Sha256, Uri Url);

internal sealed record VerifiedUpdate(string Version, string Channel, string MinimumLauncherVersion, IReadOnlyList<UpdateFile> Files);

internal static class UpdateSecurity
{
    private const int MaxManifestBytes = 1024 * 1024;
    private const int MaxFiles = 10_000;

    internal static VerifiedUpdate Verify(byte[] manifest, byte[] signature, string publicKeyPem)
    {
        if (manifest.Length is 0 or > MaxManifestBytes) throw new InvalidDataException("update manifest size is invalid");
        using ECDsa key = ECDsa.Create();
        key.ImportFromPem(publicKeyPem);
        if (!key.VerifyData(manifest, signature, HashAlgorithmName.SHA256))
            throw new CryptographicException("update manifest signature is invalid");

        using JsonDocument document = JsonDocument.Parse(manifest, new JsonDocumentOptions { MaxDepth = 16 });
        JsonElement root = document.RootElement;
        if (root.ValueKind != JsonValueKind.Object || root.GetProperty("schema_version").GetInt32() != 1)
            throw new InvalidDataException("unsupported update manifest schema");
        if (root.GetProperty("signature").GetProperty("algorithm").GetString() != "ECDSA_P256_SHA256")
            throw new InvalidDataException("unsupported update signature algorithm");

        string version = RequiredText(root, "version", 64);
        string channel = RequiredText(root, "channel", 32);
        string minimum = RequiredText(root, "minimum_launcher_version", 64);
        JsonElement files = root.GetProperty("files");
        if (files.ValueKind != JsonValueKind.Array || files.GetArrayLength() is 0 or > MaxFiles)
            throw new InvalidDataException("update file list is invalid");

        string rootPath = Path.GetFullPath(AppContext.BaseDirectory);
        string rootPrefix = rootPath.TrimEnd(Path.DirectorySeparatorChar) + Path.DirectorySeparatorChar;
        HashSet<string> paths = new(StringComparer.OrdinalIgnoreCase);
        List<UpdateFile> result = new(files.GetArrayLength());
        foreach (JsonElement file in files.EnumerateArray())
        {
            string relative = RequiredText(file, "path", 512).Replace('/', Path.DirectorySeparatorChar);
            string full = Path.GetFullPath(relative, rootPath);
            if (Path.IsPathRooted(relative) || !full.StartsWith(rootPrefix, StringComparison.OrdinalIgnoreCase) || !paths.Add(relative))
                throw new InvalidDataException($"unsafe or duplicate update path: {relative}");
            long size = file.GetProperty("size").GetInt64();
            if (size < 0) throw new InvalidDataException($"invalid update size: {relative}");
            string sha256 = RequiredText(file, "sha256", 64);
            if (!Regex.IsMatch(sha256, "^[a-fA-F0-9]{64}$")) throw new InvalidDataException($"invalid update hash: {relative}");
            Uri url = new(RequiredText(file, "url", 2048), UriKind.Absolute);
            if (url.Scheme != Uri.UriSchemeHttps) throw new InvalidDataException($"update URL must use HTTPS: {relative}");
            result.Add(new UpdateFile(relative, size, sha256, url));
        }
        return new VerifiedUpdate(version, channel, minimum, result);
    }

    private static string RequiredText(JsonElement element, string name, int maxLength)
    {
        string value = element.GetProperty(name).GetString() ?? throw new InvalidDataException($"missing {name}");
        if (string.IsNullOrWhiteSpace(value) || value.Length > maxLength) throw new InvalidDataException($"invalid {name}");
        return value;
    }
}
