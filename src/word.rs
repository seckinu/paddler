use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
pub struct Word<'a> {
    pub orthography: &'a str,
    pub surface: &'a str,
}

impl<'a> PartialOrd for Word<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.orthography.partial_cmp(&other.orthography)
    }
}

impl<'a> Word<'a> {
    pub fn new(orthography: &'a str, surface: &'a str) -> Self {
        Self {
            orthography,
            surface,
        }
    }
}

impl<'a> Display for Word<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.orthography, self.surface)
    }
}
