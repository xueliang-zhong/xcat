# Failed Paths

- String-based line processing with `read_line`/`lines()`: loses binary parity, invalid UTF-8, and CRLF fidelity.
- Making `syntect` a default dependency in this offline environment: pulls uncached crates and blocks builds.
- Keeping a syntect-gated syntax-highlight build path: the offline feature build still tried to fetch uncached transitive crates.
- Heap-allocating every rendered control-byte token in the hot path: unnecessary churn compared with direct writes and stack buffers.
- Writing each rendered byte individually in the hot path: too many write calls versus batching plain spans and emitting only transformed tokens.
- Unconditional stdin raw-copy fast path: bypasses color highlighting for piped text when ANSI output is enabled.
- Globally excluding any CRLF body from syntax highlighting: too conservative; it blocks safe color coverage when end markers are off.
