# Color and Highlighting

`xcat` colors line numbers, markers, and text tokens when color is enabled.

## Modes

- `auto`: color only when stdout is a terminal
- `always`: force ANSI output
- `never`: disable all ANSI output

## Themes

Available palette names:

- `default`
- `monokai`
- `solarized`
- `github`
- `nord`
- `dracula`
- `gruvbox`
- `onedark`
- `tokyonight`
- `catppuccin`

## Highlighting

The default offline highlighter uses lightweight token rules for common languages:

- comments
- quoted strings
- numbers
- identifiers and keywords
- simple function-call emphasis

This keeps the binary working without external syntax databases while still making code easier to scan in `less`, `vim`, and `fzf`.
The same rules apply to stdin when color output is enabled.
In practice, `xcat --color always -` will colorize piped text as well as files.
