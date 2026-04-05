use std::fs::File;
use std::io::{self, BufRead, BufReader, IsTerminal, Write};
use std::path::Path;

use crate::cli::Cli;
use crate::colorizer::Colorizer;
use crate::config::Config;
use crate::display::DisplayOptions;
use crate::error::{XcatError, XcatResult};
use crate::reader::{
    is_blank_line, strip_trailing_newline, write_end_marker, write_line_number, write_rendered_body,
};

pub fn run() -> i32 {
    let cli = Cli::parse_args();

    if cli.list_themes {
        for theme in Colorizer::available_themes() {
            println!("{theme}");
        }
        return 0;
    }

    let config = match Config::load() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            return 1;
        }
    };

    let stdout_is_terminal = io::stdout().is_terminal();
    let opts = DisplayOptions::from_cli_and_config(&cli, &config, stdout_is_terminal);

    let colorizer = Colorizer::new(opts.color_enabled, &opts.theme_name);
    match execute(&cli, &opts, &colorizer) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{err}");
            1
        }
    }
}

pub fn execute(cli: &Cli, opts: &DisplayOptions, colorizer: &Colorizer) -> XcatResult<usize> {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut state = StreamState::default();
    let mut total_lines = 0usize;

    let sources: Vec<&str> = if cli.files.is_empty() {
        vec!["-"]
    } else {
        cli.files.iter().map(|s| s.as_str()).collect()
    };

    for source in sources {
        total_lines += process_source(source, opts, colorizer, &mut state, &mut out)?;
    }

    if opts.count_lines {
        writeln!(out, "Total lines: {total_lines}")
            .map_err(|e| XcatError::Io(e, String::from("stdout")))?;
    }

    out.flush()
        .map_err(|e| XcatError::Io(e, String::from("stdout")))?;
    Ok(total_lines)
}

#[derive(Debug, Default)]
struct StreamState {
    line_number: usize,
    blank_run: usize,
}

fn process_source<W: Write>(
    source: &str,
    opts: &DisplayOptions,
    colorizer: &Colorizer,
    state: &mut StreamState,
    out: &mut W,
) -> XcatResult<usize> {
    if source == "-" {
        let mut syntax = syntax_session_for_path(Path::new(source), opts);
        if can_fast_copy_plain(opts, syntax.is_some()) {
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            io::copy(&mut handle, out).map_err(|e| XcatError::Io(e, String::from("stdin")))?;
            return Ok(0);
        }

        let stdin = io::stdin();
        let mut handle = stdin.lock();
        return process_reader_with_syntax(
            &mut handle,
            source,
            opts,
            colorizer,
            state,
            out,
            syntax.as_mut(),
        );
    }

    let path = Path::new(source);
    let file = File::open(path).map_err(|e| XcatError::Io(e, source.to_string()))?;

    let mut syntax = syntax_session_for_path(path, opts);
    if can_fast_copy_plain(opts, syntax.is_some()) {
        return copy_fast(file, source, opts.use_mmap, out);
    }

    let mut reader = BufReader::with_capacity(opts.buffer_size.max(1), file);
    process_reader_with_syntax(
        &mut reader,
        source,
        opts,
        colorizer,
        state,
        out,
        syntax.as_mut(),
    )
}

#[cfg(test)]
fn process_reader<R: BufRead, W: Write>(
    reader: &mut R,
    source: &str,
    opts: &DisplayOptions,
    colorizer: &Colorizer,
    state: &mut StreamState,
    out: &mut W,
) -> XcatResult<usize> {
    process_reader_with_syntax(reader, source, opts, colorizer, state, out, None)
}

