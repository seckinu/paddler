use crate::config::Config;
use crate::error::EngineError;
use crate::utils::syllabize;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Boundary {
    WordBegin,
    WordEnd,
    Syllable,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Token {
    Group(char),
    Boundary(Boundary),
    Char(char),
}

pub struct Pattern {
    pub raw: String,
    pub tokens: Vec<Token>,
    pub anchored_start: bool,
    pub anchored_end: bool,
}

impl Pattern {
    pub fn new(pattern: &str, config: Option<&Config>) -> Result<Self, EngineError> {
        let mut tokens = Self::parse(pattern)?;

        // FIXED: Use explicit checks for the first and last elements
        let anchored_start = matches!(tokens.first(), Some(Token::Boundary(Boundary::WordBegin)));
        let anchored_end = matches!(tokens.last(), Some(Token::Boundary(Boundary::WordEnd)));

        if anchored_start {
            tokens.remove(0);
        }
        if anchored_end {
            tokens.pop();
        }

        // FIXED: Validate that NO other boundaries exist in the middle
        for token in &tokens {
            match token {
                Token::Boundary(Boundary::WordBegin) => {
                    return Err(EngineError::MisplacedWordBegin);
                }
                Token::Boundary(Boundary::WordEnd) => return Err(EngineError::MisplacedWordEnd),
                Token::Group(g) => {
                    if let Some(cfg) = config {
                        if !cfg.groups.contains_key(g) {
                            return Err(EngineError::UnknownGroup(*g));
                        }
                    } else {
                        // If no config provided, any Group token is an error
                        return Err(EngineError::UnknownGroup(*g));
                    }
                }
                _ => {}
            }
        }

        Ok(Self {
            raw: pattern.to_string(),
            tokens,
            anchored_start,
            anchored_end,
        })
    }

    fn parse(pattern: &str) -> Result<Vec<Token>, EngineError> {
        pattern
            .chars()
            .map(|ch| match ch {
                '$' => Ok(Token::Boundary(Boundary::WordEnd)),
                '^' => Ok(Token::Boundary(Boundary::WordBegin)),
                '.' => Ok(Token::Boundary(Boundary::Syllable)),
                v if v.is_uppercase() => Ok(Token::Group(v)),
                v if v.is_alphabetic() => Ok(Token::Char(v)),
                _ => Err(EngineError::InvalidChar(ch)),
            })
            .collect()
    }
}

pub struct Matcher<'a> {
    pattern: &'a Pattern,
    config: Option<&'a Config>,
}

impl<'a> Matcher<'a> {
    pub fn new(pattern: &'a Pattern, config: Option<&'a Config>) -> Self {
        Self { pattern, config }
    }

    pub fn matches(&self, input: &str) -> bool {
        let syllables = match syllabize(input) {
            Some(s) => s,
            None => return false,
        };

        let mut chars: Vec<char> = Vec::with_capacity(32);
        let mut syllable_boundary_indices: Vec<usize> = Vec::with_capacity(10);

        for (i, syllable) in syllables.iter().enumerate() {
            if i > 0 {
                syllable_boundary_indices.push(chars.len());
            }
            for ch in syllable.chars() {
                chars.push(ch);
            }
        }

        let word_len = chars.len();
        let start_range = if self.pattern.anchored_start {
            0..=0
        } else {
            0..=word_len
        };

        'outer: for start in start_range {
            let mut char_pos = start;
            let mut tok_pos = 0;

            while tok_pos < self.pattern.tokens.len() {
                let token = &self.pattern.tokens[tok_pos];

                match token {
                    Token::Boundary(Boundary::Syllable) => {
                        if !syllable_boundary_indices.binary_search(&char_pos).is_ok() {
                            continue 'outer;
                        }
                        tok_pos += 1;
                    }
                    Token::Group(g) => {
                        if char_pos >= word_len {
                            continue 'outer;
                        }

                        let cfg = self
                            .config
                            .expect("Pattern contains groups but no config provided");
                        let ch = chars[char_pos];

                        if !cfg.groups.get(g).map_or(false, |set| set.contains(&ch)) {
                            continue 'outer;
                        }
                        char_pos += 1;
                        tok_pos += 1;
                    }
                    Token::Char(c) => {
                        if char_pos >= word_len || chars[char_pos] != *c {
                            continue 'outer;
                        }
                        char_pos += 1;
                        tok_pos += 1;
                    }
                    _ => continue 'outer,
                }
            }

            if self.pattern.anchored_end && char_pos != word_len {
                continue 'outer;
            }

            return true;
        }

        false
    }
}
