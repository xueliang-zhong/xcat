use crate::cli::Cli;
use crate::config::Config;

#[derive(Debug, Clone)]
pub struct DisplayOptions {
    pub number: bool,
    pub number_nonblank: bool,
    pub show_ends: bool,
    pub squeeze_blank: bool,
    pub show_tabs: bool,
    pub show_nonprinting: bool,
    pub no_color: bool,
    pub color_enabled: bool,
    pub count_lines: bool,
    pub use_mmap: bool,
    pub theme_name: String,
}

impl DisplayOptions {
    pub fn from_cli_and_config(cli: &Cli, config: &Config) -> Self {
        let color_from_config = config.color.enabled && !cli.no_color;

        Self {
            number: cli.number || config.display.number,
            number_nonblank: cli.number_nonblank || config.display.number_nonblank,
            show_ends: cli.effective_show_ends() || config.display.show_ends,
            squeeze_blank: cli.squeeze_blank || config.display.squeeze_blank,
            show_tabs: cli.effective_show_tabs() || config.display.show_tabs,
            show_nonprinting: cli.effective_show_nonprinting() || config.display.show_nonprinting,
            no_color: cli.no_color,
            color_enabled: color_from_config,
            count_lines: cli.count_lines,
            use_mmap: config.performance.use_mmap,
            theme_name: config.color.theme.clone(),
        }
    }

    pub fn number_lines(&self) -> bool {
        self.number || self.number_nonblank
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
    fn test_display_options_default() {
        let cli = make_cli(&[]);
        let config = Config::default();
        let opts = DisplayOptions::from_cli_and_config(&cli, &config);

        assert!(!opts.number);
        assert!(!opts.number_nonblank);
        assert!(!opts.show_ends);
        assert!(!opts.squeeze_blank);
        assert!(!opts.show_tabs);
        assert!(!opts.show_nonprinting);
        assert!(opts.color_enabled);
        assert!(opts.use_mmap);
        assert_eq!(opts.theme_name, "default");
    }

    #[test]
    fn test_display_options_from_cli_flags() {
        let cli = make_cli(&["-n", "-E", "-T", "file.txt"]);
        let config = Config::default();
        let opts = DisplayOptions::from_cli_and_config(&cli, &config);

        assert!(opts.number);
        assert!(opts.show_ends);
        assert!(opts.show_tabs);
    }

    #[test]
    fn test_display_options_from_config() {
        let cli = make_cli(&[]);
        let mut config = Config::default();
        config.display.number = true;
        config.display.show_ends = true;

        let opts = DisplayOptions::from_cli_and_config(&cli, &config);
        assert!(opts.number);
        assert!(opts.show_ends);
    }

    #[test]
    fn test_cli_overrides_config() {
        let cli = make_cli(&["-n", "file.txt"]);
        let mut config = Config::default();
        config.display.number = false;

        let opts = DisplayOptions::from_cli_and_config(&cli, &config);
        assert!(opts.number);
    }

    #[test]
    fn test_no_color_disables_coloring() {
        let cli = make_cli(&["--no-color"]);
        let config = Config::default();
        let opts = DisplayOptions::from_cli_and_config(&cli, &config);

        assert!(!opts.color_enabled);
    }

    #[test]
    fn test_config_color_disabled() {
        let cli = make_cli(&[]);
        let mut config = Config::default();
        config.color.enabled = false;

        let opts = DisplayOptions::from_cli_and_config(&cli, &config);
        assert!(!opts.color_enabled);
    }

    #[test]
    fn test_show_all_expands_flags() {
        let cli = make_cli(&["-A", "file.txt"]);
        let config = Config::default();
        let opts = DisplayOptions::from_cli_and_config(&cli, &config);

        assert!(opts.show_nonprinting);
        assert!(opts.show_tabs);
        assert!(opts.show_ends);
    }

    #[test]
    fn test_number_lines() {
        let opts = DisplayOptions {
            number: true,
            number_nonblank: false,
            show_ends: false,
            squeeze_blank: false,
            show_tabs: false,
            show_nonprinting: false,
            no_color: false,
            color_enabled: true,
            count_lines: false,
            use_mmap: true,
            theme_name: String::from("default"),
        };
        assert!(opts.number_lines());
    }

    #[test]
    fn test_number_lines_nonblank() {
        let opts = DisplayOptions {
            number: false,
            number_nonblank: true,
            show_ends: false,
            squeeze_blank: false,
            show_tabs: false,
            show_nonprinting: false,
            no_color: false,
            color_enabled: true,
            count_lines: false,
            use_mmap: true,
            theme_name: String::from("default"),
        };
        assert!(opts.number_lines());
    }

    #[test]
    fn test_no_numbering() {
        let opts = DisplayOptions {
            number: false,
            number_nonblank: false,
            show_ends: false,
            squeeze_blank: false,
            show_tabs: false,
            show_nonprinting: false,
            no_color: false,
            color_enabled: true,
            count_lines: false,
            use_mmap: true,
            theme_name: String::from("default"),
        };
        assert!(!opts.number_lines());
    }

    #[test]
    fn test_display_options_mmap_config() {
        let cli = make_cli(&[]);
        let mut config = Config::default();
        config.performance.use_mmap = false;
        let opts = DisplayOptions::from_cli_and_config(&cli, &config);
        assert!(!opts.use_mmap);
    }

    #[test]
    fn test_display_options_theme_config() {
        let cli = make_cli(&[]);
        let mut config = Config::default();
        config.color.theme = String::from("monokai");
        let opts = DisplayOptions::from_cli_and_config(&cli, &config);
        assert_eq!(opts.theme_name, "monokai");
    }
}