fn process_reader_with_syntax<R: BufRead, W: Write>(
    reader: &mut R,
    source: &str,
    opts: &DisplayOptions,
    colorizer: &Colorizer,
    state: &mut StreamState,
    out: &mut W,
    mut syntax: Option<&mut SyntaxSession>,
) -> XcatResult<usize> {
    let mut buffer = Vec::with_capacity(opts.buffer_size.max(1));
    let mut total_lines = 0usize;

    loop {
        buffer.clear();
        let read = reader
            .read_until(b'\n', &mut buffer)
            .map_err(|e| XcatError::Io(e, source.to_string()))?;
        if read == 0 {
            break;
        }

        total_lines += 1;
        let (body, had_newline) = strip_trailing_newline(&buffer);
        let blank = is_blank_line(body);

        if opts.squeeze_blank && blank {
            state.blank_run += 1;
            if state.blank_run > 1 {
                continue;
            }
        } else {
            state.blank_run = 0;
        }

        if opts.number_nonblank {
            if !blank {
                state.line_number += 1;
                write_line_number(out, colorizer, state.line_number)
                    .map_err(|e| XcatError::Io(e, source.to_string()))?;
            }
        } else if opts.number {
            state.line_number += 1;
            write_line_number(out, colorizer, state.line_number)
                .map_err(|e| XcatError::Io(e, source.to_string()))?;
        }

        if let Some(syntax) = syntax.as_deref_mut() {
            if should_use_syntax_highlighting(opts, body) {
                if let Ok(text) = std::str::from_utf8(body) {
                    highlight_line(out, syntax, text, opts, colorizer, had_newline)
                        .map_err(|e| XcatError::Io(e, source.to_string()))?;
                    continue;
                }
            }
        }

        write_rendered_body(out, body, opts, colorizer, had_newline)?;
        if opts.show_ends && had_newline {
            write_end_marker(out, colorizer).map_err(|e| XcatError::Io(e, source.to_string()))?;
        }
        if had_newline {
            out.write_all(b"\n")
                .map_err(|e| XcatError::Io(e, source.to_string()))?;
        }
    }

    Ok(total_lines)
}

fn should_use_syntax_highlighting(opts: &DisplayOptions, body: &[u8]) -> bool {
    opts.syntax_highlighting && !opts.show_tabs && !opts.show_nonprinting && !body.contains(&b'\r')
}

fn copy_fast<W: Write>(
    mut file: File,
    source: &str,
    use_mmap: bool,
    out: &mut W,
) -> XcatResult<usize> {
    if use_mmap {
        match unsafe { memmap2::MmapOptions::new().map(&file) } {
            Ok(mmap) => {
                out.write_all(&mmap)
                    .map_err(|e| XcatError::Io(e, source.to_string()))?;
                return Ok(0);
            }
            Err(err) => {
                let _ = err;
            }
        }
    }

    let mut reader = BufReader::new(&mut file);
    io::copy(&mut reader, out).map_err(|e| XcatError::Io(e, source.to_string()))?;
    Ok(0)
}

#[inline]
fn can_fast_copy_plain(opts: &DisplayOptions, syntax_session_present: bool) -> bool {
    opts.should_render_plain_bytes() && !opts.count_lines && !syntax_session_present
}

struct SyntaxSession {
    comment_markers: &'static [&'static str],
    markup: bool,
}

fn syntax_session_for_path(path: &Path, opts: &DisplayOptions) -> Option<SyntaxSession> {
    if !opts.syntax_highlighting {
        return None;
    }

    Some(SyntaxSession {
        comment_markers: comment_markers_for_path(path),
        markup: is_markup_file(path),
    })
}

fn highlight_line<W: Write>(
    out: &mut W,
    syntax: &mut SyntaxSession,
    text: &str,
    opts: &DisplayOptions,
    colorizer: &Colorizer,
    had_newline: bool,
) -> io::Result<()> {
    highlight_text(out, text, syntax, colorizer)?;
    if opts.show_ends && had_newline {
        colorizer.write_end_marker(out)?;
    }
    if had_newline {
        out.write_all(b"\n")?;
    }
    Ok(())
}

