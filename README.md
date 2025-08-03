# Lyra - 终端歌词播放器

[English](docs/README.en.md) | [中文](#)

_一个基于终端的MPD歌词显示工具，实时同步滚动显示歌词。支持音频文件内嵌歌词和外部LRC文件。_

---

## ✨ 功能特点

- **音乐服务器连接**
    - 实时连接 MPD 服务器
    - 自动同步播放状态

- **歌词支持**
    - 自动加载 LRC 格式歌词文件
    - 解析音频文件内嵌元数据歌词
    - 双模式智能匹配歌词源

- **播放体验**
    - 实时滚动显示歌词
    - 当前行高亮显示
    - 平滑的滚动效果

- **个性化设置**
    - 可配置 MPD 连接参数
    - 自定义歌词文件路径
    - 界面颜色和样式调整

- **终端界面**
    - 简洁优雅的设计
    - 低资源占用
    - 响应迅速

---

## 🖼️ 界面预览

<p align="center">
  <img src="docs/images/lyra.png" alt="Lyra 预览" width="60%">
</p>

---

## 📥 安装方法

### 从 crates.io 安装（推荐）

1. **环境准备**  
   确保已安装 Rust 开发环境（推荐使用 [官方安装指南](https://www.rust-lang.org/tools/install)）

2. **直接安装**  
   执行以下命令从 crates.io 安装最新稳定版：
    ```bash
     cargo install lyra
    ```

### 从源码编译（开发者选项）

1. **克隆仓库**

    ```bash
    git clone https://github.com/WayneKent/lyra.git
    cd lyra
    ```

2. **编译安装**  
   执行以下命令进行安装：

    ```bash
    cargo install .
    ```

---

## ⚙️ 配置说明

首次运行时会自动生成默认配置，配置文件路径为：  
`~/.config/lyra/config.toml`

```toml
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

## 🚀 使用方法

1. 确保 MPD 服务已启动并正在播放音乐
2. 在终端中运行：

```bash
lyra
```

3. 操作说明:

- 按 `q` 键退出程序

---

## 📝 歌词文件支持

Lyra 会按以下优先级查找歌词：

1. 在配置的 `lyrics_dir` 目录中查找匹配的 `.lrc` 文件

- 文件名需与歌曲名一致
- 支持 UTF-8 编码

2. 从音频文件元数据中提取内嵌歌词

- 支持 MP3、FLAC 等常见格式

---

## 📚 依赖项目

- [ratatui](https://github.com/tui-rs-revival/ratatui) - 终端用户界面库
- [rust-mpd](https://github.com/kstep/rust-mpd/) - MPD 客户端实现
- [crossterm](https://github.com/crossterm-rs/crossterm) - 跨平台终端处理
- [lofty](https://github.com/Serial-ATA/lofty-rs) - 音频元数据解析

---

## 📜 许可证

本项目采用 [MIT License](LICENSE) 开源协议
