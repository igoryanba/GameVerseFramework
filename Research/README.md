# External resource corpus

`resource-corpus.lock.json` records exact research revisions and archive hashes.
Source trees belong in the ignored `.research/resources` directory and are not
part of GameVerse. Only entries marked `executable_canary` may be considered for
execution, and only after the analyzer reports no blocked capabilities. Unknown,
LGPL, GPL, and mixed-license entries are inventory inputs unless a separate
distribution and linking review approves a different use.

`python Research/fetch_corpus.py` downloads only pinned `executable_canary`
archives by default. It verifies SHA-256 before extraction, rejects traversal
and symlink entries, applies bounded archive limits, and writes an ignored
receipt. The script never executes resource code.

## Native world-loader research

Native traces are written only to the ignored `.research/telemetry` directory.
The probe reads the verified main image and emits bounded RVAs, counts and
hashes; it does not emit raw values or absolute addresses and does not install
hooks in `telemetry_only` mode.

The first pair of manual traces appeared to identify `.data` RVA `43718912`.
A later frontend-only control trace reproduced the same scalar sequence, so the
candidate is rejected as startup/frontend initialization. A runtime Zydis scan
also found no direct RIP-relative writer for that RVA. It must not be promoted
to an observe-only or world-loader manifest.

`run_native_telemetry.ps1` now delays the transition marker for ten seconds
after `frontend_ready`. Candidate selection requires two new manual Story
traces with the same sequence and a delayed control trace in which the RVA is
absent. Runtime writer inspection is requested only after this gate passes.
