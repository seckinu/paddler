use std::str::FromStr;

use thiserror::Error;

use crate::segment::{Segment, SegmentError, SegmentMatchResult};

#[derive(Debug, Error)]
pub enum PatternError {
    #[error(transparent)]
    SegmentError(#[from] SegmentError),

    #[error("Invalid pattern format")]
    InvalidPatternFormat,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub segments: Vec<Segment>,
    anchored_start: bool,
    anchored_end: bool,
}

impl FromStr for Pattern {
    type Err = PatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl Pattern {
    pub fn new(pattern_str: &str) -> Result<Self, PatternError> {
        let anchored_start = pattern_str.starts_with('#');
        let anchored_end = pattern_str.ends_with('#');

        let segments = Segment::parse_all(
            &pattern_str[anchored_start as usize..pattern_str.len() - anchored_end as usize],
        )?;

        Ok(Self {
            segments,
            anchored_start,
            anchored_end,
        })
    }

    pub fn matches(&self, word: &str, strict: bool) -> bool {
        let mut word_iter = word.chars().peekable();

        let mut word_segments: Vec<Segment> = Vec::new();

        let Ok(Some(ws)) = Segment::parse(&mut word_iter) else {
            return false;
        };

        word_segments.push(ws);

        let mut start_word_idx = 0;

        let mut word_idx = 0;
        let mut pattern_idx = 0;

        loop {
            if word_segments.get(word_idx).is_none() {
                let ws: Result<Option<Segment>, SegmentError> = Segment::parse(&mut word_iter);

                if let Ok(Some(ws)) = ws {
                    word_segments.push(ws);
                } else if ws.is_err() {
                    return false;
                }
            }

            let word_segment = word_segments.get(word_idx);
            let pattern_segment = self.segments.get(pattern_idx);

            match (word_segment, pattern_segment, strict) {
                (None, None, _) => return true,
                (None, Some(_), _) => return false,
                (Some(_), None, _) => {
                    if self.anchored_end {
                        if self.anchored_start {
                            return false;
                        }

                        start_word_idx += 1;
                        word_idx = start_word_idx;
                        pattern_idx = 0;
                    } else {
                        return true;
                    }
                }
                (Some(ws), Some(ps), false) => match ps.matches(ws) {
                    SegmentMatchResult::Match => {
                        word_idx += 1;
                        pattern_idx += 1;
                    }
                    SegmentMatchResult::SkipWord => word_idx += 1,
                    SegmentMatchResult::SkipPattern => pattern_idx += 1,

                    SegmentMatchResult::NoMatch => {
                        if self.anchored_start {
                            return false;
                        }

                        start_word_idx += 1;
                        word_idx = start_word_idx;
                        pattern_idx = 0;
                    }
                },
                (Some(ws), Some(ps), true) => match ps.matches(ws) {
                    SegmentMatchResult::Match => {
                        word_idx += 1;
                        pattern_idx += 1;
                    }

                    _ => {
                        if self.anchored_start {
                            return false;
                        }

                        start_word_idx += 1;
                        word_idx = start_word_idx;
                        pattern_idx = 0;
                    }
                },
            }
        }
    }
}
