use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders},
};

use crate::ui::{app::App, widgets::LyricsWidget};

pub fn render(app: &App, frame: &mut Frame) {
    let full_screen = frame.area();

    let file_name = app
        .current_song_file_name
        .as_deref()
        .unwrap_or("Lyra 歌词播放器")
        .to_string();

    let song_title = app
        .current_song_title
        .as_deref()
        .unwrap_or(&file_name)
        .to_string();

    let border_block = Block::default()
        .title(Span::styled(
            song_title,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(Color::Cyan));

    let inner_area = border_block.inner(full_screen);

    frame.render_widget(border_block, full_screen);

    let terminal_height = inner_area.height;

    let lyrics_widget = LyricsWidget::new(
        &app.current_lyrics,
        app.scroll_offset,
        &app.config.lyric_style,
        terminal_height,
    );
    frame.render_widget(lyrics_widget, inner_area);
}
