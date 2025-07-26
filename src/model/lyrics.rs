use std::time::Duration;

#[derive(Debug)]
pub struct Lyrics {
    pub lines: Vec<LyricLine>
}

#[derive(Debug)]
pub struct LyricLine {
    pub timestamp: Duration,
    pub text: String
}
