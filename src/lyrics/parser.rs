use std::{fs, io::Error, path::Path, time::Duration};

use lofty::{error::LoftyError, file::TaggedFileExt, read_from_path};

use crate::model::lyrics::{LyricLine, Lyrics};

fn parse_lrc_timestamp(s: &str) -> Option<Duration> {
    let s = s.trim_start_matches('[');
    let mut parts = s.split(':');
    let minutes = parts.next()?.parse::<u64>().ok()?;
    let seconds = parts.next()?.parse::<f64>().ok()?;

    Some(Duration::from_secs(minutes * 60) + Duration::from_millis((seconds * 1000.0) as u64))
}

pub fn from_audio<P: AsRef<Path>>(path: P) -> Result<Option<Lyrics>, LoftyError> {
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

pub fn from_lrc<P: AsRef<Path>>(path: P) -> Result<Lyrics, Error> {
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

#[cfg(test)]
mod tests {

    use crate::lyrics::parser::{from_audio, from_lrc};

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
}
