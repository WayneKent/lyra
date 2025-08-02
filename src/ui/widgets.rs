use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Paragraph, Widget, Wrap},
};

use crate::model::{
    config::LyricStyleConfig,
    lyrics::{LyricLine, Lyrics},
};

#[derive(Debug)]

pub struct LyricsWidget<'a> {
    lyrics: &'a Option<Lyrics>,
    scroll_offset: usize,
    style: &'a LyricStyleConfig,
    terminal_height: u16,
}

impl<'a> LyricsWidget<'a> {
    pub fn new(
        lyrics: &'a Option<Lyrics>,
        scroll_offset: usize,
        style: &'a LyricStyleConfig,
        terminal_height: u16,
    ) -> Self {
        Self {
            lyrics,
            scroll_offset,
            style,
            terminal_height,
        }
    }
}

impl<'a> Widget for LyricsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let lyrics_lines = match &self.lyrics {
            Some(lyrics) => &lyrics.lines,
            None => {
                let no_lyrics_text = "暂无歌词".to_string();
                Paragraph::new(no_lyrics_text)
                    .style(
                        Style::default()
                            .fg(self.parse_color(&self.style.color))
                            .add_modifier(if self.style.bold {
                                Modifier::BOLD
                            } else {
                                Modifier::empty()
                            }),
                    )
                    .alignment(Alignment::Center)
                    .render(area, buf);
                return;
            }
        };

        let default_style = Style::default()
            .fg(self.parse_color(&self.style.color))
            .add_modifier(if self.style.bold {
                Modifier::BOLD
            } else {
                Modifier::empty()
            });

        let current_line_style = Style::default()
            .fg(self.parse_color(&self.style.current.color))
            .add_modifier(if self.style.current.bold {
                Modifier::BOLD
            } else {
                Modifier::empty()
            });

        let visible_lines = self.calculate_visible_lines(lyrics_lines);

        let text = self.build_lyrics_text(
            lyrics_lines,
            visible_lines,
            default_style,
            current_line_style,
        );

        Paragraph::new(text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}

impl<'a> LyricsWidget<'a> {
    fn parse_color(&self, color_str: &str) -> Color {
        if let Some(hex) = color_str.strip_prefix('#') {
            if hex.len() == 6 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&hex[0..2], 16),
                    u8::from_str_radix(&hex[2..4], 16),
                    u8::from_str_radix(&hex[4..6], 16),
                ) {
                    return Color::Rgb(r, g, b);
                }
            }
        }
        Color::White
    }

    fn calculate_visible_lines(&self, all_lines: &[LyricLine]) -> std::ops::Range<usize> {
        let total_lines = all_lines.len();
        let max_visible_lines = self.terminal_height as usize;

        let ideal_start = self.scroll_offset.saturating_sub(max_visible_lines / 2);

        let start = if total_lines <= max_visible_lines {
            0
        } else {
            let max_possible_start = total_lines.saturating_sub(max_visible_lines);
            ideal_start.min(max_possible_start)
        };

        let end = std::cmp::min(start + max_visible_lines, total_lines);
        start..end
    }

    fn build_lyrics_text(
        &self,
        all_lines: &[LyricLine],
        visible_range: std::ops::Range<usize>,
        default_style: Style,
        current_style: Style,
    ) -> Text {
        let mut lines = Vec::new();

        for (line_idx, line) in all_lines.iter().enumerate() {
            if visible_range.contains(&line_idx) {
                let style = if line_idx == self.scroll_offset {
                    current_style
                } else {
                    default_style
                };

                let span = Span::styled(line.text.clone(), style);
                let lyric_line = Line::from(span);

                lines.push(lyric_line);
            }
        }

        Text::from(lines)
    }
}
