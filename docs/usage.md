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
- `--syntax NAME`: force a syntax profile for files or stdin; common aliases like `bash`, `dockerfile`, `makefile`, `markdown`, `json`, `terraform`, `nix`, and `yarn` work too
- `--list-themes`: print available palette names and exit
- `-c`, `--count-lines`: print a summary count after concatenation
- unreadable inputs are reported on stderr and later files still get processed, matching GNU `cat`
- common config manifests like `Cargo.toml`, `pyproject.toml`, `go.mod`, `package.json`, `.env`, and `.editorconfig` colorize by default when color is enabled
- dependency lockfiles like `Cargo.lock`, `composer.lock`, `poetry.lock`, `uv.lock`, and `yarn.lock` colorize by default too

## Precedence

- `-b` wins over `-n`
- `-A` expands to `-vET`
- `--no-color` overrides any explicit `--color` value
- Empty arguments or `-` read from stdin
- `--list-themes` bypasses config loading so it still works even if `~/.xcat/config.toml` is broken
- `--syntax` can force a language profile for piped content that does not have a helpful filename
- an explicit `--syntax` keeps highlighting enabled for that command even if the config disables automatic syntax detection
- if a syntax hint is unknown, xcat falls back to filename heuristics instead of dropping color
- shell rc files such as `.bashrc` and `.zshrc` colorize with shell keyword heuristics by default
- Nix files such as `flake.nix` and `shell.nix` also pick up lightweight keyword coloring automatically
- Yarn lockfiles use a dedicated lockfile heuristic so leading keys and package selector headers stay readable
- `-T` can stay colorized with syntax highlighting, so you can inspect indentation without losing language-aware colors
- `-v` keeps the byte-preserving nonprinting path, which means syntax highlighting stays disabled there on purpose
