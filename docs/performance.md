# Performance

`xcat` prefers a fast copy path whenever it can preserve output exactly.

## Fast Path

- Plain files without transformations use buffered copy or `mmap`
- Stdin without transformations uses direct streaming copy
- `-c` keeps the plain-copy path when no rendering flags are active and counts lines from the emitted bytes
- `mmap` is configurable in `performance.use_mmap`

## Streaming Path

The streaming path is used when any of these are active:

- line numbering
- blank-line squeezing
- end markers
- tab/nonprinting rendering
- color highlighting

When `performance.use_mmap` is enabled, regular files can use `mmap` for both
plain copying and rendered output. That keeps the render logic byte-oriented
without forcing an extra buffered read loop for large files.

When ANSI output is enabled, `xcat` writes styled prefixes/suffixes directly
to stdout and keeps plain byte spans batched. That avoids building whole
highlighted lines in temporary `String`s while preserving byte-for-byte output.

## Practical Effects

- Small text files stay interactive and colorful
- Large files still avoid loading the whole file into memory
- Binary data is preserved byte-for-byte unless a display flag explicitly transforms it
