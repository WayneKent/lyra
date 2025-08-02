use std::io;

use anyhow::Context;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use mpd::Client;
use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::{config::loader::load_config, lyrics::parser::find_lyrics_for_track, ui::app::App};

mod config;
mod lyrics;
mod model;
mod ui;
fn main() -> anyhow::Result<()> {
    let config = load_config();

    let mut mpd_client =
        Client::connect(config.mpd.get_address()).with_context(|| "连接MPD服务失败")?;

    let mpd_password = &config.mpd.password;
    if !mpd_password.is_empty() {
        mpd_client
            .login(mpd_password)
            .with_context(|| "MPD认证失败")?;
    }

    let status = mpd_client.status().with_context(|| "获取MPD状态失败")?;
    let song = mpd_client
        .currentsong()
        .with_context(|| "获取当前播放歌曲失败")?;

    let (current_song_file_name, current_song_title) = match song {
        Some(s) => (Some(s.file), s.title),
        None => (None, None),
    };
    let current_lyrics = current_song_file_name
        .as_ref()
        .and_then(|s| find_lyrics_for_track(&config.paths, s).ok())
        .flatten();

    let app = App::new(
        config,
        mpd_client,
        status.state,
        current_lyrics,
        current_song_file_name,
        current_song_title,
    );

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut controller = crate::ui::controller::Controller::new(app);
    loop {
        if controller.app.should_quit {
            break;
        }

        terminal.draw(|frame| {
            crate::ui::renderer::render(&controller.app, frame);
        })?;

        controller.run()?;

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
