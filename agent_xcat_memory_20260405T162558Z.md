# xcat Run Memory

- Timestamp: 2026-04-05T16:25:58Z
- Scope: speed up rendered file handling with `mmap`, prune dead runtime deps, and keep the docs/memory trail aligned.
- Commit: `779ca05f4f29f0d1e0fd7cb7cdb9b6d63d2e43d1`

## Decisions

- Introduced a shared line-rendering context so both buffered readers and `mmap` slices use the same byte-oriented render logic.
- Let regular files use `mmap` for rendered output as well as plain copies when `performance.use_mmap` is enabled.
- Removed unused runtime dependencies from `Cargo.toml` to shrink the normal build graph and reduce audit surface.
- Added a helper-level regression that exercises the shared byte-slice render path with numbering, tabs, end markers, and syntax highlighting.
- Updated README and performance docs so the new mmap-backed render path is visible to users.

## Failed Ideas

- Keeping the render path split between a buffered-reader implementation and a separate mmap-only branch: too much duplication and easier to drift out of parity.
- Trying to justify the unused runtime dependencies as future-proofing: they were not referenced by the codebase and only added noise to the build graph.

## Metrics

- `cargo test --offline --quiet`: passed
- `cargo test --offline --quiet --features syntax-highlight`: passed
- `cargo fmt --all --check`: passed
- `cargo clippy --offline --quiet -- -D warnings`: passed
- New regression tests added: 1

## Reusable Lessons

- A tiny renderer context object is often a cleaner way to share byte-oriented rendering logic than growing helper argument lists.
- If a file path already requires line-oriented rendering, `mmap` can still pay off without compromising byte parity when the line renderer is shared.
