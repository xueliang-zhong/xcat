# Current Memory

- Keep the core `cat` pipeline byte-oriented; `String`/`lines()` shortcuts break binary parity and CRLF behavior.
- Prefer offline-safe defaults for this repo; feature-gated syntax engines are fine, but the default build must not require uncached crates.
- If a feature build is expected to work offline, it must not pull uncached transitive crates even when the feature is enabled.
- For plain files and stdin, preserve a fast copy path and only drop into per-line rendering when a display flag actually needs it.
- Direct-write ANSI helpers beat heap-allocated marker strings in the hot path, and CR-before-newline needs a special `^M` case to match GNU `cat -E`.
- Span-based plain writes beat per-byte writes in the renderer hot path, but the same byte-safe CRLF and tab rules still need to hold.
- Use system `cat` as a local oracle for tricky flag combinations like `-v`, `-E`, and mixed file/stdin concatenation.
- Keep stdin on the same rendering decision path as files; otherwise color-enabled pipes can silently bypass highlighting.
- For config-path regressions, set `HOME` on the child command so `~/.xcat/config.toml` is exercised without parallel-test interference.
- A `BufWriter` around stdout is a low-risk throughput win for the rendering path in byte-preserving terminal tools.
- `-E`/`-A` end markers must only print on newline-terminated lines; EOF without a trailing newline should stay marker-free to match GNU `cat`.
- Direct ANSI writes beat assembling full highlighted lines in `String`s; keep plain spans batched and emit styled prefixes/suffixes only around the transformed tokens.
- Commands that do not need user config, such as `--list-themes`, should bypass config loading so malformed `~/.xcat/config.toml` files do not block purely informational flows.
- Let `clippy` collapse boilerplate `Default` impls on config enums and zero-value structs; it keeps serde defaults aligned and trims maintenance noise without changing behavior.
