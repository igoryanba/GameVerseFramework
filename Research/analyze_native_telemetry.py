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
    "telemetry_candidates_v1",
    "telemetry_callers_v1",
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
    candidates: list[dict[str, Any]] = []
    candidate_series: dict[str, list[dict[str, Any]]] = {}
    callers: list[dict[str, Any]] = []
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
        elif kind == "telemetry_candidates_v1":
            raw_candidates = message.get("candidates")
            if not isinstance(raw_candidates, list) or len(raw_candidates) > 16:
                raise ValueError(f"{path}: malformed telemetry candidates")
            for candidate in raw_candidates:
                if (
                    not isinstance(candidate, dict)
                    or not isinstance(candidate.get("candidate_id"), str)
                    or not isinstance(candidate.get("rva"), int)
                    or candidate["rva"] < 0
                    or candidate["rva"] > 0xFFFFFFFF
                    or not isinstance(candidate.get("unique_match_count"), int)
                    or not isinstance(candidate.get("call_count"), int)
                    or (
                        "entry_sha256" in candidate
                        and (
                            not isinstance(candidate["entry_sha256"], str)
                            or re.fullmatch(r"[0-9A-Fa-f]{64}", candidate["entry_sha256"])
                            is None
                        )
                    )
                ):
                    raise ValueError(f"{path}: malformed telemetry candidate")
                candidates.append(candidate)
                candidate_series.setdefault(candidate["candidate_id"], []).append(candidate)
        elif kind == "telemetry_callers_v1":
            raw_callers = message.get("callers")
            if not isinstance(raw_callers, list) or len(raw_callers) > 128:
                raise ValueError(f"{path}: malformed telemetry callers")
            for caller in raw_callers:
                if (
                    not isinstance(caller, dict)
                    or not isinstance(caller.get("candidate_id"), str)
                    or not isinstance(caller.get("caller_rva"), int)
                    or caller["caller_rva"] < 0
                    or caller["caller_rva"] > 0xFFFFFFFF
                    or not isinstance(caller.get("direct_call_sites"), int)
                    or caller["direct_call_sites"] < 1
                    or not isinstance(caller.get("entry_sha256"), str)
                    or re.fullmatch(r"[0-9A-Fa-f]{64}", caller["entry_sha256"])
                    is None
                ):
                    raise ValueError(f"{path}: malformed telemetry caller")
                callers.append(caller)

    candidate_observations = []
    for candidate_id, series in sorted(candidate_series.items()):
        identities = {
            (
                item["rva"],
                item.get("section", ""),
                item["unique_match_count"],
                item.get("entry_sha256", ""),
            )
            for item in series
        }
        counts = [item["call_count"] for item in series]
        if len(identities) != 1:
            raise ValueError(f"{path}: candidate identity changed inside trace")
        if counts != sorted(counts):
            raise ValueError(f"{path}: candidate call count decreased")
        latest_candidate = series[-1]
        candidate_observations.append(
            {
                "candidate_id": candidate_id,
                "rva": latest_candidate["rva"],
                "section": latest_candidate.get("section", ""),
                "unique_match_count": latest_candidate["unique_match_count"],
                "entry_sha256": latest_candidate.get("entry_sha256", ""),
                "initial_call_count": counts[0],
                "final_call_count": counts[-1],
                "call_delta": counts[-1] - counts[0],
                "sample_count": len(counts),
            }
        )

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
    changed_by_section: dict[str, set[int]] = {}
    transition_changed_by_section: dict[str, set[int]] = {}
    for snapshot in snapshots:
        raw_sections = snapshot.get("sections", [])
        if not isinstance(raw_sections, list):
            raise ValueError(f"{path}: malformed telemetry sections")
        for index, section in enumerate(raw_sections):
            if not isinstance(section, dict):
                raise ValueError(f"{path}: malformed telemetry section")
            identity = f"{index}:{section.get('name', '')}"
            raw_rvas = section.get("changed_page_rvas", [])
            if not isinstance(raw_rvas, list) or any(
                not isinstance(rva, int) or rva < 0 or rva > 0xFFFFFFFF
                for rva in raw_rvas
            ):
                raise ValueError(f"{path}: malformed changed page RVA")
            changed_by_section.setdefault(identity, set()).update(raw_rvas)
            if snapshot.get("stage") in {"world_transition", "adapter_loaded"}:
                transition_changed_by_section.setdefault(identity, set()).update(raw_rvas)

    sections = []
    for index, section in enumerate(latest.get("sections", [])):
        identity = f"{index}:{section.get('name', '')}"
        sections.append(
            {
                "identity": identity,
                "changed_pages": section.get("changed_pages", 0),
                "changed_page_rvas": sorted(changed_by_section.get(identity, set())),
                "transition_changed_page_rvas": sorted(
                    transition_changed_by_section.get(identity, set())
                ),
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
        "candidates": candidates,
        "candidate_observations": candidate_observations,
        "callers": callers,
    }


def build_report(paths: list[Path]) -> dict[str, Any]:
    traces = [summarize(path) for path in paths]
    fingerprints = {trace["fingerprint"] for trace in traces if trace["fingerprint"]}
    manual = [trace for trace in traces if trace["classification"] == "adapter_ready"]
    controls = [trace for trace in traces if trace["classification"] == "control_frontend"]
    page_hits: dict[tuple[str, int], list[int]] = {}
    for trace_index, trace in enumerate(traces):
        for section in trace["sections"]:
            field = (
                "transition_changed_page_rvas"
                if trace["classification"] == "adapter_ready"
                else "changed_page_rvas"
            )
            for rva in section[field]:
                page_hits.setdefault((section["identity"], rva), []).append(trace_index)
    manual_indexes = {
        index
        for index, trace in enumerate(traces)
        if trace["classification"] == "adapter_ready"
    }
    control_indexes = {
        index
        for index, trace in enumerate(traces)
        if trace["classification"] == "control_frontend"
    }
    candidate_pages = []
    for (identity, rva), hits in sorted(page_hits.items()):
        hit_indexes = set(hits)
        manual_hits = len(hit_indexes & manual_indexes)
        control_hits = len(hit_indexes & control_indexes)
        if manual and manual_hits == len(manual) and control_hits == 0:
            candidate_pages.append(
                {
                    "section": identity,
                    "rva": rva,
                    "manual_trace_hits": manual_hits,
                    "control_trace_hits": control_hits,
                }
            )

    observation_keys: set[tuple[str, int, str, str]] = set()
    for trace in traces:
        for observation in trace["candidate_observations"]:
            observation_keys.add(
                (
                    observation["candidate_id"],
                    observation["rva"],
                    observation["section"],
                    observation["entry_sha256"],
                )
            )
    candidate_observations = []
    for candidate_id, rva, section, entry_sha256 in sorted(observation_keys):
        manual_deltas = []
        control_deltas = []
        invalid_matches = 0
        for trace in traces:
            matches = [
                item
                for item in trace["candidate_observations"]
                if (
                    item["candidate_id"],
                    item["rva"],
                    item["section"],
                    item["entry_sha256"],
                )
                == (candidate_id, rva, section, entry_sha256)
            ]
            for item in matches:
                if item["unique_match_count"] != 1:
                    invalid_matches += 1
                if trace["classification"] == "adapter_ready":
                    manual_deltas.append(item["call_delta"])
                elif trace["classification"] == "control_frontend":
                    control_deltas.append(item["call_delta"])
        confirmed = (
            len(fingerprints) == 1
            and len(manual_deltas) >= 2
            and len(control_deltas) >= 1
            and all(delta > 0 for delta in manual_deltas)
            and all(delta == 0 for delta in control_deltas)
            and invalid_matches == 0
        )
        candidate_observations.append(
            {
                "candidate_id": candidate_id,
                "rva": rva,
                "section": section,
                "entry_sha256": entry_sha256,
                "manual_call_deltas": manual_deltas,
                "control_call_deltas": control_deltas,
                "invalid_match_count": invalid_matches,
                "observe_gate_satisfied": confirmed,
            }
        )
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
        "candidate_pages": candidate_pages,
        "candidate_observations": candidate_observations,
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
