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
