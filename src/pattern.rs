use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use thiserror::Error;

use crate::{
    segment::{Segment, SegmentError, SegmentMatchResult},
    word::Word,
};

#[derive(Debug, Error)]
pub enum PatternError {
    #[error(transparent)]
    SegmentError(#[from] SegmentError),

    #[error("Invalid pattern format")]
    InvalidPatternFormat,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    segments: Vec<Segment>,
    anchored_start: bool,
    anchored_end: bool,
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

    pub fn from_str(pattern_str: &str) -> Result<Self, PatternError> {
        Self::new(pattern_str)
    }

    pub fn matches(&self, word: &Word) -> bool {
        let mut word_iter = word.surface.chars().peekable();

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

            match (word_segment, pattern_segment) {
                (None, None) => return true,
                (None, Some(_)) => return false,
                (Some(_), None) => {
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
                (Some(ws), Some(ps)) => {
                    let match_result: SegmentMatchResult = ps.matches(ws);
                    if match_result == SegmentMatchResult::Match {
                        word_idx += 1;
                        pattern_idx += 1;
                    } else if match_result == SegmentMatchResult::Skip {
                        word_idx += 1;
                    } else {
                        if self.anchored_start {
                            return false;
                        }

                        start_word_idx += 1;
                        word_idx = start_word_idx;
                        pattern_idx = 0;
                    }
                }
            }
        }
    }

    pub fn find_matches<'a>(&self, words: &'a Vec<Word>) -> Vec<&'a Word> {
        let mut matches = words
            .par_iter()
            .filter(|word| self.matches(word))
            .collect::<Vec<&Word>>();

        matches.par_sort();

        matches
    }
}
