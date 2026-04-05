use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::ColorMode;
use crate::error::{XcatError, XcatResult};

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct Config {
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub color: ColorConfig,
    #[serde(default)]
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct DisplayConfig {
    #[serde(default)]
    pub number: bool,
    #[serde(default)]
    pub number_nonblank: bool,
    #[serde(default)]
    pub show_ends: bool,
    #[serde(default)]
    pub squeeze_blank: bool,
    #[serde(default)]
    pub show_tabs: bool,
    #[serde(default)]
    pub show_nonprinting: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ColorConfig {
    #[serde(default)]
    pub mode: ColorMode,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_true")]
    pub syntax_highlighting: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PerformanceConfig {
    #[serde(default = "default_true")]
    pub use_mmap: bool,
    #[serde(default = "default_buffer_size")]
    pub buffer_size: usize,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            mode: ColorMode::Auto,
            theme: default_theme(),
            syntax_highlighting: true,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            use_mmap: true,
            buffer_size: default_buffer_size(),
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_theme() -> String {
    String::from("default")
}

fn default_buffer_size() -> usize {
    64 * 1024
}

impl Config {
    pub fn load() -> XcatResult<Self> {
        Self::load_from_path(Self::config_path().as_path())
    }

    pub fn load_from_path(path: &Path) -> XcatResult<Self> {
        if path.exists() {
            Self::from_file(path)
        } else {
            Ok(Self::default())
        }
    }

    pub fn from_file(path: &Path) -> XcatResult<Self> {
        let content = fs::read_to_string(path).map_err(|e| {
            XcatError::Config(format!(
                "failed to read config file {}: {}",
                path.display(),
                e
            ))
        })?;
        let config: Config = toml::from_str(&content).map_err(|e| {
            XcatError::Config(format!(
                "failed to parse config file {}: {}",
                path.display(),
                e
            ))
        })?;
        Ok(config)
    }

    pub fn config_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".xcat");
        path.push("config.toml");
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn default_config_uses_auto_color_and_mmap() {
        let config = Config::default();
        assert_eq!(config.color.mode, ColorMode::Auto);
        assert!(config.color.syntax_highlighting);
        assert!(config.performance.use_mmap);
    }

    #[test]
    fn parses_partial_toml() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
[color]
mode = "always"
theme = "monokai"

[display]
number = true
"#
        )
        .unwrap();

        let config = Config::from_file(file.path()).unwrap();
        assert_eq!(config.color.mode, ColorMode::Always);
        assert_eq!(config.color.theme, "monokai");
        assert!(config.display.number);
        assert!(!config.display.show_ends);
    }

    #[test]
    fn config_path_points_to_home_directory() {
        let path = Config::config_path();
        assert!(path.ends_with(".xcat/config.toml"));
    }
}
