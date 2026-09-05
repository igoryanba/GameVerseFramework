import json
import tempfile
import unittest
from pathlib import Path

from analyze_native_telemetry import MAX_FRAME, build_report, load_trace, summarize


FINGERPRINT = "a" * 64


def write_trace(path: Path, adapter: bool, changed_rvas: list[int] | None = None) -> None:
    messages = [
        {
            "type": "bootstrap_hello",
            "schema_version": 1,
            "fingerprint": FINGERPRINT,
        },
        {
            "type": "telemetry_snapshot_v1",
            "schema_version": 1,
            "snapshot": {
                "stage": "frontend_stable",
                "readiness": {"adapter_loaded": False},
                "sections": [
                    {
                        "name": ".text",
                        "changed_pages": len(changed_rvas or []) if not adapter else 0,
                        "changed_page_rvas": (changed_rvas or []) if not adapter else [],
                        "aggregate_sha256": "a" * 64,
                    }
                ],
            },
        },
    ]
    if adapter:
        messages.append(
            {
                "type": "telemetry_snapshot_v1",
                "schema_version": 1,
                "snapshot": {
                    "stage": "adapter_loaded",
                    "readiness": {"adapter_loaded": True},
                    "sections": [
                        {
                            "name": ".text",
                            "changed_pages": len(changed_rvas or []),
                            "changed_page_rvas": changed_rvas or [],
                            "aggregate_sha256": "b" * 64,
                        }
                    ],
                },
            }
        )
    path.write_text("".join(json.dumps(v) + "\n" for v in messages), encoding="utf-8")


class AnalyzeNativeTelemetryTests(unittest.TestCase):
    def test_candidate_message_is_bounded_and_preserved(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "candidate"
            write_trace(path, False)
            with path.open("a", encoding="utf-8") as stream:
                stream.write(
                    json.dumps(
                        {
                            "type": "telemetry_candidates_v1",
                            "schema_version": 1,
                            "candidates": [
                                {
                                    "candidate_id": "transition_ref_a",
                                    "rva": 4096,
                                    "section": ".text",
                                    "unique_match_count": 1,
                                    "call_count": 0,
                                }
                            ],
                        }
                    )
                    + "\n"
                )
            self.assertEqual(summarize(path)["candidates"][0]["rva"], 4096)

    def test_candidate_gate_requires_two_manual_and_one_control(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            paths = [root / "manual-1", root / "manual-2", root / "control"]
            write_trace(paths[0], True, [4096, 8192])
            write_trace(paths[1], True, [4096, 12288])
            write_trace(paths[2], False)
            report = build_report(paths)
            self.assertTrue(report["candidate_gate_satisfied"])
            self.assertEqual(
                report["candidate_pages"],
                [
                    {
                        "section": "0:.text",
                        "rva": 4096,
                        "manual_trace_hits": 2,
                        "control_trace_hits": 0,
                    }
                ],
            )

    def test_control_trace_excludes_page_and_bad_rva_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            paths = [root / "manual-1", root / "manual-2", root / "control"]
            write_trace(paths[0], True, [4096])
            write_trace(paths[1], True, [4096])
            write_trace(paths[2], False, [4096])
            self.assertEqual(build_report(paths)["candidate_pages"], [])

            malformed = root / "malformed"
            write_trace(malformed, True, [-1])
            with self.assertRaisesRegex(ValueError, "malformed changed page RVA"):
                build_report([malformed])

    def test_rejects_sensitive_data_and_oversized_frames(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "bad"
            path.write_text(
                json.dumps(
                    {
                        "type": "bootstrap_failure",
                        "schema_version": 1,
                        "message": "refresh_token=secret",
                    }
                )
                + "\n",
                encoding="utf-8",
            )
            with self.assertRaisesRegex(ValueError, "sensitive"):
                load_trace(path)
            path.write_bytes(b"x" * (MAX_FRAME + 1) + b"\n")
            with self.assertRaisesRegex(ValueError, "frame length"):
                load_trace(path)


if __name__ == "__main__":
    unittest.main()
