# xcat Run Memory

- Timestamp: 2026-04-05T13:31:17Z
- Scope: remove the remaining `String` assembly from the syntax-highlighting render path, keep ANSI writes streaming, and record the optimization in repo memory/docs.

## Decisions

- Rewrote the syntax-highlighting path to write plain spans directly to stdout and emit ANSI prefixes/suffixes only around transformed tokens.
- Kept the byte-oriented line parser and the fast plain-copy path unchanged.
- Added a file-based color-highlight regression so the path-based syntax heuristics stay covered, not just stdin.
- Documented the direct-write renderer in `docs/performance.md` and captured the lesson in `CURRENT_MEMORY.md` and `skill_byte_safe_streams.md`.

## Failed Ideas

- Building full highlighted lines in temporary `String`s left avoidable allocation overhead on the hot path.
- Leaving the streaming syntax path covered only by stdin tests would have missed file-extension heuristics.

## Metrics

- `cargo fmt --check`: passed
- `cargo test --offline --quiet`: passed
- `cargo test --offline --quiet --features syntax-highlight`: passed
- New regression tests added: 1

## Reusable Lessons

- For ANSI-heavy terminal tools, write style prefixes/suffixes directly and keep plain byte spans batched.
- Path-based highlighting needs at least one file regression in addition to stdin coverage.
- Performance docs and memory notes should capture successful hot-path allocation removals so later runs do not reintroduce them.
