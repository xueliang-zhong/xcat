use std::io::{self, Write};

use nu_ansi_term::{Color, Style};

#[derive(Debug, Clone)]
pub struct ThemePalette {
    pub line_number: Style,
    pub end_marker: Style,
    pub tab_marker: Style,
    pub nonprint: Style,
    pub keyword: Style,
    pub string: Style,
    pub comment: Style,
    pub number: Style,
    pub function: Style,
}

#[derive(Debug, Clone)]
pub struct Colorizer {
    pub enabled: bool,
    theme_name: String,
    palette: ThemePalette,
}

const AVAILABLE_THEMES: &[&str] = &[
    "default",
    "monokai",
    "solarized",
    "github",
    "nord",
    "dracula",
    "gruvbox",
    "onedark",
    "tokyonight",
    "catppuccin",
];

impl Colorizer {
    pub fn new(enabled: bool, theme_name: &str) -> Self {
        Self {
            enabled,
            theme_name: theme_name.to_string(),
            palette: palette_for(theme_name),
        }
    }

    pub fn theme_name(&self) -> &str {
        &self.theme_name
    }

    pub fn available_themes() -> &'static [&'static str] {
        AVAILABLE_THEMES
    }

    pub fn syntax_theme_candidates(&self) -> [&'static str; 2] {
        syntax_theme_candidates(self.theme_name())
    }

    pub fn colorize_line_number(&self, num: usize) -> String {
        let text = format!("{num:>6}\t");
        if self.enabled {
            self.palette.line_number.paint(text).to_string()
        } else {
            text
        }
    }

    pub fn colorize_end_marker(&self) -> String {
        let text = "$";
        if self.enabled {
            self.palette.end_marker.paint(text).to_string()
        } else {
            text.to_string()
        }
    }

    pub fn colorize_tab_marker(&self) -> String {
        let text = "^I";
        if self.enabled {
            self.palette.tab_marker.paint(text).to_string()
        } else {
            text.to_string()
        }
    }

    pub fn colorize_nonprint(&self, text: &str) -> String {
        if self.enabled {
            self.palette.nonprint.paint(text).to_string()
        } else {
            text.to_string()
        }
    }

    pub fn colorize_keyword(&self, text: &str) -> String {
        if self.enabled {
            self.palette.keyword.paint(text).to_string()
        } else {
            text.to_string()
        }
    }

    pub fn colorize_string(&self, text: &str) -> String {
        if self.enabled {
            self.palette.string.paint(text).to_string()
        } else {
            text.to_string()
        }
    }

    pub fn colorize_comment(&self, text: &str) -> String {
        if self.enabled {
            self.palette.comment.paint(text).to_string()
        } else {
            text.to_string()
        }
    }

    pub fn colorize_number(&self, text: &str) -> String {
        if self.enabled {
            self.palette.number.paint(text).to_string()
        } else {
            text.to_string()
        }
    }

    pub fn colorize_function(&self, text: &str) -> String {
        if self.enabled {
            self.palette.function.paint(text).to_string()
        } else {
            text.to_string()
        }
    }

    pub fn write_line_number<W: Write>(&self, out: &mut W, num: usize) -> io::Result<()> {
        if self.enabled {
            write!(
                out,
                "{}{:>6}\t{}",
                self.palette.line_number.prefix(),
                num,
                self.palette.line_number.suffix()
            )
        } else {
            write!(out, "{num:>6}\t")
        }
    }

    pub fn write_end_marker<W: Write>(&self, out: &mut W) -> io::Result<()> {
        self.write_styled_text(out, self.palette.end_marker, "$")
    }

    pub fn write_tab_marker<W: Write>(&self, out: &mut W) -> io::Result<()> {
        self.write_styled_text(out, self.palette.tab_marker, "^I")
    }

    pub fn write_nonprint<W: Write>(&self, out: &mut W, text: &str) -> io::Result<()> {
        self.write_styled_text(out, self.palette.nonprint, text)
    }

    pub fn write_keyword<W: Write>(&self, out: &mut W, text: &str) -> io::Result<()> {
        self.write_styled_text(out, self.palette.keyword, text)
    }

    pub fn write_string<W: Write>(&self, out: &mut W, text: &str) -> io::Result<()> {
        self.write_styled_text(out, self.palette.string, text)
    }

    pub fn write_comment<W: Write>(&self, out: &mut W, text: &str) -> io::Result<()> {
        self.write_styled_text(out, self.palette.comment, text)
    }

    pub fn write_number<W: Write>(&self, out: &mut W, text: &str) -> io::Result<()> {
        self.write_styled_text(out, self.palette.number, text)
    }

    pub fn write_function<W: Write>(&self, out: &mut W, text: &str) -> io::Result<()> {
        self.write_styled_text(out, self.palette.function, text)
    }

    fn write_styled_text<W: Write>(&self, out: &mut W, style: Style, text: &str) -> io::Result<()> {
        if self.enabled {
            write!(out, "{}{}{}", style.prefix(), text, style.suffix())
        } else {
            out.write_all(text.as_bytes())
        }
    }
}

