use std::{error::Error, path::PathBuf};

use crate::{pattern::Pattern, word::Word};

#[derive(Debug)]
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

    pub fn find_matches(&'a self, pattern: Pattern) -> Vec<&'a Word<'a>> {
        pattern.find_matches(&self)
    }
}
