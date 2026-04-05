# xcat Run Memory

- Timestamp: 2026-04-05T14:01:43Z
- Scope: fix config precedence so explicit `--syntax` can re-enable highlighting, then document and test the behavior.

## Decisions

- Treated `color.syntax_highlighting` as the automatic-detection toggle instead of a hard stop for explicit CLI syntax hints.
- Kept explicit `--syntax` dependent on ANSI color being enabled, so `--no-color` still wins cleanly.
- Added a unit regression for `DisplayOptions` and an integration regression that exercises `~/.xcat/config.toml` with `syntax_highlighting = false`.
- Updated `docs/config.md`, `docs/usage.md`, and `CURRENT_MEMORY.md` so the precedence rule is discoverable.

## Failed Ideas

- Leaving explicit `--syntax` under the config gate would preserve the old behavior but violate the documented CLI-over-config precedence.
- Broadening filename heuristics again would not address the real precedence bug and would add noise instead of fixing user intent.

## Metrics

- `cargo test --offline --quiet`: passed
- `cargo clippy --offline --quiet -- -D warnings`: passed
- `cargo fmt`: passed
- New regression tests added: 2

## Reusable Lessons

- When a config option is phrased as an automatic toggle, explicit command-line hints should still be able to opt back in.
- Integration coverage should pair config precedence changes with a real `HOME`-scoped config file to prove the path the user will hit.
