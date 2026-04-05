# xcat

`xcat` is a byte-preserving Rust `cat` replacement with Linux-style display flags, configurable color modes, fast plain-file streaming, and lightweight language-aware highlighting for terminal use.

## Quick Start

```bash
cargo build --release
cargo run -- file.txt
```

## Highlights

- `cat`-style flags: `-n`, `-b`, `-E`, `-s`, `-T`, `-v`, `-A`
- GNU compatibility flags: `-e`, `-t`, `-u`
- TTY-aware color output with `--color auto|always|never`
- Theme selection with `--theme`
- `--list-themes` to print the built-in palette names
- Config file support at `~/.xcat/config.toml`
- Fast streaming path for plain files and stdin

## Docs

- [Configuration](docs/config.md)
- [Usage](docs/usage.md)
- [Color and highlighting](docs/color.md)
- [Terminal workflows](docs/workflows.md)
- [Performance](docs/performance.md)

## Configuration

Create `~/.xcat/config.toml` to set defaults. Use [`config.example.toml`](config.example.toml) as a starting point.
