use crate::cli::{Cli, ColorMode};
use crate::config::Config;

#[derive(Debug, Clone)]
pub struct DisplayOptions {
    pub number: bool,
    pub number_nonblank: bool,
    pub show_ends: bool,
    pub squeeze_blank: bool,
    pub show_tabs: bool,
    pub show_nonprinting: bool,
    pub color_mode: ColorMode,
    pub color_enabled: bool,
    pub syntax_highlighting: bool,
    pub syntax: Option<String>,
    pub theme_name: String,
    pub use_mmap: bool,
    pub buffer_size: usize,
    pub count_lines: bool,
    pub list_themes: bool,
}

impl DisplayOptions {
    pub fn from_cli_and_config(cli: &Cli, config: &Config, stdout_is_terminal: bool) -> Self {
        let number_nonblank = cli.number_nonblank || config.display.number_nonblank;
        let number = (cli.number || config.display.number) && !number_nonblank;

        let show_ends = cli.effective_show_ends() || config.display.show_ends;
        let show_tabs = cli.effective_show_tabs() || config.display.show_tabs;
        let show_nonprinting = cli.effective_show_nonprinting() || config.display.show_nonprinting;
        let squeeze_blank = cli.squeeze_blank || config.display.squeeze_blank;
        let count_lines = cli.count_lines;
        let list_themes = cli.list_themes;

        let color_mode = cli.color.unwrap_or(config.color.mode);
        let color_mode = if cli.no_color {
            ColorMode::Never
        } else {
            color_mode
        };

        let color_enabled = match color_mode {
            ColorMode::Always => true,
            ColorMode::Never => false,
            ColorMode::Auto => stdout_is_terminal,
        };

        Self {
            number,
            number_nonblank,
            show_ends,
            squeeze_blank,
            show_tabs,
            show_nonprinting,
            color_mode,
            color_enabled,
            syntax_highlighting: color_enabled
                && (config.color.syntax_highlighting || cli.syntax.is_some()),
            syntax: cli
                .syntax
                .clone()
                .or_else(|| config.color.syntax.clone())
                .and_then(|syntax| {
                    let syntax = syntax.trim();
                    (!syntax.is_empty()).then(|| syntax.to_string())
                }),
            theme_name: cli
                .theme
                .clone()
                .unwrap_or_else(|| config.color.theme.clone()),
            use_mmap: config.performance.use_mmap,
            buffer_size: config.performance.buffer_size,
            count_lines,
            list_themes,
        }
    }

    pub fn numbering_enabled(&self) -> bool {
        self.number_nonblank || self.number
    }

    pub fn should_render_plain_bytes(&self) -> bool {
        !self.numbering_enabled()
            && !self.show_ends
            && !self.squeeze_blank
            && !self.show_tabs
            && !self.show_nonprinting
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn make_cli(args: &[&str]) -> Cli {
        let mut full_args = vec!["xcat"];
        full_args.extend(args.iter().copied());
        Cli::parse_from(full_args)
    }

    #[test]
    fn cli_overrides_config_and_auto_color_requires_terminal() {
        let cli = make_cli(&["-n", "--color", "auto"]);
        let mut config = Config::default();
        config.display.show_ends = true;
        let opts = DisplayOptions::from_cli_and_config(&cli, &config, false);

        assert!(!opts.color_enabled);
        assert!(opts.number);
        assert!(opts.show_ends);
    }

    #[test]
    fn number_nonblank_wins_over_number() {
        let cli = make_cli(&["-n", "-b"]);
        let config = Config::default();
        let opts = DisplayOptions::from_cli_and_config(&cli, &config, true);

        assert!(opts.number_nonblank);
        assert!(!opts.number);
        assert!(opts.numbering_enabled());
    }

    #[test]
    fn no_color_forces_never() {
        let cli = make_cli(&["--color", "always", "--no-color"]);
        let config = Config::default();
        let opts = DisplayOptions::from_cli_and_config(&cli, &config, true);

        assert_eq!(opts.color_mode, ColorMode::Never);
        assert!(!opts.color_enabled);
        assert!(!opts.syntax_highlighting);
    }

    #[test]
    fn cli_syntax_hint_overrides_config() {
        let cli = make_cli(&["--syntax", "json"]);
        let mut config = Config::default();
        config.color.syntax = Some(String::from("rust"));
        let opts = DisplayOptions::from_cli_and_config(&cli, &config, true);

        assert_eq!(opts.syntax.as_deref(), Some("json"));
    }

    #[test]
    fn explicit_syntax_hint_reenables_highlighting_even_when_config_disables_it() {
        let cli = make_cli(&["--syntax", "json"]);
        let mut config = Config::default();
        config.color.syntax_highlighting = false;
        let opts = DisplayOptions::from_cli_and_config(&cli, &config, true);

        assert!(opts.syntax_highlighting);
    }
}
