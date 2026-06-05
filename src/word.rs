use smol_str::{SmolStr, ToSmolStr};
use std::{fmt::Display, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
pub struct Word {
    pub orthography: SmolStr,
    pub surface: SmolStr,
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.orthography.partial_cmp(&other.orthography)
    }
}

impl Word {
    pub fn new(orthography: SmolStr, surface: SmolStr) -> Self {
        Self {
            orthography,
            surface,
        }
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.orthography, self.surface)
    }
}

pub fn parse_words_from_tsv(file_path: PathBuf) -> Result<Vec<Word>, std::io::Error> {
    let mut words = Vec::new();
    let content = std::fs::read_to_string(file_path)?;

    for line in content.lines() {
        let mut parts = line.split('\t');

        // underlying not yet implemented
        if let (Some(orthography), Some(surface)) = (parts.next(), parts.next()) {
            for surface in surface.split(',').map(|s| s.trim()) {
                words.push(Word::new(orthography.to_smolstr(), surface.to_smolstr()));
            }
        }
    }

    Ok(words)
}