fn highlight_text<W: Write>(
    out: &mut W,
    text: &str,
    syntax: &SyntaxSession,
    colorizer: &Colorizer,
) -> io::Result<()> {
    const KEYWORDS: &[&str] = &[
        "as",
        "async",
        "await",
        "become",
        "break",
        "case",
        "catch",
        "class",
        "const",
        "continue",
        "def",
        "default",
        "do",
        "else",
        "enum",
        "except",
        "export",
        "extends",
        "false",
        "finally",
        "fn",
        "for",
        "from",
        "function",
        "if",
        "impl",
        "import",
        "in",
        "interface",
        "let",
        "match",
        "mod",
        "mut",
        "namespace",
        "null",
        "object",
        "of",
        "package",
        "pass",
        "pub",
        "raise",
        "ref",
        "return",
        "self",
        "static",
        "struct",
        "super",
        "switch",
        "template",
        "this",
        "throw",
        "trait",
        "true",
        "try",
        "type",
        "union",
        "use",
        "var",
        "while",
        "where",
        "with",
        "yield",
    ];

    let mut i = 0usize;
    let mut plain_start = 0usize;
    let bytes = text.as_bytes();

    while i < bytes.len() {
        if syntax
            .comment_markers
            .iter()
            .any(|marker| is_line_comment(text, i, marker))
        {
            write_plain_text(out, &text[plain_start..i])?;
            colorizer.write_comment(out, &text[i..])?;
            plain_start = bytes.len();
            break;
        }

        let ch = text[i..].chars().next().unwrap_or_default();
        if syntax.markup && ch == '<' {
            if let Some(end) = text[i..].find('>') {
                let end = i + end + 1;
                write_plain_text(out, &text[plain_start..i])?;
                colorizer.write_keyword(out, &text[i..end])?;
                plain_start = end;
                i = end;
                continue;
            }
        }

        if ch == '"' || ch == '\'' || ch == '`' {
            let end = scan_quoted(text, i, ch);
            write_plain_text(out, &text[plain_start..i])?;
            colorizer.write_string(out, &text[i..end])?;
            plain_start = end;
            i = end;
            continue;
        }

        if ch.is_ascii_digit() {
            let end = scan_number(text, i);
            write_plain_text(out, &text[plain_start..i])?;
            colorizer.write_number(out, &text[i..end])?;
            plain_start = end;
            i = end;
            continue;
        }

        if is_ident_start(ch) {
            let end = scan_ident(text, i);
            let token = &text[i..end];
            if KEYWORDS.contains(&token) {
                write_plain_text(out, &text[plain_start..i])?;
                colorizer.write_keyword(out, token)?;
                plain_start = end;
            } else if next_non_ws_char(text, end) == Some('(') {
                write_plain_text(out, &text[plain_start..i])?;
                colorizer.write_function(out, token)?;
                plain_start = end;
            } else {
                // Defer plain spans so the common case stays as a single write.
            }
            i = end;
            continue;
        }

        i += ch.len_utf8();
    }

    write_plain_text(out, &text[plain_start..])?;
    Ok(())
}

fn is_line_comment(text: &str, index: usize, marker: &str) -> bool {
    if !text[index..].starts_with(marker) {
        return false;
    }

    text[..index].chars().all(|ch| ch.is_whitespace())
}

fn scan_quoted(text: &str, start: usize, quote: char) -> usize {
    let mut escaped = false;
    let mut end = start + quote.len_utf8();

    for (offset, ch) in text[end..].char_indices() {
        if escaped {
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == quote {
            end += offset + ch.len_utf8();
            return end;
        }
    }

    text.len()
}

fn scan_number(text: &str, start: usize) -> usize {
    let mut end = start;
    for (offset, ch) in text[start..].char_indices() {
        if offset == 0 {
            end += ch.len_utf8();
            continue;
        }
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' || ch == 'x' || ch == 'X' {
            end += ch.len_utf8();
        } else {
            break;
        }
    }
    end
}

fn scan_ident(text: &str, start: usize) -> usize {
    let mut end = start;
    for (offset, ch) in text[start..].char_indices() {
        if offset == 0 {
            end += ch.len_utf8();
            continue;
        }
        if is_ident_continue(ch) {
            end += ch.len_utf8();
        } else {
            break;
        }
    }
    end
}

fn next_non_ws_char(text: &str, index: usize) -> Option<char> {
    text[index..].chars().find(|ch| !ch.is_whitespace())
}

fn write_plain_text<W: Write>(out: &mut W, text: &str) -> io::Result<()> {
    if text.is_empty() {
        return Ok(());
    }

    out.write_all(text.as_bytes())
}

fn is_ident_start(ch: char) -> bool {
    ch == '_' || ch.is_alphabetic()
}

fn is_ident_continue(ch: char) -> bool {
    ch == '_' || ch.is_alphanumeric()
}

fn comment_markers_for_path(path: &Path) -> &'static [&'static str] {
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase()
        .as_str()
    {
        "sh" | "bash" | "zsh" | "fish" | "py" | "rb" | "pl" | "r" | "yml" | "yaml" | "toml"
        | "ini" | "cfg" | "conf" | "dockerfile" | "mk" | "make" => &["#"],
        "sql" | "psql" => &["--", "#"],
        "html" | "htm" | "xml" | "xhtml" | "svg" => &["<!--"],
        "rs" | "c" | "h" | "cc" | "cpp" | "cxx" | "hpp" | "hh" | "java" | "js" | "jsx" | "ts"
        | "tsx" | "go" | "kt" | "kts" | "swift" | "cs" | "php" | "scala" | "dart" => &["//", "/*"],
        _ => &["//", "#", "--"],
    }
}

