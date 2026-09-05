#!/usr/bin/env python3
"""Validate and compare bounded GameVerse native telemetry traces."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any

MAX_FRAME = 64 * 1024
KNOWN_TYPES = {
    "bootstrap_stage",
    "bootstrap_hello",
    "bootstrap_failure",
    "telemetry_hello_v1",
    "telemetry_snapshot_v1",
}
SENSITIVE = re.compile(
    r"(?i)(password|access[_-]?token|refresh[_-]?token|dpapi|0x[0-9a-f]{8,})"
)


def load_trace(path: Path) -> list[dict[str, Any]]:
    messages: list[dict[str, Any]] = []
    with path.open("rb") as stream:
        for number, raw in enumerate(stream, 1):
            raw = raw.rstrip(b"\r\n")
            if not raw or len(raw) > MAX_FRAME:
                raise ValueError(f"{path}:{number}: invalid frame length {len(raw)}")
            text = raw.decode("utf-8", errors="strict")
            if SENSITIVE.search(text):
                raise ValueError(f"{path}:{number}: sensitive or raw-address data")
            value = json.loads(text)
            if not isinstance(value, dict) or value.get("type") not in KNOWN_TYPES:
                raise ValueError(f"{path}:{number}: unknown message type")
            if value.get("schema_version") != 1:
                raise ValueError(f"{path}:{number}: incompatible schema")
            messages.append(value)
    if not messages:
        raise ValueError(f"{path}: empty trace")
    return messages


def summarize(path: Path) -> dict[str, Any]:
    messages = load_trace(path)
    stages: list[str] = []
    snapshots: list[dict[str, Any]] = []
    fingerprint = None
    failure = None
    for message in messages:
        kind = message["type"]
        if kind == "bootstrap_stage":
            stages.append(str(message.get("stage", "")))
        elif kind == "telemetry_snapshot_v1":
            snapshot = message.get("snapshot")
            if not isinstance(snapshot, dict):
                raise ValueError(f"{path}: malformed telemetry snapshot")
            stages.append(str(snapshot.get("stage", "")))
            snapshots.append(snapshot)
        elif kind in {"bootstrap_hello", "telemetry_hello_v1"}:
            current = message.get("fingerprint")
            if fingerprint is not None and current != fingerprint:
                raise ValueError(f"{path}: fingerprint changed inside trace")
            fingerprint = current
        elif kind == "bootstrap_failure":
            failure = message.get("code")

    if fingerprint is None or "frontend_stable" not in stages:
        classification = "incomplete"
    elif any(
        snapshot.get("readiness", {}).get("adapter_loaded") for snapshot in snapshots
    ) or "adapter_ready" in stages:
        classification = "adapter_ready"
    elif "world_transition" in stages:
        classification = "world_transition_partial"
    else:
        classification = "control_frontend"

    latest = snapshots[-1] if snapshots else {}
    sections = []
    for index, section in enumerate(latest.get("sections", [])):
        sections.append(
            {
                "identity": f"{index}:{section.get('name', '')}",
                "changed_pages": section.get("changed_pages", 0),
                "aggregate_sha256": section.get("aggregate_sha256", ""),
            }
        )
    readiness = latest.get("readiness", {})
    return {
        "path": str(path.resolve()),
        "classification": classification,
        "fingerprint": fingerprint,
        "stages": stages,
        "snapshot_count": len(snapshots),
        "failure": failure,
        "readiness": readiness,
        "sections": sections,
    }


def build_report(paths: list[Path]) -> dict[str, Any]:
    traces = [summarize(path) for path in paths]
    fingerprints = {trace["fingerprint"] for trace in traces if trace["fingerprint"]}
    return {
        "schema_version": 1,
        "trace_count": len(traces),
        "fingerprints_match": len(fingerprints) <= 1,
        "manual_ready_count": sum(t["classification"] == "adapter_ready" for t in traces),
        "control_count": sum(t["classification"] == "control_frontend" for t in traces),
        "candidate_gate_satisfied": (
            len(fingerprints) == 1
            and sum(t["classification"] == "adapter_ready" for t in traces) >= 2
            and sum(t["classification"] == "control_frontend" for t in traces) >= 1
        ),
        "traces": traces,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("traces", nargs="+", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    report = build_report(args.traces)
    encoded = json.dumps(report, indent=2, ensure_ascii=False) + "\n"
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(encoded, encoding="utf-8")
    else:
        print(encoded, end="")
    return 0 if report["fingerprints_match"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
