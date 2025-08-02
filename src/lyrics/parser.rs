use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::Ok;
use lofty::{file::TaggedFileExt, read_from_path};
use walkdir::WalkDir;

use crate::model::{
    config::PathConfig,
    lyrics::{LyricLine, Lyrics},
};

fn parse_lrc_timestamp(s: &str) -> Option<Duration> {
    let s = s.trim_start_matches('[');
    let mut parts = s.split(':');
    let minutes = parts.next()?.parse::<u64>().ok()?;
    let seconds = parts.next()?.parse::<f64>().ok()?;

    Some(Duration::from_secs(minutes * 60) + Duration::from_millis((seconds * 1000.0) as u64))
}

fn from_audio<P: AsRef<Path>>(path: P) -> anyhow::Result<Option<Lyrics>> {
    let tagged_file = read_from_path(path)?;

    if let Some(tag) = tagged_file.primary_tag() {
        if let Some(lyrics) = tag.get_string(&lofty::tag::ItemKey::Lyrics) {
            let mut lines = Vec::new();

            for line in lyrics.lines() {
                let mut parts = line.splitn(2, ']');
                if let (Some(time_part), Some(text)) = (parts.next(), parts.next()) {
                    if let Some(timestamp) = parse_lrc_timestamp(time_part) {
                        lines.push(LyricLine {
                            timestamp,
                            text: text.trim().to_string(),
                        });
                    }
                }
            }
            if !lines.is_empty() {
                return Ok(Some(Lyrics { lines }));
            }
        }
    }

    Ok(None)
}

fn from_lrc<P: AsRef<Path>>(path: P) -> anyhow::Result<Lyrics> {
    let content = fs::read_to_string(&path)?;

    let mut lines = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.splitn(2, ']');

        if let (Some(time_part), Some(text)) = (parts.next(), parts.next()) {
            if let Some(timestamp) = parse_lrc_timestamp(time_part) {
                lines.push(LyricLine {
                    timestamp,
                    text: text.trim().to_string(),
                });
            }
        }
    }

    Ok(Lyrics { lines })
}

fn find_lrc_file(lyrics_dir: &Path, target_name: &str) -> Option<PathBuf> {
    let target_filename = format!("{target_name}.lrc");

    WalkDir::new(lyrics_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .find(|entry| entry.file_name().to_str() == Some(target_filename.as_str()))
        .map(|entry| entry.into_path())
}

fn find_audio_file(music_dir: &Path, target_name: &str) -> Option<PathBuf> {
    WalkDir::new(music_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .find(|entry| entry.file_name().to_str() == Some(target_name))
        .map(|entry| entry.into_path())
}

pub fn find_lyrics_for_track(
    config: &PathConfig,
    track_name: &str,
) -> anyhow::Result<Option<Lyrics>> {
    let base_name = Path::new(track_name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(track_name);

    let lyrics_dir = &config.lyrics_dir;

    let lrc_file = find_lrc_file(lyrics_dir, base_name);

    if let Some(file) = lrc_file {
        let lyrics = from_lrc(file)?;
        return Ok(Some(lyrics));
    }

    let music_dir = &config.music_dir;

    let audio_file = find_audio_file(music_dir, track_name);
    if let Some(file) = audio_file {
        let lyrics = from_audio(file)?;
        return Ok(lyrics);
    }

    Ok(None)
}

#[cfg(test)]
mod tests {

    use std::path::Path;

    use crate::lyrics::parser::{find_audio_file, find_lrc_file, from_audio, from_lrc};

    #[test]
    fn test_audio_lyrics() {
        let result = from_audio("test_data/1个球 - 大雨还在下.mp3");
        assert!(result.is_ok(), "应该成功解析MP3");
        assert!(result.unwrap().is_some(), "应该包含歌词");
    }

    #[test]
    fn get_lyrics_by_lrc() {
        let result = from_lrc("test_data/完美借口_歌词.lrc");
        assert!(result.is_ok(), "应该成功解析LRC文件");
        let lyrics = result.unwrap();
        assert!(!lyrics.lines.is_empty(), "歌词不应为空");
    }

    #[test]
    fn test_find_lrc_file() {
        let path = Path::new("test_data");
        let result = find_lrc_file(path, "完美借口_歌词");
        println!("{result:#?}");
        assert!(result.is_some());
    }

    #[test]
    fn test_find_audio_file() {
        let path = Path::new("test_data");
        let result = find_audio_file(path, "1个球 - 大雨还在下.mp3");
        println!("{result:#?}");
        assert!(result.is_some());
    }
}
