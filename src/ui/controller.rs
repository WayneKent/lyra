use anyhow::{Context, Ok, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use mpd::State as MpdState;
use std::time::{Duration, Instant};

use crate::lyrics::parser::find_lyrics_for_track;
use crate::ui::app::App;

pub struct Controller {
    pub app: App,
    last_mpd_check: Instant,
    mpd_poll_interval: Duration,
}

impl Controller {
    pub fn new(app: App) -> Self {
        Self {
            app,
            last_mpd_check: Instant::now(),
            mpd_poll_interval: Duration::from_millis(500),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.handle_user_input()?;

        if self.last_mpd_check.elapsed() >= self.mpd_poll_interval {
            self.handle_mpd_state()?;
            self.last_mpd_check = Instant::now();
        }

        if self.app.play_state == MpdState::Play {
            self.app.update_scroll_offset()?;
        }

        Ok(())
    }

    pub fn handle_user_input(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(event) = event::read()? {
                self.handle_key_event(event);
            }
        }
        Ok(())
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {
        if let KeyCode::Char('q') = event.code {
            self.app.should_quit = true;
        }
    }

    pub fn handle_mpd_state(&mut self) -> Result<()> {
        let status = self.app.mpd_client.status().context("获取MPD状态失败")?;
        let current_song = self
            .app
            .mpd_client
            .currentsong()
            .context("获取当前歌曲失败")?;

        self.app.play_state = status.state;

        let new_song_file = current_song.as_ref().map(|s| s.file.clone());
        if self.app.update_current_song_name(new_song_file.clone()) {
            self.app.current_song_title = current_song.and_then(|s| s.title);
            self.app.current_lyrics = new_song_file
                .as_ref()
                .and_then(|file| find_lyrics_for_track(&self.app.config.paths, file).ok())
                .flatten();
        }

        Ok(())
    }
}
