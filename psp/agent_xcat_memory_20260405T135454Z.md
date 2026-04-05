# xcat Run Memory

- Timestamp: 2026-04-05T13:54:54Z
- Scope: add explicit syntax-hint support for stdin and mismatched filenames, broaden lightweight heuristics for common build/config files, and refresh docs/tests.

## Decisions

- Added `--syntax` and `color.syntax` so users can force a language profile when stdin or a generic filename would otherwise hide the intended syntax.
- Kept the byte-safe fast path intact by letting the new syntax hint feed the existing streaming renderer instead of introducing a separate decode path.
- Expanded filename heuristics for common project files such as `CMakeLists.txt`, Gradle, Terraform, Lua, Zig, JSON, and JSONC.
- Kept unknown syntax hints non-fatal by falling back to filename heuristics rather than disabling coloring.

## Failed Ideas

- Relying on filename heuristics alone left pipe-heavy workflows without a good way to recover language-aware colors.
- Leaving the new syntax hint unthreaded through test fixtures caused compile-time misses that the validation pass caught immediately.
- Treating the syntax hint as authoritative even when it was unrecognized would have made the feature brittle.

## Metrics

- `cargo test --offline --quiet`: passed, 28 unit/integration tests in the default build
- `cargo test --offline --quiet --features syntax-highlight`: passed, 30 tests
- `cargo clippy --offline --quiet -- -D warnings`: passed
- `cargo fmt --check`: passed

## Reusable Lessons

- A small explicit syntax override is often more valuable than more filename guesses when stdin is involved.
- Fallback from explicit hints to heuristics preserves coverage and keeps the feature tolerant of user typos.
- When a shared struct gains a new field, every hand-written test fixture needs to be updated immediately or the compiler will catch it later.
