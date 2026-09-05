#!/usr/bin/env python3
"""Fetch pinned FiveM resources for static analysis without executing them."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import pathlib
import re
import shutil
import stat
import tempfile
import urllib.request
import zipfile

MAX_ARCHIVE = 256 * 1024 * 1024
MAX_EXPANDED = 2 * 1024 * 1024 * 1024
MAX_ENTRIES = 100_000


def archive_url(repository: str, commit: str) -> str:
    match = re.fullmatch(r"https://github\.com/([^/]+)/([^/]+?)(?:\.git)?", repository)
    if not match:
        raise ValueError("only canonical GitHub repository URLs are accepted")
    return f"https://codeload.github.com/{match.group(1)}/{match.group(2)}/zip/{commit}"


def safe_extract(archive: pathlib.Path, destination: pathlib.Path) -> None:
    with zipfile.ZipFile(archive) as source:
        entries = source.infolist()
        if len(entries) > MAX_ENTRIES:
            raise ValueError("archive has too many entries")
        if sum(entry.file_size for entry in entries) > MAX_EXPANDED:
            raise ValueError("archive expands beyond the corpus limit")
        root = destination.resolve()
        validated: list[tuple[zipfile.ZipInfo, pathlib.Path]] = []
        for entry in entries:
            mode = entry.external_attr >> 16
            if stat.S_ISLNK(mode):
                raise ValueError("archive symlinks are forbidden")
            target = (destination / entry.filename).resolve()
            if target != root and root not in target.parents:
                raise ValueError("archive path escapes the corpus directory")
            validated.append((entry, target))
        for entry, target in validated:
            native = f"\\\\?\\{target}" if os.name == "nt" else str(target)
            if entry.is_dir():
                os.makedirs(native, exist_ok=True)
                continue
            parent = f"\\\\?\\{target.parent}" if os.name == "nt" else str(target.parent)
            os.makedirs(parent, exist_ok=True)
            with source.open(entry) as input_file, open(native, "wb") as output_file:
                shutil.copyfileobj(input_file, output_file, 1024 * 1024)


def fetch(resource: dict[str, str], destination: pathlib.Path) -> dict[str, str]:
    name = resource["name"]
    commit = resource["commit"]
    expected = resource["archive_sha256"].upper()
    target = destination / name
    if target.exists():
        shutil.rmtree(target)
    target.mkdir(parents=True)
    with tempfile.NamedTemporaryFile(delete=False, suffix=".zip") as temporary:
        archive = pathlib.Path(temporary.name)
    try:
        request = urllib.request.Request(
            archive_url(resource["url"], commit),
            headers={"User-Agent": "GameVerse-static-corpus/1"},
        )
        digest = hashlib.sha256()
        size = 0
        with urllib.request.urlopen(request, timeout=60) as response, archive.open("wb") as output:
            while chunk := response.read(1024 * 1024):
                size += len(chunk)
                if size > MAX_ARCHIVE:
                    raise ValueError("archive exceeds the download limit")
                digest.update(chunk)
                output.write(chunk)
        actual = digest.hexdigest().upper()
        if actual != expected:
            raise ValueError(f"archive hash mismatch for {name}: {actual}")
        safe_extract(archive, target)
        roots = [path for path in target.iterdir() if path.is_dir()]
        if len(roots) != 1:
            raise ValueError("archive must contain exactly one repository root")
        return {
            "name": name,
            "commit": commit,
            "license": resource["license"],
            "mode": resource["mode"],
            "archive_sha256": actual,
            "path": str(roots[0]),
        }
    finally:
        archive.unlink(missing_ok=True)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--lock", default="Research/resource-corpus.lock.json")
    parser.add_argument("--output", default=".research/resources")
    parser.add_argument("--mode", action="append", default=["executable_canary"])
    parser.add_argument("--receipt", default=".research/resources/receipt.json")
    arguments = parser.parse_args()
    lock = json.loads(pathlib.Path(arguments.lock).read_text(encoding="utf-8"))
    selected = [item for item in lock["resources"] if item["mode"] in arguments.mode]
    destination = pathlib.Path(arguments.output)
    destination.mkdir(parents=True, exist_ok=True)
    receipts = [fetch(item, destination) for item in selected]
    receipt = pathlib.Path(arguments.receipt)
    receipt.parent.mkdir(parents=True, exist_ok=True)
    receipt.write_text(
        json.dumps({"schema_version": 1, "resources": receipts}, indent=2) + "\n",
        encoding="utf-8",
    )
    print(json.dumps({"status": "verified", "resources": len(receipts)}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
