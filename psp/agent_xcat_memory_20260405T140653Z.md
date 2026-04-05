# xcat Run Memory

- Timestamp: 2026-04-05T14:06:53Z
- Scope: let syntax highlighting coexist with `-T` tab rendering while preserving the byte-safe fast path and cat parity.

## Decisions

- Kept the lightweight syntax highlighter on for `-T` because tab expansion is ASCII-only and can be interleaved safely with token coloring.
- Left `-v` out of the syntax path because nonprinting rendering rewrites arbitrary bytes and breaks UTF-8 token boundaries.
- Reused the existing batched plain-span renderer instead of adding a separate syntax-only path, so the new behavior stays localized and testable.

## Failed Ideas

- Making syntax highlighting coexist with `-v` as well: that would require byte-level token recovery across transformed multibyte sequences and would risk incorrect output.
- Rewriting the whole renderer around a unified syntax engine: too much surface area for a narrow compatibility win.

## Metrics

- `cargo test --offline`: passed
- `cargo clippy --offline -- -D warnings`: passed
- New regression tests added: 2

## Reusable Lessons

- ASCII-only display transforms such as tab markers can safely share a syntax-highlighting path.
- Byte-rewriting features like `-v` should stay isolated from text tokenization unless the token boundaries are derived from bytes, not UTF-8 strings.
