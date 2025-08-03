use serde::{Deserialize, Serialize};
use std::{
    env::var,
    path::{Path, PathBuf},
};
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct MpdConfig {
    pub host: String,
    pub port: u16,
    pub password: String,
}
impl MpdConfig {
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
struct RawPathConfig {
    #[serde(default)]
    music_dir: Option<PathBuf>,
    #[serde(default)]
    lyrics_dir: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(from = "RawPathConfig")]
pub struct PathConfig {
    pub music_dir: PathBuf,
    pub lyrics_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct LyricStyleConfig {
    pub color: String,
    pub bold: bool,
    pub current: LyricStateOverride,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct LyricStateOverride {
    pub color: String,
    pub bold: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(default)]
pub struct Config {
    pub mpd: MpdConfig,
    pub paths: PathConfig,
    pub lyric_style: LyricStyleConfig,
}

impl Default for MpdConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 6600,
            password: "".into(),
        }
    }
}

impl From<RawPathConfig> for PathConfig {
    fn from(raw: RawPathConfig) -> Self {
        let music_dir = expand_tilde(raw.music_dir.unwrap_or_else(|| PathBuf::from("~/Music")));
        let lyrics_dir = expand_tilde(raw.lyrics_dir.unwrap_or_else(|| music_dir.clone()));
        Self {
            music_dir,
            lyrics_dir,
        }
    }
}

impl Default for PathConfig {
    fn default() -> Self {
        Self {
            music_dir: PathBuf::from("~/Music"),
            lyrics_dir: PathBuf::from("~/Music"),
        }
    }
}

impl Default for LyricStyleConfig {
    fn default() -> Self {
        Self {
            color: "#AAAAAA".into(),
            bold: false,
            current: LyricStateOverride::default(),
        }
    }
}

impl Default for LyricStateOverride {
    fn default() -> Self {
        Self {
            color: "#00FF7F".into(),
            bold: true,
        }
    }
}

fn expand_tilde<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    if path.starts_with("~") {
        if let Ok(home) = var("HOME") {
            let home = PathBuf::from(home);
            if path == Path::new("~") {
                home
            } else {
                home.join(path.strip_prefix("~").unwrap())
            }
        } else {
            path.to_path_buf()
        }
    } else {
        path.to_path_buf()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mpd_config_default() {
        let cfg = MpdConfig::default();
        assert_eq!(cfg.host, "127.0.0.1");
        assert_eq!(cfg.port, 6600);
        assert_eq!(cfg.password, "");
    }

    #[test]
    fn test_path_config_default() {
        let cfg = PathConfig::default();
        assert_eq!(cfg.music_dir, PathBuf::from("~/Music"));
        assert_eq!(cfg.lyrics_dir, PathBuf::from("~/Music"));
    }

    #[test]
    fn test_path_config_from_raw() {
        let raw = RawPathConfig {
            music_dir: Some(PathBuf::from("/custom/music")),
            lyrics_dir: None,
        };
        let cfg = PathConfig::from(raw);
        assert_eq!(cfg.music_dir, PathBuf::from("/custom/music"));
        assert_eq!(cfg.lyrics_dir, PathBuf::from("/custom/music"));
    }

    #[test]
    fn test_lyric_style_default() {
        let cfg = LyricStyleConfig::default();
        assert_eq!(cfg.color, "#FFFFFF");
        assert!(!cfg.bold);

        let current = cfg.current;
        assert_eq!(current.color, "#FFFFFF");
        assert!(current.bold);
    }

    #[test]
    fn test_config_default() {
        let cfg = Config::default();
        assert_eq!(cfg.mpd.host, "127.0.0.1");
        assert_eq!(cfg.paths.music_dir, PathBuf::from("~/Music"));
    }
}
