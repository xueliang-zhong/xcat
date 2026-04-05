use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{XcatError, XcatResult};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Config {
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub color: ColorConfig,
    #[serde(default)]
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DisplayConfig {
    #[serde(default = "default_true")]
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
    #[serde(default = "default_buffer_size")]
    pub buffer_size: usize,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ColorConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_true")]
    pub line_number_color: bool,
    #[serde(default = "default_true")]
    pub end_marker_color: bool,
    #[serde(default = "default_true")]
    pub tab_marker_color: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PerformanceConfig {
    #[serde(default = "default_true")]
    pub use_mmap: bool,
    #[serde(default = "default_buffer_size")]
    pub buffer_size: usize,
    #[serde(default)]
    pub parallel: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            number: false,
            number_nonblank: false,
            show_ends: false,
            squeeze_blank: false,
            show_tabs: false,
            show_nonprinting: false,
            buffer_size: default_buffer_size(),
        }
    }
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            theme: String::from("default"),
            line_number_color: true,
            end_marker_color: true,
            tab_marker_color: true,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            use_mmap: true,
            buffer_size: default_buffer_size(),
            parallel: false,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display: DisplayConfig::default(),
            color: ColorConfig::default(),
            performance: PerformanceConfig::default(),
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
    64 * 1024 // 64KB
}

impl Config {
    pub fn load() -> XcatResult<Self> {
        let path = Self::config_path();
        if path.exists() {
            Self::from_file(&path)
        } else {
            Ok(Config::default())
        }
    }

    pub fn from_file(path: &Path) -> XcatResult<Self> {
        let content = fs::read_to_string(path).map_err(|e| {
            XcatError::Config(format!("Failed to read config file {}: {}", path.display(), e))
        })?;
        let config: Config = toml::from_str(&content).map_err(|e| {
            XcatError::Config(format!("Failed to parse config file {}: {}", path.display(), e))
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
    fn test_default_config() {
        let config = Config::default();
        assert!(config.color.enabled);
        assert_eq!(config.color.theme, "default");
        assert!(config.performance.use_mmap);
        assert_eq!(config.performance.buffer_size, 64 * 1024);
    }

    #[test]
    fn test_config_from_valid_toml() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
[color]
enabled = false
theme = "monokai"

[performance]
use_mmap = false
"#
        )
        .unwrap();

        let config = Config::from_file(file.path()).unwrap();
        assert!(!config.color.enabled);
        assert_eq!(config.color.theme, "monokai");
        assert!(!config.performance.use_mmap);
    }

    #[test]
    fn test_config_from_invalid_toml() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "this is not valid toml").unwrap();

        let result = Config::from_file(file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_config_from_nonexistent_file() {
        let result = Config::from_file(Path::new("/nonexistent/path/config.toml"));
        assert!(result.is_err());
    }

    #[test]
    fn test_config_partial_toml() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "[color]\nenabled = false").unwrap();

        let config = Config::from_file(file.path()).unwrap();
        assert!(!config.color.enabled);
        assert_eq!(config.color.theme, "default"); // default value
    }

    #[test]
    fn test_display_config_defaults() {
        let config = Config::default();
        assert!(!config.display.number);
        assert!(!config.display.number_nonblank);
        assert!(!config.display.show_ends);
        assert!(!config.display.squeeze_blank);
        assert!(!config.display.show_tabs);
        assert!(!config.display.show_nonprinting);
    }

    #[test]
    fn test_config_equality() {
        let a = Config::default();
        let b = Config::default();
        assert_eq!(a, b);
    }

    #[test]
    fn test_config_clone() {
        let config = Config::default();
        let cloned = config.clone();
        assert_eq!(config, cloned);
    }
}
