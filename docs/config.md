# Configuration

`xcat` reads `~/.xcat/config.toml` by default.
The `--list-themes` command skips config loading so you can inspect palette names
even if the file is missing or malformed.

## Precedence

Settings resolve in this order:

1. command-line flags
2. `~/.xcat/config.toml`
3. built-in defaults

That lets you keep a stable personal profile while still overriding behavior per command.

## Example

```toml
[display]
number = false
number_nonblank = false
show_ends = false
squeeze_blank = false
show_tabs = false
show_nonprinting = false

[color]
mode = "auto"
theme = "default"
syntax_highlighting = true

[performance]
use_mmap = true
buffer_size = 65536
```

## Fields

- `display.number`: default for `-n`
- `display.number_nonblank`: default for `-b`
- `display.show_ends`: default for `-E`
- `display.squeeze_blank`: default for `-s`
- `display.show_tabs`: default for `-T`
- `display.show_nonprinting`: default for `-v`
- `color.mode`: `auto`, `always`, or `never`
- `color.theme`: palette name used for line numbers and markers
- `color.syntax_highlighting`: enables the internal language-aware highlighter
- `performance.use_mmap`: enables file mmap fast paths
- `performance.buffer_size`: buffer size used for streaming reads

## Notes

- `auto` follows terminal detection
- `always` is useful for `less -R`, `fzf --ansi`, and similar pipelines
- `never` forces plain output
- `--no-color` always wins over any configured color mode
