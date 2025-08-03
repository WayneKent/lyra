use std::fs;

use crate::config::paths;
use crate::model::config::Config;

pub fn load_config() -> Config {
    let path = paths::get_config_path();

    if !path.exists() {
        println!("未找到配置文件， 使用默认配置");
        return Config::default();
    }

    let content = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("读取文件失败: {e}");
            return Config::default();
        }
    };

    match toml::from_str(&content) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("解析配置失败: {e}");
            Config::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::paths::get_config_path;

    use super::*;
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn with_temp_home<F>(test_fn: F)
    where
        F: FnOnce(PathBuf),
    {
        let temp_home = tempdir().expect("无法创建临时目录");
        let original_home = env::var("HOME").ok();

        unsafe { env::set_var("HOME", temp_home.path()) };
        test_fn(temp_home.path().to_path_buf());

        // 恢复原始HOME环境
        match original_home {
            Some(home) => unsafe { env::set_var("HOME", home) },
            None => unsafe { env::remove_var("HOME") },
        }
    }

    #[test]
    fn test_load_config_when_file_not_exists() {
        with_temp_home(|home| {
            let config = load_config();
            assert_eq!(config, Config::default());

            // 验证配置文件路径是否正确
            let expected_path = home.join(".config/lyra/config.toml");
            assert!(!expected_path.exists());
        });
    }

    #[test]
    fn test_load_config_with_invalid_toml() {
        with_temp_home(|home| {
            let config_dir = home.join(".config/lyra");
            fs::create_dir_all(&config_dir).unwrap();

            let config_path = config_dir.join("config.toml");
            let mut file = File::create(&config_path).unwrap();
            writeln!(file, "invalid toml content").unwrap();

            let config = load_config();
            assert_eq!(config, Config::default());
        });
    }

    #[test]
    fn test_load_config_successfully() {
        with_temp_home(|home| {
            let config_dir = home.join(".config/lyra");
            fs::create_dir_all(&config_dir).unwrap();

            let config_path = config_dir.join("config.toml");
            let mut file = File::create(&config_path).unwrap();

            let toml_content = r##"
                    [mpd]
                    host = "localhost"
                    port = 1234
                    password = "secret"

                    [paths]
                    music_dir = "/custom/music"
                    lyrics_dir = "/custom/lyrics"

                    [lyric_style]
                    font_size = 18
                    color = "#FF0000"
                    bold = true

                    [lyric_style.current]
                    font_size = 20
                    color = "#00FF00"
                    bold = false
                "##;

            writeln!(file, "{toml_content}").unwrap();

            let config = load_config();

            assert_eq!(config.mpd.host, "localhost");
            assert_eq!(config.mpd.port, 1234);
            assert_eq!(config.mpd.password, "secret");
            assert_eq!(config.paths.music_dir.to_string_lossy(), "/custom/music");
            assert_eq!(config.paths.lyrics_dir.to_string_lossy(), "/custom/lyrics");
            assert_eq!(config.lyric_style.color, "#FF0000");
            assert!(config.lyric_style.bold);
            assert_eq!(config.lyric_style.current.color, "#00FF00");
            assert!(!config.lyric_style.current.bold);
        });
    }

    #[test]
    fn test_load_config_with_io_error() {
        with_temp_home(|home| {
            let config_dir = home.join(".config/lyra");
            fs::create_dir_all(&config_dir).unwrap();

            let config_path = config_dir.join("config.toml");
            fs::create_dir(&config_path).unwrap();

            let config = load_config();
            assert_eq!(config, Config::default());
        });
    }

    #[test]
    fn test_get_config_path() {
        with_temp_home(|home| {
            let path = get_config_path();
            let expected_path = home.join(".config/lyra/config.toml");
            assert_eq!(path, expected_path);
        });
    }
}
