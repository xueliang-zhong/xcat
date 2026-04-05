# Usage

## Basic Examples

```bash
xcat file.txt
xcat -n src/main.rs
xcat -A README.md
xcat -b -E notes.txt
```

## Flags

- `-n`, `--number`: number every output line
- `-b`, `--number-nonblank`: number non-empty lines only
- `-E`, `--show-ends`: show `$` at line endings
- `-s`, `--squeeze-blank`: collapse repeated blank lines
- `-T`, `--show-tabs`: show tabs as `^I`
- `-v`, `--show-nonprinting`: show control bytes
- `-A`, `--show-all`: enable `-vET`
- `-e`: shorthand for `-vE`
- `-t`: shorthand for `-vT`
- `-u`: ignored, kept for GNU `cat` compatibility
- `--color auto|always|never`: control ANSI color emission
- `--theme NAME`: select a palette
- `--list-themes`: print available palette names and exit
- `-c`, `--count-lines`: print a summary count after concatenation

## Precedence

- `-b` wins over `-n`
- `-A` expands to `-vET`
- `--no-color` overrides any explicit `--color` value
- Empty arguments or `-` read from stdin
- `--list-themes` bypasses config loading so it still works even if `~/.xcat/config.toml` is broken
