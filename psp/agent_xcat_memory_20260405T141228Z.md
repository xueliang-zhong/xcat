# xcat Run Memory

- Timestamp: 2026-04-05T14:12:28Z
- Scope: keep `-c` on the plain-copy fast path by counting lines from copied bytes, while preserving byte-for-byte output and existing cat parity.

## Decisions

- Let `-c` piggyback on the direct-copy path for plain files and stdin instead of forcing the line-by-line renderer.
- Count lines from the same bytes being emitted, including the final unterminated line, so the summary stays consistent with the streamed content.
- Keep the mmap path for plain files when available, then fall back to a buffered read-and-count loop without changing output semantics.
- Added a chunked-reader unit test and an integration regression that checks `-c` still emits the file bytes before the summary.

## Failed Ideas

- Leaving `-c` on the streaming renderer for all inputs: simpler, but it throws away the existing fast path for the common plain-output case.
- Counting newlines only after the copy completes by re-reading the file: correct, but redundant work and worse cache behavior.

## Metrics

- `cargo test --all --all-features`: passed
- `cargo clippy --all --all-features -- -D warnings`: passed
- New regression tests added: 2

## Reusable Lessons

- Summary-only features can often preserve a tool's hot path if they derive their result from the bytes already being copied.
- For byte-preserving terminals, the most valuable regression is often a direct output-plus-summary check, not just a count assertion.