fn is_markup_file(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str(),
        "html" | "htm" | "xml" | "xhtml" | "svg"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::ColorMode;
    use std::fs;
    use std::path::Path;
    use tempfile::NamedTempFile;

    fn opts() -> DisplayOptions {
        DisplayOptions {
            number: false,
            number_nonblank: false,
            show_ends: false,
            squeeze_blank: false,
            show_tabs: false,
            show_nonprinting: false,
            color_mode: ColorMode::Never,
            color_enabled: false,
            syntax_highlighting: false,
            theme_name: String::from("default"),
            use_mmap: true,
            buffer_size: 64 * 1024,
            count_lines: false,
            list_themes: false,
        }
    }

    #[test]
    fn fast_path_copies_plain_bytes() {
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), b"hello\nworld").unwrap();
        let mut out = Vec::new();
        let colorizer = Colorizer::new(false, "default");
        let mut state = StreamState::default();
        let result = process_source(
            file.path().to_str().unwrap(),
            &opts(),
            &colorizer,
            &mut state,
            &mut out,
        )
        .unwrap();

        assert_eq!(result, 0);
        assert_eq!(out, b"hello\nworld");
    }

    #[test]
    fn plain_stream_numbers_nonblank_lines() {
        let mut reader = io::Cursor::new(b"hello\n\nworld\n".to_vec());
        let mut out = Vec::new();
        let mut state = StreamState::default();
        let mut test_opts = opts();
        test_opts.number_nonblank = true;
        test_opts.number = false;
        let colorizer = Colorizer::new(false, "default");

        let count = process_reader(
            &mut reader,
            "-",
            &test_opts,
            &colorizer,
            &mut state,
            &mut out,
        )
        .unwrap();

        assert_eq!(count, 3);
        assert_eq!(
            String::from_utf8(out).unwrap(),
            "     1\thello\n\n     2\tworld\n"
        );
    }

    #[test]
    fn lightweight_syntax_highlighter_emits_ansi_for_keywords() {
        let mut test_opts = opts();
        test_opts.syntax_highlighting = true;
        test_opts.show_ends = true;
        let mut syntax = syntax_session_for_path(Path::new("main.rs"), &test_opts).unwrap();
        let colorizer = Colorizer::new(true, "default");
        let mut out = Vec::new();

        highlight_line(
            &mut out,
            &mut syntax,
            "fn main() { return 1; }",
            &test_opts,
            &colorizer,
            true,
        )
        .unwrap();

        let rendered = String::from_utf8(out).unwrap();
        assert!(rendered.contains("\u{1b}["));
        assert!(rendered.contains("fn"));
        assert!(rendered.contains("$"));
        assert!(rendered.ends_with('\n'));
    }

    #[test]
    fn lightweight_syntax_highlighter_skips_end_marker_without_newline() {
        let mut test_opts = opts();
        test_opts.syntax_highlighting = true;
        test_opts.show_ends = true;
        let mut syntax = syntax_session_for_path(Path::new("main.rs"), &test_opts).unwrap();
        let colorizer = Colorizer::new(true, "default");
        let mut out = Vec::new();

        highlight_line(
            &mut out,
            &mut syntax,
            "fn main() { return 1; }",
            &test_opts,
            &colorizer,
            false,
        )
        .unwrap();

        let rendered = String::from_utf8(out).unwrap();
        assert!(rendered.contains("\u{1b}["));
        assert!(rendered.contains("fn"));
        assert!(!rendered.contains('$'));
        assert!(!rendered.ends_with('\n'));
    }

    #[test]
    fn stdin_can_use_the_lightweight_highlighter_too() {
        let mut test_opts = opts();
        test_opts.syntax_highlighting = true;
        let mut syntax = syntax_session_for_path(Path::new("-"), &test_opts).unwrap();
        let colorizer = Colorizer::new(true, "default");
        let mut reader = io::Cursor::new(b"let answer = 42;\n".to_vec());
        let mut out = Vec::new();
        let mut state = StreamState::default();

        process_reader_with_syntax(
            &mut reader,
            "-",
            &test_opts,
            &colorizer,
            &mut state,
            &mut out,
            Some(&mut syntax),
        )
        .unwrap();

        let rendered = String::from_utf8(out).unwrap();
        assert!(rendered.contains("\u{1b}["));
        assert!(rendered.contains("let"));
        assert!(rendered.contains("42"));
    }
}
