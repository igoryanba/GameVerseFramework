import pathlib
import stat
import tempfile
import unittest
import zipfile

from fetch_corpus import archive_url, safe_extract


class CorpusFetchTests(unittest.TestCase):
    def make_archive(self, entries: list[tuple[zipfile.ZipInfo, bytes]]) -> pathlib.Path:
        temporary = tempfile.NamedTemporaryFile(delete=False, suffix=".zip")
        temporary.close()
        path = pathlib.Path(temporary.name)
        with zipfile.ZipFile(path, "w") as archive:
            for info, body in entries:
                archive.writestr(info, body)
        self.addCleanup(path.unlink, missing_ok=True)
        return path

    def test_accepts_only_canonical_github_urls(self) -> None:
        self.assertEqual(
            archive_url("https://github.com/owner/repo.git", "a" * 40),
            f"https://codeload.github.com/owner/repo/zip/{'a' * 40}",
        )
        with self.assertRaises(ValueError):
            archive_url("https://example.invalid/owner/repo", "a" * 40)

    def test_rejects_traversal_and_symlink_entries(self) -> None:
        traversal = self.make_archive([(zipfile.ZipInfo("../escape.lua"), b"bad")])
        symlink = zipfile.ZipInfo("resource/link")
        symlink.create_system = 3
        symlink.external_attr = (stat.S_IFLNK | 0o777) << 16
        link_archive = self.make_archive([(symlink, b"../outside")])
        with tempfile.TemporaryDirectory() as output:
            with self.assertRaises(ValueError):
                safe_extract(traversal, pathlib.Path(output))
            with self.assertRaises(ValueError):
                safe_extract(link_archive, pathlib.Path(output))

    def test_extracts_regular_files(self) -> None:
        archive = self.make_archive([(zipfile.ZipInfo("resource/fxmanifest.lua"), b"game 'gta5'")])
        with tempfile.TemporaryDirectory() as output:
            root = pathlib.Path(output)
            safe_extract(archive, root)
            self.assertEqual((root / "resource/fxmanifest.lua").read_bytes(), b"game 'gta5'")


if __name__ == "__main__":
    unittest.main()
