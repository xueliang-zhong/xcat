use nu_ansi_term::{Color, Style};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorDepth {
    Basic16,
    Extended256,
    TrueColor,
}

impl Default for ColorDepth {
    fn default() -> Self {
        ColorDepth::Extended256
    }
}

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub line_number: Style,
    pub end_marker: Style,
    pub tab_marker: Style,
    pub nonprint: Style,
    pub error: Style,
    pub header: Style,
    pub separator: Style,
}

pub trait ColorTheme {
    fn name(&self) -> &str;
    fn colors(&self) -> ThemeColors;
}

#[derive(Debug, Clone)]
pub struct DefaultTheme;
impl ColorTheme for DefaultTheme {
    fn name(&self) -> &str { "default" }
    fn colors(&self) -> ThemeColors {
        ThemeColors {
            line_number: Style::new().fg(Color::Cyan).bold(),
            end_marker: Style::new().fg(Color::Red).bold(),
            tab_marker: Style::new().fg(Color::Yellow).bold(),
            nonprint: Style::new().fg(Color::Magenta),
            error: Style::new().fg(Color::Red).bold(),
            header: Style::new().fg(Color::Green).bold().underline(),
            separator: Style::new().fg(Color::Blue).dimmed(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonokaiTheme;
impl ColorTheme for MonokaiTheme {
    fn name(&self) -> &str { "monokai" }
    fn colors(&self) -> ThemeColors {
        ThemeColors {
            line_number: Style::new().fg(Color::Fixed(141)).bold(),
            end_marker: Style::new().fg(Color::Fixed(197)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(228)).bold(),
            nonprint: Style::new().fg(Color::Fixed(198)),
            error: Style::new().fg(Color::Fixed(197)).bold(),
            header: Style::new().fg(Color::Fixed(148)).bold().underline(),
            separator: Style::new().fg(Color::Fixed(242)).dimmed(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SolarizedTheme;
impl ColorTheme for SolarizedTheme {
    fn name(&self) -> &str { "solarized" }
    fn colors(&self) -> ThemeColors {
        ThemeColors {
            line_number: Style::new().fg(Color::Fixed(33)).bold(),
            end_marker: Style::new().fg(Color::Fixed(160)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(136)).bold(),
            nonprint: Style::new().fg(Color::Fixed(125)),
            error: Style::new().fg(Color::Fixed(160)).bold(),
            header: Style::new().fg(Color::Fixed(64)).bold().underline(),
            separator: Style::new().fg(Color::Fixed(245)).dimmed(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GithubTheme;
impl ColorTheme for GithubTheme {
    fn name(&self) -> &str { "github" }
    fn colors(&self) -> ThemeColors {
        ThemeColors {
            line_number: Style::new().fg(Color::Fixed(103)).bold(),
            end_marker: Style::new().fg(Color::Fixed(161)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(94)).bold(),
            nonprint: Style::new().fg(Color::Fixed(126)),
            error: Style::new().fg(Color::Fixed(161)).bold(),
            header: Style::new().fg(Color::Fixed(22)).bold().underline(),
            separator: Style::new().fg(Color::Fixed(249)).dimmed(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NordTheme;
impl ColorTheme for NordTheme {
    fn name(&self) -> &str { "nord" }
    fn colors(&self) -> ThemeColors {
        ThemeColors {
            line_number: Style::new().fg(Color::Fixed(67)).bold(),
            end_marker: Style::new().fg(Color::Fixed(203)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(180)).bold(),
            nonprint: Style::new().fg(Color::Fixed(210)),
            error: Style::new().fg(Color::Fixed(203)).bold(),
            header: Style::new().fg(Color::Fixed(109)).bold().underline(),
            separator: Style::new().fg(Color::Fixed(59)).dimmed(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DraculaTheme;
impl ColorTheme for DraculaTheme {
    fn name(&self) -> &str { "dracula" }
    fn colors(&self) -> ThemeColors {
        ThemeColors {
            line_number: Style::new().fg(Color::Fixed(177)).bold(),
            end_marker: Style::new().fg(Color::Fixed(210)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(228)).bold(),
            nonprint: Style::new().fg(Color::Fixed(203)),
            error: Style::new().fg(Color::Fixed(210)).bold(),
            header: Style::new().fg(Color::Fixed(83)).bold().underline(),
            separator: Style::new().fg(Color::Fixed(60)).dimmed(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GruvboxTheme;
impl ColorTheme for GruvboxTheme {
    fn name(&self) -> &str { "gruvbox" }
    fn colors(&self) -> ThemeColors {
        ThemeColors {
            line_number: Style::new().fg(Color::Fixed(109)).bold(),
            end_marker: Style::new().fg(Color::Fixed(167)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(214)).bold(),
            nonprint: Style::new().fg(Color::Fixed(175)),
            error: Style::new().fg(Color::Fixed(167)).bold(),
            header: Style::new().fg(Color::Fixed(142)).bold().underline(),
            separator: Style::new().fg(Color::Fixed(245)).dimmed(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OneDarkTheme;
impl ColorTheme for OneDarkTheme {
    fn name(&self) -> &str { "onedark" }
    fn colors(&self) -> ThemeColors {
        ThemeColors {
            line_number: Style::new().fg(Color::Fixed(67)).bold(),
            end_marker: Style::new().fg(Color::Fixed(204)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(180)).bold(),
            nonprint: Style::new().fg(Color::Fixed(211)),
            error: Style::new().fg(Color::Fixed(204)).bold(),
            header: Style::new().fg(Color::Fixed(114)).bold().underline(),
            separator: Style::new().fg(Color::Fixed(59)).dimmed(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokyoNightTheme;
impl ColorTheme for TokyoNightTheme {
    fn name(&self) -> &str { "tokyonight" }
    fn colors(&self) -> ThemeColors {
        ThemeColors {
            line_number: Style::new().fg(Color::Fixed(75)).bold(),
            end_marker: Style::new().fg(Color::Fixed(203)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(222)).bold(),
            nonprint: Style::new().fg(Color::Fixed(217)),
            error: Style::new().fg(Color::Fixed(203)).bold(),
            header: Style::new().fg(Color::Fixed(110)).bold().underline(),
            separator: Style::new().fg(Color::Fixed(60)).dimmed(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CatppuccinTheme;
impl ColorTheme for CatppuccinTheme {
    fn name(&self) -> &str { "catppuccin" }
    fn colors(&self) -> ThemeColors {
        ThemeColors {
            line_number: Style::new().fg(Color::Fixed(153)).bold(),
            end_marker: Style::new().fg(Color::Fixed(203)).bold(),
            tab_marker: Style::new().fg(Color::Fixed(229)).bold(),
            nonprint: Style::new().fg(Color::Fixed(204)),
            error: Style::new().fg(Color::Fixed(203)).bold(),
            header: Style::new().fg(Color::Fixed(151)).bold().underline(),
            separator: Style::new().fg(Color::Fixed(103)).dimmed(),
        }
    }
}

pub struct ThemeRegistry {
    themes: HashMap<String, Box<dyn ColorTheme>>,
}

impl ThemeRegistry {
    pub fn new() -> Self {
        let mut reg = Self {
            themes: HashMap::new(),
        };
        reg.register(Box::new(DefaultTheme));
        reg.register(Box::new(MonokaiTheme));
        reg.register(Box::new(SolarizedTheme));
        reg.register(Box::new(GithubTheme));
        reg.register(Box::new(NordTheme));
        reg.register(Box::new(DraculaTheme));
        reg.register(Box::new(GruvboxTheme));
        reg.register(Box::new(OneDarkTheme));
        reg.register(Box::new(TokyoNightTheme));
        reg.register(Box::new(CatppuccinTheme));
        reg
    }

    pub fn register(&mut self, theme: Box<dyn ColorTheme>) {
        self.themes.insert(theme.name().to_string(), theme);
    }

    pub fn get(&self, name: &str) -> Option<&dyn ColorTheme> {
        self.themes.get(name).map(|t| t.as_ref())
    }

    pub fn default_theme(&self) -> &dyn ColorTheme {
        self.get("default").unwrap_or_else(|| &DefaultTheme)
    }

    pub fn available_themes(&self) -> Vec<&str> {
        self.themes.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for ThemeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Colorizer {
    pub enabled: bool,
    pub colors: ThemeColors,
    pub depth: ColorDepth,
}

impl Colorizer {
    pub fn new(enabled: bool) -> Self {
        Self::with_theme(enabled, &DefaultTheme)
    }

    pub fn with_theme(enabled: bool, theme: &dyn ColorTheme) -> Self {
        Self {
            enabled,
            colors: theme.colors(),
            depth: ColorDepth::default(),
        }
    }

    pub fn with_depth(mut self, depth: ColorDepth) -> Self {
        self.depth = depth;
        self
    }

    pub fn colorize_line_number(&self, num: usize) -> String {
        if !self.enabled {
            return format!("{num:>6}\t");
        }
        self.colors.line_number
            .paint(format!("{num:>6}\t"))
            .to_string()
    }

    pub fn colorize_end_marker(&self) -> String {
        if !self.enabled {
            return String::from("$");
        }
        self.colors.end_marker.paint("$").to_string()
    }

    pub fn colorize_tab_marker(&self) -> String {
        if !self.enabled {
            return String::from("^I");
        }
        self.colors.tab_marker.paint("^I").to_string()
    }

    pub fn colorize_nonprint(&self, ch: &str) -> String {
        if !self.enabled {
            return ch.to_string();
        }
        self.colors.nonprint.paint(ch).to_string()
    }

    pub fn colorize_header(&self, filename: &str) -> String {
        if !self.enabled {
            return format!("==> {filename} <==");
        }
        self.colors
            .header
            .paint(format!("==> {filename} <=="))
            .to_string()
    }

    pub fn colorize_error(&self, msg: &str) -> String {
        if !self.enabled {
            return format!("xcat: {msg}");
        }
        self.colors.error.paint(format!("xcat: {msg}")).to_string()
    }

    pub fn colorize_separator(&self) -> String {
        if !self.enabled {
            return String::from("────────────────────────────────────────");
        }
        self.colors
            .separator
            .paint("────────────────────────────────────────")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colorizer_disabled() {
        let c = Colorizer::new(false);
        assert_eq!(c.colorize_line_number(1), "     1\t");
        assert_eq!(c.colorize_end_marker(), "$");
        assert_eq!(c.colorize_tab_marker(), "^I");
        assert_eq!(c.colorize_nonprint("^A"), "^A");
        assert_eq!(c.colorize_header("test.txt"), "==> test.txt <==");
    }

    #[test]
    fn test_colorizer_enabled() {
        let c = Colorizer::new(true);
        let line_num = c.colorize_line_number(42);
        assert!(line_num.contains("42"));
        let end = c.colorize_end_marker();
        assert!(end.contains("$"));
        let tab = c.colorize_tab_marker();
        assert!(tab.contains("^I"));
    }

    #[test]
    fn test_colorize_line_number_format() {
        let c = Colorizer::new(false);
        assert_eq!(c.colorize_line_number(1), "     1\t");
        assert_eq!(c.colorize_line_number(42), "    42\t");
        assert_eq!(c.colorize_line_number(123456), "123456\t");
    }

    #[test]
    fn test_colorize_header() {
        let c = Colorizer::new(false);
        assert_eq!(c.colorize_header("foo.txt"), "==> foo.txt <==");
    }

    #[test]
    fn test_colorizer_clone() {
        let c1 = Colorizer::new(true);
        let c2 = c1.clone();
        assert_eq!(c1.enabled, c2.enabled);
    }

    #[test]
    fn test_colorizer_debug() {
        let c = Colorizer::new(true);
        let debug_str = format!("{c:?}");
        assert!(debug_str.contains("Colorizer"));
    }

    #[test]
    fn test_theme_registry_has_default() {
        let reg = ThemeRegistry::new();
        assert!(reg.get("default").is_some());
        assert_eq!(reg.default_theme().name(), "default");
    }

    #[test]
    fn test_theme_registry_all_themes() {
        let reg = ThemeRegistry::new();
        let themes = reg.available_themes();
        assert!(themes.contains(&"monokai"));
        assert!(themes.contains(&"solarized"));
        assert!(themes.contains(&"github"));
        assert!(themes.contains(&"nord"));
        assert!(themes.contains(&"dracula"));
        assert!(themes.contains(&"gruvbox"));
        assert!(themes.contains(&"onedark"));
        assert!(themes.contains(&"tokyonight"));
        assert!(themes.contains(&"catppuccin"));
    }

    #[test]
    fn test_monokai_theme() {
        let theme = MonokaiTheme;
        assert_eq!(theme.name(), "monokai");
        let colors = theme.colors();
        assert_eq!(colors.line_number, Style::new().fg(Color::Fixed(141)).bold());
    }

    #[test]
    fn test_nord_theme() {
        let theme = NordTheme;
        assert_eq!(theme.name(), "nord");
        let colors = theme.colors();
        assert_eq!(colors.line_number, Style::new().fg(Color::Fixed(67)).bold());
    }

    #[test]
    fn test_dracula_theme() {
        let theme = DraculaTheme;
        assert_eq!(theme.name(), "dracula");
        let colors = theme.colors();
        assert_eq!(colors.line_number, Style::new().fg(Color::Fixed(177)).bold());
    }

    #[test]
    fn test_gruvbox_theme() {
        let theme = GruvboxTheme;
        assert_eq!(theme.name(), "gruvbox");
        let colors = theme.colors();
        assert_eq!(colors.line_number, Style::new().fg(Color::Fixed(109)).bold());
    }

    #[test]
    fn test_solarized_theme() {
        let theme = SolarizedTheme;
        assert_eq!(theme.name(), "solarized");
        let colors = theme.colors();
        assert_eq!(colors.line_number, Style::new().fg(Color::Fixed(33)).bold());
    }

    #[test]
    fn test_github_theme() {
        let theme = GithubTheme;
        assert_eq!(theme.name(), "github");
        let colors = theme.colors();
        assert_eq!(colors.line_number, Style::new().fg(Color::Fixed(103)).bold());
    }

    #[test]
    fn test_onedark_theme() {
        let theme = OneDarkTheme;
        assert_eq!(theme.name(), "onedark");
        let colors = theme.colors();
        assert_eq!(colors.line_number, Style::new().fg(Color::Fixed(67)).bold());
    }

    #[test]
    fn test_tokyonight_theme() {
        let theme = TokyoNightTheme;
        assert_eq!(theme.name(), "tokyonight");
        let colors = theme.colors();
        assert_eq!(colors.line_number, Style::new().fg(Color::Fixed(75)).bold());
    }

    #[test]
    fn test_catppuccin_theme() {
        let theme = CatppuccinTheme;
        assert_eq!(theme.name(), "catppuccin");
        let colors = theme.colors();
        assert_eq!(colors.line_number, Style::new().fg(Color::Fixed(153)).bold());
    }

    #[test]
    fn test_theme_with_colorizer() {
        let theme = MonokaiTheme;
        let c = Colorizer::with_theme(true, &theme);
        assert!(c.enabled);
        let line_num = c.colorize_line_number(1);
        assert!(line_num.contains("1"));
    }

    #[test]
    fn test_colorize_separator() {
        let c = Colorizer::new(false);
        assert!(c.colorize_separator().contains("──"));
    }

    #[test]
    fn test_color_depth_default() {
        let depth = ColorDepth::default();
        assert_eq!(depth, ColorDepth::Extended256);
    }

    #[test]
    fn test_colorizer_with_depth() {
        let c = Colorizer::new(true).with_depth(ColorDepth::TrueColor);
        assert_eq!(c.depth, ColorDepth::TrueColor);
    }

    #[test]
    fn test_theme_registry_register_custom() {
        #[derive(Debug, Clone)]
        struct CustomTheme;
        impl ColorTheme for CustomTheme {
            fn name(&self) -> &str { "custom" }
            fn colors(&self) -> ThemeColors {
                ThemeColors {
                    line_number: Style::new().fg(Color::White),
                    end_marker: Style::new().fg(Color::White),
                    tab_marker: Style::new().fg(Color::White),
                    nonprint: Style::new().fg(Color::White),
                    error: Style::new().fg(Color::White),
                    header: Style::new().fg(Color::White),
                    separator: Style::new().fg(Color::White),
                }
            }
        }

        let mut reg = ThemeRegistry::new();
        reg.register(Box::new(CustomTheme));
        assert!(reg.get("custom").is_some());
    }
}