fn palette_for(theme_name: &str) -> ThemePalette {
    match theme_name.to_ascii_lowercase().as_str() {
        "monokai" => ThemePalette {
            line_number: Style::new().fg(Color::Fixed(141)).bold(),
            end_marker: Style::new().fg(Color::Fixed(197)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(228)).bold(),
            nonprint: Style::new().fg(Color::Fixed(198)),
            keyword: Style::new().fg(Color::Fixed(81)).bold(),
            string: Style::new().fg(Color::Fixed(186)),
            comment: Style::new().fg(Color::Fixed(102)).italic(),
            number: Style::new().fg(Color::Fixed(203)),
            function: Style::new().fg(Color::Fixed(220)).bold(),
        },
        "solarized" => ThemePalette {
            line_number: Style::new().fg(Color::Fixed(33)).bold(),
            end_marker: Style::new().fg(Color::Fixed(160)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(136)).bold(),
            nonprint: Style::new().fg(Color::Fixed(125)),
            keyword: Style::new().fg(Color::Fixed(33)).bold(),
            string: Style::new().fg(Color::Fixed(64)),
            comment: Style::new().fg(Color::Fixed(244)).italic(),
            number: Style::new().fg(Color::Fixed(166)),
            function: Style::new().fg(Color::Fixed(37)).bold(),
        },
        "github" => ThemePalette {
            line_number: Style::new().fg(Color::Fixed(103)).bold(),
            end_marker: Style::new().fg(Color::Fixed(161)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(94)).bold(),
            nonprint: Style::new().fg(Color::Fixed(126)),
            keyword: Style::new().fg(Color::Fixed(27)).bold(),
            string: Style::new().fg(Color::Fixed(28)),
            comment: Style::new().fg(Color::Fixed(245)).italic(),
            number: Style::new().fg(Color::Fixed(124)),
            function: Style::new().fg(Color::Fixed(24)).bold(),
        },
        "nord" => ThemePalette {
            line_number: Style::new().fg(Color::Fixed(67)).bold(),
            end_marker: Style::new().fg(Color::Fixed(203)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(180)).bold(),
            nonprint: Style::new().fg(Color::Fixed(210)),
            keyword: Style::new().fg(Color::Fixed(111)).bold(),
            string: Style::new().fg(Color::Fixed(152)),
            comment: Style::new().fg(Color::Fixed(103)).italic(),
            number: Style::new().fg(Color::Fixed(81)),
            function: Style::new().fg(Color::Fixed(109)).bold(),
        },
        "dracula" => ThemePalette {
            line_number: Style::new().fg(Color::Fixed(177)).bold(),
            end_marker: Style::new().fg(Color::Fixed(210)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(228)).bold(),
            nonprint: Style::new().fg(Color::Fixed(203)),
            keyword: Style::new().fg(Color::Fixed(141)).bold(),
            string: Style::new().fg(Color::Fixed(230)),
            comment: Style::new().fg(Color::Fixed(244)).italic(),
            number: Style::new().fg(Color::Fixed(198)),
            function: Style::new().fg(Color::Fixed(117)).bold(),
        },
        "gruvbox" => ThemePalette {
            line_number: Style::new().fg(Color::Fixed(109)).bold(),
            end_marker: Style::new().fg(Color::Fixed(167)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(214)).bold(),
            nonprint: Style::new().fg(Color::Fixed(175)),
            keyword: Style::new().fg(Color::Fixed(172)).bold(),
            string: Style::new().fg(Color::Fixed(142)),
            comment: Style::new().fg(Color::Fixed(244)).italic(),
            number: Style::new().fg(Color::Fixed(108)),
            function: Style::new().fg(Color::Fixed(214)).bold(),
        },
        "onedark" => ThemePalette {
            line_number: Style::new().fg(Color::Fixed(67)).bold(),
            end_marker: Style::new().fg(Color::Fixed(204)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(180)).bold(),
            nonprint: Style::new().fg(Color::Fixed(211)),
            keyword: Style::new().fg(Color::Fixed(75)).bold(),
            string: Style::new().fg(Color::Fixed(180)),
            comment: Style::new().fg(Color::Fixed(244)).italic(),
            number: Style::new().fg(Color::Fixed(180)),
            function: Style::new().fg(Color::Fixed(110)).bold(),
        },
        "tokyonight" => ThemePalette {
            line_number: Style::new().fg(Color::Fixed(75)).bold(),
            end_marker: Style::new().fg(Color::Fixed(203)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(222)).bold(),
            nonprint: Style::new().fg(Color::Fixed(217)),
            keyword: Style::new().fg(Color::Fixed(75)).bold(),
            string: Style::new().fg(Color::Fixed(151)),
            comment: Style::new().fg(Color::Fixed(244)).italic(),
            number: Style::new().fg(Color::Fixed(180)),
            function: Style::new().fg(Color::Fixed(110)).bold(),
        },
        "catppuccin" => ThemePalette {
            line_number: Style::new().fg(Color::Fixed(153)).bold(),
            end_marker: Style::new().fg(Color::Fixed(203)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(229)).bold(),
            nonprint: Style::new().fg(Color::Fixed(204)),
            keyword: Style::new().fg(Color::Fixed(117)).bold(),
            string: Style::new().fg(Color::Fixed(214)),
            comment: Style::new().fg(Color::Fixed(244)).italic(),
            number: Style::new().fg(Color::Fixed(110)),
            function: Style::new().fg(Color::Fixed(151)).bold(),
        },
        _ => ThemePalette {
            line_number: Style::new().fg(Color::Cyan).bold(),
            end_marker: Style::new().fg(Color::Red).bold(),
            tab_marker: Style::new().fg(Color::Yellow).bold(),
            nonprint: Style::new().fg(Color::Magenta),
            keyword: Style::new().fg(Color::Blue).bold(),
            string: Style::new().fg(Color::Green),
            comment: Style::new().fg(Color::Fixed(244)).italic(),
            number: Style::new().fg(Color::Cyan),
            function: Style::new().fg(Color::Yellow).bold(),
        },
    }
}

