# Byte-Safe Stream Processing

Use this pattern for `cat`-like tools and any byte-preserving terminal copier:

- Read with `BufRead::read_until(b'\n', ...)` or raw `Read`, not `String::lines()`
- Treat newline as a byte suffix, not a text delimiter
- Keep rendering logic separate from copying logic
- Preserve a direct copy path for plain files and stdin
- Only decode UTF-8 when a feature explicitly needs text analysis
- Add tests for invalid UTF-8, CRLF, control bytes, and EOF without newline
- If stdin gets a special fast path, gate it with the same rendering conditions as files so color/highlighting behavior stays aligned
- End markers like `-E`/`-A` belong only on lines that ended with `\n`; do not synthesize them for a final unterminated line.
- When ANSI styling is needed, keep plain byte spans batched and write prefixes/suffixes directly instead of materializing whole rendered lines.
- For lightweight syntax highlighting, extensionless filename heuristics like `Dockerfile` and `Makefile` can expand coverage cheaply without changing the streaming/copy core.
