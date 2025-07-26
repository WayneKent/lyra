use std::path::PathBuf;

pub fn get_config_path() -> PathBuf {
    dirs::home_dir()
        .expect("无法获取用户主目录")
        .join(".config/lyra/config.toml")
}
