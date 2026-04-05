# Terminal Workflows

`xcat` is designed to stay useful in terminal pipelines where readability matters.

## `less`

Use ANSI-aware mode when you want colors to survive paging:

```bash
xcat --color always src/main.rs | less -R
```

`less -R` keeps color escape codes while still letting `less` handle navigation, searching, and wrapping.

## `fzf`

`fzf` needs ANSI mode explicitly:

```bash
xcat --color always Cargo.toml | fzf --ansi
```

That keeps syntax and marker colors visible while you filter and jump through results.

## `vim`

For editing, prefer plain output so the buffer stays clean:

```bash
xcat --no-color docs/usage.md > /tmp/usage.md
vim /tmp/usage.md
```

`xcat` remains byte-preserving in this mode, so it is safe for files that contain control bytes, CRLF line endings, or invalid UTF-8.

## Practical Defaults

- Use `--color auto` in interactive shells
- Use `--color always` when piping into `less -R` or `fzf --ansi`
- Use `--no-color` when feeding another editor or non-ANSI tool
