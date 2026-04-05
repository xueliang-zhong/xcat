# xcat Run Memory

- Timestamp: 2026-04-05T13:35:41Z
- Scope: make `--list-themes` independent of config parsing, add a malformed-config regression, and refresh the docs/memory trail.

## Decisions

- Moved the `--list-themes` fast path ahead of config loading in `run()`.
- Added an integration regression that seeds a broken `~/.xcat/config.toml` and verifies theme listing still succeeds.
- Documented the config-bypass behavior in `README.md`, `docs/usage.md`, and `docs/config.md`.
- Recorded the lesson in `CURRENT_MEMORY.md` so future runs keep informational commands decoupled from config errors.

## Failed Ideas

- Loading config before checking `--list-themes` needlessly couples an informational command to file parsing and can fail for the wrong reason.

## Metrics

- `cargo fmt --check`: passed
- `cargo test --offline --quiet`: passed
- `cargo test --offline --quiet --features syntax-highlight`: passed
- New regression tests added: 1

## Reusable Lessons

- If a command only needs static metadata, bypass config parsing so broken user config does not block it.
- Malformed-config regressions are useful for commands that should stay available even when regular execution would fail.
