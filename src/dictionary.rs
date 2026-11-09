use std::{error::Error, path::PathBuf};

use crate::{
    pattern::{PaddlerMatchResult, Pattern},
    word::Word,
};

#[cfg(feature = "rayon")]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dictionary<'a>(pub Vec<Word<'a>>);

impl<'a> Dictionary<'a> {
    pub fn from_file(file_path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let content: &'a str = Box::leak(Box::from(std::fs::read_to_string(file_path)?));
        let mut words = Vec::new();

        for line in content.lines() {
            let mut parts = line.split('\t');

            if let (Some(orthography), Some(surface)) = (parts.next(), parts.next()) {
                for surface in surface.split(',').map(|s| {
                    s.trim()
                        .trim_end_matches(['/', ']'])
                        .trim_start_matches(['/', '['])
                }) {
                    let word = Word::new(orthography, surface);
                    words.push(word);
                }
            }
        }

        Ok(Self(words))
    }

    #[cfg(feature = "rayon")]
    pub fn find_matches_map<F, O>(&'a self, pattern: Pattern, strict: bool, map_op: F) -> Vec<O>
    where
        F: Fn((&'a Word, PaddlerMatchResult)) -> O + Sync + Send,
        O: Send,
    {
        self.0
            .par_iter()
            .filter_map(|word| pattern.matches(word.surface, strict).map(|m| (word, m)))
            .map(map_op)
            .collect()
    }

    #[cfg(not(feature = "rayon"))]
    pub fn find_matches_map<F, O>(&'a self, pattern: Pattern, strict: bool, map_op: F) -> Vec<O>
    where
        F: Fn((&'a Word, PaddlerMatchResult)) -> O,
    {
        self.0
            .iter()
            .filter_map(|word| pattern.matches(word.surface, strict).map(|m| (word, m)))
            .map(map_op)
            .collect()

        // matches
    }

    #[cfg(feature = "rayon")]
    pub fn find_matches(
        &'a self,
        pattern: Pattern,
        strict: bool,
    ) -> Vec<(&'a Word<'a>, PaddlerMatchResult)> {
        self.0
            .par_iter()
            .filter_map(|word| pattern.matches(word.surface, strict).map(|m| (word, m)))
            .collect()
    }

    #[cfg(not(feature = "rayon"))]
    pub fn find_matches(
        &'a self,
        pattern: Pattern,
        strict: bool,
    ) -> Vec<(&'a Word<'a>, PaddlerMatchResult)> {
        self.0
            .iter()
            .filter_map(|word| pattern.matches(word.surface, strict).map(|m| (word, m)))
            .collect()
    }
}
