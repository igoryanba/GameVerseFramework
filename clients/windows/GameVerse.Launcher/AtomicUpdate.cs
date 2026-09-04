using System.Diagnostics;
using System.Net;
using System.Security.Cryptography;
using System.Text.Json;

internal static class AtomicUpdate
{
    private const long MaxPackageBytes = 2L * 1024 * 1024 * 1024;

    internal static async Task<byte[]> DownloadSmallAsync(HttpClient client, Uri uri, int maximumBytes, CancellationToken cancellationToken)
    {
        using HttpResponseMessage response = await client.GetAsync(uri, HttpCompletionOption.ResponseHeadersRead, cancellationToken);
        response.EnsureSuccessStatusCode();
        if (response.Content.Headers.ContentLength is long advertised && advertised > maximumBytes)
            throw new InvalidDataException("update metadata exceeds its size limit");
        await using Stream input = await response.Content.ReadAsStreamAsync(cancellationToken);
        using MemoryStream output = new(Math.Min(maximumBytes, 64 * 1024));
        byte[] buffer = new byte[16 * 1024];
        int read;
        while ((read = await input.ReadAsync(buffer, cancellationToken)) > 0)
        {
            if (output.Length + read > maximumBytes) throw new InvalidDataException("update metadata exceeds its size limit");
            output.Write(buffer, 0, read);
        }
        return output.ToArray();
    }

    internal static async Task<string> StageAsync(VerifiedUpdate update, string installRoot, HttpClient client, CancellationToken cancellationToken)
    {
        string install = NormalizeDirectory(installRoot);
        string parent = Directory.GetParent(install)?.FullName ?? throw new InvalidDataException("install directory has no parent");
        string staging = Path.Combine(parent, $".gameverse-update-{Guid.NewGuid():N}");
        Directory.CreateDirectory(staging);
        long total = 0;
        try
        {
            foreach (UpdateFile file in update.Files)
            {
                total = checked(total + file.Size);
                if (total > MaxPackageBytes) throw new InvalidDataException("update package exceeds 2 GiB");
                string destination = SafeChild(staging, file.Path);
                Directory.CreateDirectory(Path.GetDirectoryName(destination)!);
                string partial = destination + ".partial";
                using HttpResponseMessage response = await client.GetAsync(file.Url, HttpCompletionOption.ResponseHeadersRead, cancellationToken);
                if (response.StatusCode != HttpStatusCode.OK) throw new HttpRequestException($"update download failed: {(int)response.StatusCode} {file.Path}");
                if (response.Content.Headers.ContentLength is long advertised && advertised != file.Size)
                    throw new InvalidDataException($"update size header mismatch: {file.Path}");
                await using (Stream input = await response.Content.ReadAsStreamAsync(cancellationToken))
                await using (FileStream output = new(partial, FileMode.CreateNew, FileAccess.Write, FileShare.None, 1024 * 1024, FileOptions.Asynchronous | FileOptions.SequentialScan))
                {
                    byte[] buffer = new byte[1024 * 1024];
                    long received = 0;
                    int read;
                    while ((read = await input.ReadAsync(buffer, cancellationToken)) > 0)
                    {
                        received = checked(received + read);
                        if (received > file.Size) throw new InvalidDataException($"update file is larger than declared: {file.Path}");
                        await output.WriteAsync(buffer.AsMemory(0, read), cancellationToken);
                    }
                    if (received != file.Size) throw new InvalidDataException($"update file is truncated: {file.Path}");
                }
                await using FileStream hashInput = File.OpenRead(partial);
                string hash = Convert.ToHexString(await SHA256.HashDataAsync(hashInput, cancellationToken));
                if (!hash.Equals(file.Sha256, StringComparison.OrdinalIgnoreCase)) throw new InvalidDataException($"update hash mismatch: {file.Path}");
                File.Move(partial, destination);
            }
            if (!VerifyInstallTree(staging, out string detail)) throw new InvalidDataException(detail);
            return staging;
        }
        catch
        {
            TryDeleteDirectory(staging);
            throw;
        }
    }

    internal static void PreserveLocalFiles(string installRoot, string stagingRoot, params string?[] relativePaths)
    {
        foreach (string? candidate in relativePaths)
        {
            if (string.IsNullOrWhiteSpace(candidate)) continue;
            string relative = candidate;
            string source = SafeChild(installRoot, relative);
            string destination = SafeChild(stagingRoot, relative);
            if (!File.Exists(source) || File.Exists(destination)) continue;
            Directory.CreateDirectory(Path.GetDirectoryName(destination)!);
            File.Copy(source, destination);
        }
    }

