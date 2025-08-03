# Lyra - Terminal Lyrics Player

[中文](../README.md) | [English](#)

_A terminal-based lyrics display tool for MPD that syncs and scrolls lyrics in real-time. Supports embedded lyrics in audio files and external LRC files._

---

## ✨ Features

- **MPD Server Connection**
    - Real-time connection to MPD server
    - Automatic playback state synchronization

- **Lyrics Support**
    - Auto-loading LRC format lyrics files
    - Extracting embedded lyrics from audio metadata
    - Dual-mode intelligent lyrics matching

- **Playback Experience**
    - Real-time scrolling lyrics display
    - Current line highlighting
    - Smooth scrolling effect

- **Customization**
    - Configurable MPD connection parameters
    - Custom lyrics directory path
    - Adjustable interface colors and styles

- **Terminal Interface**
    - Clean and elegant design
    - Low resource consumption
    - Responsive performance

---

## 📥 Installation

### Build from Source

1. **Prerequisites**  
   Ensure Rust development environment is installed (recommended [official guide](https://www.rust-lang.org/tools/install))
    ```bash
    git clone git clone https://github.com/WayneKent/lyra.git
    cd lyra
    ```
2. **Build & Install**  
    Execute the following command:
   `bash
cargo install --path .
`

---

## ⚙️ Configuration

Default configuration will be generated on first run. Configuration file path:  
`~/.config/lyra/config.toml`

```toml
# ~/.config/lyra/config.toml
[mpd]
host = "127.0.0.1"     # MPD 服务器地址
port = 6600            # MPD 服务器端口
password = ""          # MPD 认证密码(如无密码可留空)

[paths]
music_dir = "~/Music"  # 音乐文件目录
lyrics_dir = "~/Music" # 歌词文件目录(默认与音乐目录相同)

[lyric_style]
color = "#AAAAAA"      # 普通歌词颜色
bold = false           # 普通歌词是否加粗

[lyric_style.current]
color = "#00FF7F"      # 当前播放歌词颜色
bold = true            # 当前播放歌词是否加粗
```

---

## 🚀 Usage

1. Ensure MPD service is running and playing music
2. Run in terminal:
    ```bash
    lyra
    ```
3. **Key Controls**:

- `q` - Quit application

---

## 📝 Lyrics File Support

Lyra searches for lyrics in following priority:

1. Matching `.lrc` files in configured `lyrics_dir`

- Filename must match song title
- Supports UTF-8 encoding

2. Embedded lyrics in audio file metadata

- Supports common formats (MP3, FLAC, etc.)

---

## 📚 Dependencies

- [ratatui](https://github.com/tui-rs-revival/ratatui) - Terminal UI library
- [rust-mpd](https://github.com/kstep/rust-mpd/) - MPD client implementation
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal handling
- [lofty](https://github.com/Serial-ATA/lofty-rs) - Audio metadata parsing

---

## 📜 License

This project is licensed under the [MIT License](LICENSE)
