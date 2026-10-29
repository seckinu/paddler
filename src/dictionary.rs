use std::{error::Error, path::PathBuf};

use crate::{pattern::Pattern, word::Word};
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

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

    pub fn find_matches_map<F, O>(&'a self, pattern: Pattern, strict: bool, map_op: F) -> Vec<O>
    where
        F: Fn(&'a Word) -> O + Sync + Send,
        O: Ord + Send,
    {
        let mut matches: Vec<O> = self
            .0
            .par_iter()
            .filter(|word| pattern.matches(word.surface, strict))
            .map(map_op)
            .collect();

        matches.par_sort();

        matches
    }

    pub fn find_matches(&'a self, pattern: Pattern, strict: bool) -> Vec<&'a Word<'a>> {
        let mut matches: Vec<&'a Word<'a>> = self
            .0
            .par_iter()
            .filter(|word| pattern.matches(word.surface, strict))
            .collect();

        matches.par_sort();

        matches
    }
}
