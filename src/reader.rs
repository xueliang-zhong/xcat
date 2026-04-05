use std::io::{self, Write};
use std::str;

use crate::colorizer::Colorizer;
use crate::display::DisplayOptions;
use crate::error::XcatResult;

pub fn strip_trailing_newline(line: &[u8]) -> (&[u8], bool) {
    match line.last() {
        Some(b'\n') => (&line[..line.len() - 1], true),
        _ => (line, false),
    }
}

pub fn is_blank_line(body: &[u8]) -> bool {
    body.is_empty()
}

pub fn write_line_number<W: Write>(
    out: &mut W,
    colorizer: &Colorizer,
    line_number: usize,
) -> io::Result<()> {
    colorizer.write_line_number(out, line_number)
}

pub fn write_end_marker<W: Write>(out: &mut W, colorizer: &Colorizer) -> io::Result<()> {
    colorizer.write_end_marker(out)
}

pub fn write_rendered_body<W: Write>(
    out: &mut W,
    body: &[u8],
    opts: &DisplayOptions,
    colorizer: &Colorizer,
    had_newline: bool,
) -> XcatResult<()> {
    let mut plain_start = 0usize;
    let mut index = 0usize;

    while index < body.len() {
        let byte = body[index];

        if opts.show_tabs && byte == b'\t' {
            write_plain_slice(out, &body[plain_start..index])?;
            colorizer
                .write_tab_marker(out)
                .map_err(|e| crate::error::XcatError::Io(e, String::from("stdout")))?;
            plain_start = index + 1;
            index += 1;
            continue;
        }

        let is_line_ending_carriage_return = had_newline
            && opts.show_ends
            && !opts.show_nonprinting
            && index + 1 == body.len()
            && byte == b'\r';

        if is_line_ending_carriage_return {
            write_plain_slice(out, &body[plain_start..index])?;
            colorizer
                .write_nonprint(out, "^M")
                .map_err(|e| crate::error::XcatError::Io(e, String::from("stdout")))?;
            plain_start = index + 1;
            index += 1;
            continue;
        }

        if opts.show_nonprinting && should_render_nonprinting_byte(byte) {
            write_plain_slice(out, &body[plain_start..index])?;
            write_nonprinting_byte(out, colorizer, byte)
                .map_err(|e| crate::error::XcatError::Io(e, String::from("stdout")))?;
            plain_start = index + 1;
            index += 1;
            continue;
        }

        index += 1;
    }

    write_plain_slice(out, &body[plain_start..body.len()])?;

    Ok(())
}

pub fn render_nonprinting_byte(byte: u8) -> String {
    let (buf, len) = render_nonprinting_token(byte);
    String::from_utf8(buf[..len].to_vec()).expect("rendered nonprinting bytes are valid ASCII")
}

fn write_nonprinting_byte<W: Write>(
    out: &mut W,
    colorizer: &Colorizer,
    byte: u8,
) -> io::Result<()> {
    let (buf, len) = render_nonprinting_token(byte);
    let text = str::from_utf8(&buf[..len]).expect("rendered nonprinting bytes are valid ASCII");
    colorizer.write_nonprint(out, text)
}

fn write_plain_slice<W: Write>(out: &mut W, bytes: &[u8]) -> XcatResult<()> {
    if bytes.is_empty() {
        return Ok(());
    }

    out.write_all(bytes)
        .map_err(|e| crate::error::XcatError::Io(e, String::from("stdout")))
}

fn render_nonprinting_token(byte: u8) -> ([u8; 4], usize) {
    let mut buf = [0u8; 4];
    let len = match byte {
        b'\t' => {
            buf[..2].copy_from_slice(b"^I");
            2
        }
        0x00..=0x1F => {
            buf[0] = b'^';
            buf[1] = byte + 0x40;
            2
        }
        0x20..=0x7E => {
            buf[0] = byte;
            1
        }
        0x7F => {
            buf[..2].copy_from_slice(b"^?");
            2
        }
        0x80..=0xFF => {
            buf[..2].copy_from_slice(b"M-");
            let base = byte & 0x7F;
            match base {
                0x7F => {
                    buf[2..4].copy_from_slice(b"^?");
                    4
                }
                0x00..=0x1F => {
                    buf[2] = b'^';
                    buf[3] = base + 0x40;
                    4
                }
                _ => {
                    buf[2] = base;
                    3
                }
            }
        }
    };
    (buf, len)
}

fn should_render_nonprinting_byte(byte: u8) -> bool {
    !matches!(byte, b'\t' | 0x20..=0x7e)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colorizer::Colorizer;
    use std::io::Cursor;

    #[test]
    fn strips_newline_without_touching_crlf_body() {
        let (body, had_newline) = strip_trailing_newline(b"abc\r\n");
        assert_eq!(body, b"abc\r");
        assert!(had_newline);
    }

    #[test]
    fn render_nonprinting_matches_cat_style() {
        assert_eq!(render_nonprinting_byte(0x01), "^A");
        assert_eq!(render_nonprinting_byte(0x7F), "^?");
        assert_eq!(render_nonprinting_byte(0x80), "M-^@");
        assert_eq!(render_nonprinting_byte(0xFF), "M-^?");
    }

    #[test]
    fn writes_tab_marker_and_control_bytes() {
        let opts = DisplayOptions {
            number: false,
            number_nonblank: false,
            show_ends: false,
            squeeze_blank: false,
            show_tabs: true,
            show_nonprinting: true,
            color_mode: crate::cli::ColorMode::Never,
            color_enabled: false,
            syntax_highlighting: false,
            syntax: None,
            theme_name: String::from("default"),
            use_mmap: true,
            buffer_size: 64 * 1024,
            count_lines: false,
            list_themes: false,
        };
        let colorizer = Colorizer::new(false, "default");
        let mut out = Cursor::new(Vec::new());

        write_rendered_body(&mut out, b"a\t\x01", &opts, &colorizer, false).unwrap();
        assert_eq!(String::from_utf8(out.into_inner()).unwrap(), "a^I^A");
    }

    #[test]
    fn preserves_plain_runs_around_control_bytes() {
        let opts = DisplayOptions {
            number: false,
            number_nonblank: false,
            show_ends: false,
            squeeze_blank: false,
            show_tabs: false,
            show_nonprinting: true,
            color_mode: crate::cli::ColorMode::Never,
            color_enabled: false,
            syntax_highlighting: false,
            syntax: None,
            theme_name: String::from("default"),
            use_mmap: true,
            buffer_size: 64 * 1024,
            count_lines: false,
            list_themes: false,
        };
        let colorizer = Colorizer::new(false, "default");
        let mut out = Cursor::new(Vec::new());

        write_rendered_body(&mut out, b"abc\x01def", &opts, &colorizer, false).unwrap();
        assert_eq!(String::from_utf8(out.into_inner()).unwrap(), "abc^Adef");
    }
}
