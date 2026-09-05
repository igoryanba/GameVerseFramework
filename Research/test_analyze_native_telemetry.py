import json
import tempfile
import unittest
from pathlib import Path

from analyze_native_telemetry import MAX_FRAME, build_report, load_trace


FINGERPRINT = "a" * 64


def write_trace(path: Path, adapter: bool) -> None:
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
                "sections": [],
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
                    "sections": [],
                },
            }
        )
    path.write_text("".join(json.dumps(v) + "\n" for v in messages), encoding="utf-8")


class AnalyzeNativeTelemetryTests(unittest.TestCase):
    def test_candidate_gate_requires_two_manual_and_one_control(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            paths = [root / "manual-1", root / "manual-2", root / "control"]
            write_trace(paths[0], True)
            write_trace(paths[1], True)
            write_trace(paths[2], False)
            report = build_report(paths)
            self.assertTrue(report["candidate_gate_satisfied"])

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