    internal static bool Apply(string installRoot, string stagingRoot, string backupRoot, out string detail)
    {
        string install = NormalizeDirectory(installRoot);
        string staging = NormalizeDirectory(stagingRoot);
        string backup = NormalizeDirectory(backupRoot);
        string parent = Directory.GetParent(install)?.FullName ?? "";
        if (!Directory.Exists(install) || !Directory.Exists(staging)) throw new DirectoryNotFoundException("install or staging directory is missing");
        if (!StringComparer.OrdinalIgnoreCase.Equals(Directory.GetParent(staging)?.FullName, parent)
            || !Path.GetFileName(staging).StartsWith(".gameverse-update-", StringComparison.OrdinalIgnoreCase)
            || !StringComparer.OrdinalIgnoreCase.Equals(backup, install + ".previous"))
            throw new InvalidDataException("update directories are outside the permitted transaction layout");
        if (!VerifyInstallTree(staging, out detail)) return false;

        string staleBackup = backup + ".stale-" + Guid.NewGuid().ToString("N");
        bool oldMoved = false;
        try
        {
            if (Directory.Exists(backup)) Directory.Move(backup, staleBackup);
            Directory.Move(install, backup);
            oldMoved = true;
            Directory.Move(staging, install);
            if (!VerifyInstallTree(install, out detail)) throw new InvalidDataException(detail);
            TryDeleteDirectory(staleBackup);
            detail = "update installed; previous version retained";
            return true;
        }
        catch (Exception error)
        {
            if (oldMoved)
            {
                if (Directory.Exists(install)) Directory.Move(install, install + ".failed-" + Guid.NewGuid().ToString("N"));
                if (Directory.Exists(backup)) Directory.Move(backup, install);
            }
            if (Directory.Exists(staleBackup) && !Directory.Exists(backup)) Directory.Move(staleBackup, backup);
            detail = "update rolled back: " + error.Message;
            return false;
        }
    }

    internal static bool VerifyInstallTree(string root, out string detail)
    {
        try
        {
            string manifestPath = Path.Combine(root, "install-manifest.json");
            using JsonDocument document = JsonDocument.Parse(File.ReadAllText(manifestPath), new JsonDocumentOptions { MaxDepth = 16 });
            if (document.RootElement.GetProperty("schema_version").GetInt32() != 1) throw new InvalidDataException("unsupported install manifest schema");
            int count = 0;
            foreach (JsonElement file in document.RootElement.GetProperty("files").EnumerateArray())
            {
                string relative = file.GetProperty("path").GetString() ?? throw new InvalidDataException("missing install path");
                string full = SafeChild(root, relative);
                if (!File.Exists(full)) throw new InvalidDataException($"installed file is missing: {relative}");
                long size = file.GetProperty("size").GetInt64();
                string expected = file.GetProperty("sha256").GetString() ?? "";
                using FileStream hashInput = File.OpenRead(full);
                string actual = Convert.ToHexString(SHA256.HashData(hashInput));
                if (new FileInfo(full).Length != size || !actual.Equals(expected, StringComparison.OrdinalIgnoreCase))
                    throw new InvalidDataException($"installed file failed verification: {relative}");
                count++;
            }
            detail = $"{count} installed files verified";
            return count > 0;
        }
        catch (Exception error) when (error is IOException or JsonException or InvalidDataException or KeyNotFoundException)
        {
            detail = error.Message;
            return false;
        }
    }

    internal static async Task WaitForExitAsync(int processId, TimeSpan timeout)
    {
        try
        {
            using Process process = Process.GetProcessById(processId);
            using CancellationTokenSource deadline = new(timeout);
            await process.WaitForExitAsync(deadline.Token);
        }
        catch (ArgumentException) { }
    }

    private static string SafeChild(string root, string relative)
    {
        if (Path.IsPathRooted(relative)) throw new InvalidDataException($"absolute update path is forbidden: {relative}");
        string normalizedRoot = NormalizeDirectory(root);
        string prefix = normalizedRoot + Path.DirectorySeparatorChar;
        string full = Path.GetFullPath(relative.Replace('/', Path.DirectorySeparatorChar), normalizedRoot);
        if (!full.StartsWith(prefix, StringComparison.OrdinalIgnoreCase)) throw new InvalidDataException($"update path escapes package: {relative}");
        return full;
    }

    private static string NormalizeDirectory(string path) => Path.GetFullPath(path).TrimEnd(Path.DirectorySeparatorChar, Path.AltDirectorySeparatorChar);

    private static void TryDeleteDirectory(string path)
    {
        try { if (Directory.Exists(path)) Directory.Delete(path, true); }
        catch (IOException) { }
        catch (UnauthorizedAccessException) { }
    }
}
