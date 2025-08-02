use mpd::{Client, State as MpdState};
use std::time::Duration;

use crate::model::{config::Config, lyrics::Lyrics};

#[derive(Debug)]
pub struct App {
    pub config: Config,
    pub mpd_client: Client,
    pub current_lyrics: Option<Lyrics>,
    pub current_song_file_name: Option<String>,
    pub current_song_title: Option<String>,
    pub scroll_offset: usize,
    pub play_state: MpdState,
    pub should_quit: bool,
}

impl App {
    pub fn new(
        config: Config,
        mpd_client: Client,
        initial_state: MpdState,
        current_lyrics: Option<Lyrics>,
        current_song_file_name: Option<String>,
        current_song_title: Option<String>,
    ) -> Self {
        Self {
            config,
            mpd_client,
            current_lyrics,
            current_song_file_name,
            current_song_title,
            scroll_offset: 0,
            play_state: initial_state,
            should_quit: false,
        }
    }

    pub fn update_current_song_name(&mut self, new_song_file_name: Option<String>) -> bool {
        let changed = self.current_song_file_name != new_song_file_name;
        if changed {
            self.current_song_file_name = new_song_file_name;
            self.scroll_offset = 0;
        }
        changed
    }

    pub fn update_scroll_offset(&mut self) -> Result<(), mpd::error::Error> {
        if self.play_state != MpdState::Play || self.current_lyrics.is_none() {
            return Ok(());
        }

        let status = self.mpd_client.status()?;
        let current_play_time = status.elapsed.unwrap_or(Duration::ZERO);

        let lyrics = self.current_lyrics.as_ref().unwrap();

        let mut new_offset = self.scroll_offset;
        for (line_idx, line) in lyrics.lines.iter().enumerate() {
            if line.timestamp <= current_play_time {
                new_offset = line_idx;
            } else {
                break;
            }
        }

        self.scroll_offset = new_offset;
        Ok(())
    }
}