fn syntax_theme_candidates(theme_name: &str) -> [&'static str; 2] {
    match theme_name.to_ascii_lowercase().as_str() {
        "default" => ["InspiredGitHub", "base16-ocean.dark"],
        "github" => ["InspiredGitHub", "base16-ocean.dark"],
        "monokai" => ["Monokai Extended", "base16-ocean.dark"],
        "solarized" => ["Solarized (dark)", "base16-ocean.dark"],
        "nord" => ["Nord", "base16-ocean.dark"],
        "dracula" => ["Dracula", "base16-ocean.dark"],
        "gruvbox" => ["base16-gruvbox-dark-medium", "base16-ocean.dark"],
        "onedark" => ["base16-onedark.dark", "base16-ocean.dark"],
        "tokyonight" => ["base16-tokyonight-dark", "base16-ocean.dark"],
        "catppuccin" => ["Catppuccin Mocha", "base16-ocean.dark"],
        _ => ["InspiredGitHub", "base16-ocean.dark"],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn disabled_colorizer_returns_plain_text() {
        let colorizer = Colorizer::new(false, "default");
        assert_eq!(colorizer.colorize_line_number(7), "     7\t");
        assert_eq!(colorizer.colorize_tab_marker(), "^I");
        assert_eq!(colorizer.colorize_nonprint("^A"), "^A");
    }

    #[test]
    fn enabled_colorizer_uses_theme() {
        let colorizer = Colorizer::new(true, "nord");
        assert!(colorizer.colorize_line_number(1).contains("1"));
        assert_eq!(colorizer.theme_name(), "nord");
        assert_eq!(colorizer.syntax_theme_candidates()[1], "base16-ocean.dark");
    }

    #[test]
    fn direct_write_helpers_emit_styled_text() {
        let colorizer = Colorizer::new(true, "nord");
        let mut out = Cursor::new(Vec::new());

        colorizer.write_line_number(&mut out, 42).unwrap();
        colorizer.write_keyword(&mut out, "fn").unwrap();
        colorizer.write_comment(&mut out, "# comment").unwrap();

        let rendered = String::from_utf8(out.into_inner()).unwrap();
        assert!(rendered.contains("\u{1b}["));
        assert!(rendered.contains("    42\t"));
        assert!(rendered.contains("fn"));
        assert!(rendered.contains("# comment"));
    }
}
