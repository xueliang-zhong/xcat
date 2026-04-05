use clap::{ArgAction, Parser, ValueEnum};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ColorMode {
    #[default]
    Auto,
    Always,
    Never,
}

#[derive(Parser, Debug, Clone)]
#[command(
    name = "xcat",
    version,
    about = "Binary-safe cat with color, syntax highlighting, and config support",
    long_about = "xcat concatenates files to standard output while preserving Linux cat-style flags, optional syntax highlighting, and a configurable color theme."
)]
pub struct Cli {
    /// Number all output lines.
    #[arg(short = 'n', long = "number")]
    pub number: bool,

    /// Number non-empty output lines, overriding -n.
    #[arg(short = 'b', long = "number-nonblank")]
    pub number_nonblank: bool,

    /// Display $ at the end of each line.
    #[arg(short = 'E', long = "show-ends")]
    pub show_ends: bool,

    /// Squeeze repeated blank lines into a single blank line.
    #[arg(short = 's', long = "squeeze-blank")]
    pub squeeze_blank: bool,

    /// Display TAB characters as ^I.
    #[arg(short = 'T', long = "show-tabs")]
    pub show_tabs: bool,

    /// Display non-printing characters.
    #[arg(short = 'v', long = "show-nonprinting")]
    pub show_nonprinting: bool,

    /// Equivalent to -vET.
    #[arg(short = 'A', long = "show-all")]
    pub show_all: bool,

    /// Equivalent to -vE.
    #[arg(short = 'e', action = ArgAction::SetTrue)]
    pub legacy_e: bool,

    /// Equivalent to -vT.
    #[arg(short = 't', action = ArgAction::SetTrue)]
    pub legacy_t: bool,

    /// Files to concatenate. Reads stdin if empty or if "-" is supplied.
    #[arg(value_name = "FILE")]
    pub files: Vec<String>,

    /// Emit color in auto, always, or never mode.
    #[arg(long, value_enum)]
    pub color: Option<ColorMode>,

    /// Disable color output.
    #[arg(long = "no-color", action = ArgAction::SetTrue)]
    pub no_color: bool,

    /// Theme name used for line-number and marker colors.
    #[arg(long)]
    pub theme: Option<String>,

    /// Force syntax highlighting rules by language or file type.
    #[arg(long)]
    pub syntax: Option<String>,

    /// Show the configured theme names and exit.
    #[arg(long)]
    pub list_themes: bool,

    /// Show line count summary after concatenation.
    #[arg(short = 'c', long = "count-lines")]
    pub count_lines: bool,

    /// (ignored)
    #[arg(short = 'u', action = ArgAction::SetTrue)]
    pub unbuffered: bool,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn effective_show_nonprinting(&self) -> bool {
        self.show_nonprinting || self.show_all || self.legacy_e || self.legacy_t
    }

    pub fn effective_show_tabs(&self) -> bool {
        self.show_tabs || self.show_all || self.legacy_t
    }

    pub fn effective_show_ends(&self) -> bool {
        self.show_ends || self.show_all || self.legacy_e
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{CommandFactory, Parser};

    #[test]
    fn parses_cat_flags() {
        let cli = Cli::parse_from(["xcat", "-n", "-E", "-T", "file.txt"]);
        assert!(cli.number);
        assert!(cli.show_ends);
        assert!(cli.show_tabs);
        assert_eq!(cli.files, vec!["file.txt"]);
    }

    #[test]
    fn legacy_flags_expand_like_gnu_cat() {
        let cli = Cli::parse_from(["xcat", "-e", "-t"]);
        assert!(cli.effective_show_nonprinting());
        assert!(cli.effective_show_ends());
        assert!(cli.effective_show_tabs());
    }

    #[test]
    fn parses_color_mode_and_theme() {
        let cli = Cli::parse_from(["xcat", "--color", "always", "--theme", "nord"]);
        assert_eq!(cli.color, Some(ColorMode::Always));
        assert_eq!(cli.theme.as_deref(), Some("nord"));
    }

    #[test]
    fn help_includes_gnu_compatibility_flags() {
        let mut command = Cli::command();
        let help = command.render_help().to_string();

        assert!(help.contains("-e"));
        assert!(help.contains("-t"));
        assert!(help.contains("-u"));
    }
}
