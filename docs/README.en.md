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

## 🖼️ Preview

<p align="center">
  <img src="images/lyra.png" alt="Lyra 预览" width="60%">
</p>

---

## 📥 Installation Methods

### Install from crates.io (Recommended)

1. **Prerequisites**  
   Ensure you have Rust installed (recommended to use the [official installation guide](https://www.rust-lang.org/tools/install))

2. **Direct Installation**  
   Run the following command to install the latest stable version from crates.io:
    ```bash
     cargo install lyra-rs
    ```

### Build from Source (Developer Option)

1.  **Clone Repository**

    ```bash
    git clone https://github.com/WayneKent/lyra.git
    cd lyra
    ```

2.  **Build and Install**  
    Execute the following command to install:

    ```bash
    cargo install .
    ```

---

## ⚙️ Configuration

Default configuration will be generated on first run. Configuration file path:  
`~/.config/lyra/config.toml`

```toml
[mpd]
host = "127.0.0.1"     # MPD server address
port = 6600            # MPD server port
password = ""          # MPD authentication password (leave empty if no password)

[paths]
music_dir = "~/Music"  # Music files directory
lyrics_dir = "~/Music" # Lyrics files directory (defaults to same as music directory)

[lyric_style]
color = "#AAAAAA"      # Normal lyrics color
bold = false           # Whether to bold normal lyrics

[lyric_style.current]
color = "#00FF7F"      # Currently playing lyric color
bold = true            # Whether to bold currently playing lyric
```

---

## 🚀 Usage

1. Ensure MPD service is running and playing music
2. Run in terminal:
    ```bash
    lyra-rs
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
