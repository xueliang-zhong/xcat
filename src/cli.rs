use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "xcat", version, about = "Enhanced cat with color support", long_about = None)]
pub struct Cli {
    /// Number all output lines
    #[arg(short = 'n', long)]
    pub number: bool,

    /// Number nonempty output lines, overrides -n
    #[arg(short = 'b', long)]
    pub number_nonblank: bool,

    /// Display $ at end of each line
    #[arg(short = 'E', long)]
    pub show_ends: bool,

    /// Squeeze repeated blank lines into a single blank line
    #[arg(short = 's', long)]
    pub squeeze_blank: bool,

    /// Display TAB characters as ^I
    #[arg(short = 'T', long)]
    pub show_tabs: bool,

    /// Display nonprinting characters (except for LFD and TAB)
    #[arg(short = 'v', long)]
    pub show_nonprinting: bool,

    /// Equivalent to -vET
    #[arg(short = 'A', long)]
    pub show_all: bool,

    /// Files to concatenate (reads stdin if empty or '-')
    #[arg(value_name = "FILE")]
    pub files: Vec<String>,

    /// Disable color output
    #[arg(long)]
    pub no_color: bool,

    /// Show line count summary
    #[arg(short = 'c', long)]
    pub count_lines: bool,

    /// Color theme to use (default, monokai, solarized, github, nord, dracula, gruvbox, onedark, tokyonight, catppuccin)
    #[arg(long, default_value = "default")]
    pub theme: String,
}

impl Cli {
    pub fn parse_args() -> Self {
        Cli::parse()
    }

    pub fn effective_show_all(&self) -> bool {
        self.show_all
    }

    pub fn effective_show_nonprinting(&self) -> bool {
        self.show_nonprinting || self.show_all
    }

    pub fn effective_show_tabs(&self) -> bool {
        self.show_tabs || self.show_all
    }

    pub fn effective_show_ends(&self) -> bool {
        self.show_ends || self.show_all
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_default() {
        let cli = Cli::parse_from(["xcat"]);
        assert!(!cli.number);
        assert!(!cli.number_nonblank);
        assert!(!cli.show_ends);
        assert!(!cli.squeeze_blank);
        assert!(!cli.show_tabs);
        assert!(!cli.show_nonprinting);
        assert!(!cli.show_all);
        assert!(cli.files.is_empty());
        assert_eq!(cli.theme, "default");
    }

    #[test]
    fn test_cli_number() {
        let cli = Cli::parse_from(["xcat", "-n", "file.txt"]);
        assert!(cli.number);
        assert_eq!(cli.files, vec!["file.txt"]);
    }

    #[test]
    fn test_cli_number_nonblank() {
        let cli = Cli::parse_from(["xcat", "-b", "file.txt"]);
        assert!(cli.number_nonblank);
    }

    #[test]
    fn test_cli_show_ends() {
        let cli = Cli::parse_from(["xcat", "-E", "file.txt"]);
        assert!(cli.show_ends);
    }

    #[test]
    fn test_cli_squeeze_blank() {
        let cli = Cli::parse_from(["xcat", "-s", "file.txt"]);
        assert!(cli.squeeze_blank);
    }

    #[test]
    fn test_cli_show_tabs() {
        let cli = Cli::parse_from(["xcat", "-T", "file.txt"]);
        assert!(cli.show_tabs);
    }

    #[test]
    fn test_cli_show_nonprinting() {
        let cli = Cli::parse_from(["xcat", "-v", "file.txt"]);
        assert!(cli.show_nonprinting);
    }

    #[test]
    fn test_cli_show_all() {
        let cli = Cli::parse_from(["xcat", "-A", "file.txt"]);
        assert!(cli.show_all);
    }

    #[test]
    fn test_cli_effective_show_all() {
        let cli = Cli::parse_from(["xcat", "-A", "file.txt"]);
        assert!(cli.effective_show_nonprinting());
        assert!(cli.effective_show_tabs());
        assert!(cli.effective_show_ends());
    }

    #[test]
    fn test_cli_multiple_files() {
        let cli = Cli::parse_from(["xcat", "a.txt", "b.txt", "c.txt"]);
        assert_eq!(cli.files, vec!["a.txt", "b.txt", "c.txt"]);
    }

    #[test]
    fn test_cli_no_color() {
        let cli = Cli::parse_from(["xcat", "--no-color"]);
        assert!(cli.no_color);
    }

    #[test]
    fn test_cli_count_lines() {
        let cli = Cli::parse_from(["xcat", "-c"]);
        assert!(cli.count_lines);
    }

    #[test]
    fn test_cli_stdin_marker() {
        let cli = Cli::parse_from(["xcat", "-"]);
        assert_eq!(cli.files, vec!["-"]);
    }

    #[test]
    fn test_cli_theme_default() {
        let cli = Cli::parse_from(["xcat"]);
        assert_eq!(cli.theme, "default");
    }

    #[test]
    fn test_cli_theme_custom() {
        let cli = Cli::parse_from(["xcat", "--theme", "monokai"]);
        assert_eq!(cli.theme, "monokai");
    }

    #[test]
    fn test_cli_theme_nord() {
        let cli = Cli::parse_from(["xcat", "--theme", "nord"]);
        assert_eq!(cli.theme, "nord");
    }

    #[test]
    fn test_cli_debug() {
        let cli = Cli::parse_from(["xcat"]);
        let debug_str = format!("{cli:?}");
        assert!(debug_str.contains("Cli"));
    }

    #[test]
    fn test_cli_clone() {
        let cli = Cli::parse_from(["xcat", "-n", "file.txt"]);
        let cloned = cli.clone();
        assert_eq!(cli.files, cloned.files);
        assert_eq!(cli.number, cloned.number);
    }
}
