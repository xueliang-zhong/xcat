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
If the input does not have a useful filename, `--syntax NAME` or `color.syntax` can force a profile such as `json`, `terraform`, `lua`, `zig`, `markdown`, or `yarn`.
Common editor and filetype aliases like `bash`, `dockerfile`, `makefile`, `markdown`, `yaml`, and `yarn` are accepted, and unknown hints fall back to filename heuristics so you do not lose color coverage by mistake.
Common Lisp-family files such as `*.el`, `*.clj`, `*.rkt`, and `*.scm` also pick up simple line-comment highlighting, and CRLF text stays eligible for highlighting when end markers are off.
Extensionless build files like `Dockerfile`, `Containerfile`, `Makefile`, `GNUmakefile`, `Procfile`, `Gemfile`, and `Justfile` also get comment-aware highlighting.
Shell rc files like `.bashrc`, `.zshrc`, and `.profile` use lightweight shell keyword highlighting, which makes them readable in terminal workflows without a full syntax engine.
Nix files like `flake.nix`, `shell.nix`, and `default.nix` get similar keyword coverage for `let`/`in` style expressions.
SQL files get a broader keyword set, and markup-oriented extensions such as `*.md`, `*.markdown`, `*.org`, `*.rst`, and `*.adoc` stay eligible for embedded tag coloring.
Common infrastructure and build files such as `CMakeLists.txt`, `build.gradle`, `build.gradle.kts`, `build.zig`, `main.tf`, `main.tfvars`, `*.jsonc`, `*.lua`, `*.zig`, and `*.json` also receive focused comment and keyword rules.
Common manifest and config files such as `Cargo.toml`, `pyproject.toml`, `go.mod`, `package.json`, `.env`, `.envrc`, `.editorconfig`, and `deno.json` additionally color keys and section headers, which makes them much easier to scan in `less`, `vim`, and `fzf`.
Dependency lockfiles such as `Cargo.lock`, `composer.lock`, `poetry.lock`, `uv.lock`, and `yarn.lock` use the same manifest-style rules, so dependency snapshots stay readable without a heavier syntax engine.
Tab rendering with `-T` stays compatible with syntax highlighting, so code remains colorized while indentation markers are shown.
The more destructive nonprinting mode `-v` still bypasses syntax highlighting to preserve the byte-safe display contract.
