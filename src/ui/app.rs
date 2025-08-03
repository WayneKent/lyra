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

        let target_offset = lyrics
            .lines
            .iter()
            .enumerate()
            .find(|(_, line)| line.timestamp > current_play_time)
            .map_or(lyrics.lines.len().saturating_sub(1), |(i, _)| {
                i.saturating_sub(1)
            });

        let diff = match self.scroll_offset < target_offset {
            true => target_offset - self.scroll_offset,
            false => self.scroll_offset - target_offset,
        };

        let step = match diff {
            0 => 0,
            1..=2 => 1,                            // 近距离小步滚动
            3..=5 => 2,                            // 中等距离加速
            _ => (diff as f32 * 0.3) as usize + 1, // 远距离大幅跳转（最多30%+1行）
        };

        if self.scroll_offset < target_offset {
            self.scroll_offset += step;
            if self.scroll_offset > target_offset {
                self.scroll_offset = target_offset;
            }
        } else if self.scroll_offset > target_offset {
            self.scroll_offset -= step;
            if self.scroll_offset < target_offset {
                self.scroll_offset = target_offset;
            }
        }
        Ok(())
    }
}
