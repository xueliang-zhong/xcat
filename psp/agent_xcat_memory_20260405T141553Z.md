# xcat Run Memory

- Timestamp: 2026-04-05T14:15:53Z
- Scope: make multi-source error handling behave more like GNU `cat` by reporting unreadable inputs, continuing with later files, and aligning stderr formatting.

## Decisions

- Kept source-open/read failures nonfatal so later files still concatenate after an error.
- Flushed buffered stdout before printing a nonfatal source error to preserve output ordering.
- Treated stdout write failures as fatal, since the command cannot continue without a working sink.
- Trimmed Rust's `(os error N)` suffix from I/O error messages so unreadable-path output is closer to GNU `cat`.
- Added regression tests for missing-file-before-valid-file, valid-file-before-missing-file, and `-c` with a later valid source.

## Failed Ideas

- Returning immediately on the first source error: simpler, but it breaks GNU `cat`'s continue-after-error behavior.
- Comparing stderr against system `cat` without normalization: the program prefix and Rust's OS-error suffix made the test brittle.

## Metrics

- `cargo test --offline --quiet`: passed
- `cargo clippy --offline --quiet -- -D warnings`: passed
- `cargo fmt`: passed
- New regression tests added: 3

## Reusable Lessons

- For multi-source stream tools, source errors should usually be reported and skipped, while sink errors stay fatal.
- If you buffer stdout, flush it before reporting a later source error so captured output stays ordered like the terminal output a user expects.
