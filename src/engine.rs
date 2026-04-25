use crate::config::Config;
use crate::error::EngineError;
use crate::lang;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Boundary {
    WordBegin,
    WordEnd,
    Syllable,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

        let anchored_start = matches!(tokens.first(), Some(Token::Boundary(Boundary::WordBegin)));
        let anchored_end = matches!(tokens.last(), Some(Token::Boundary(Boundary::WordEnd)));

        if anchored_start {
            tokens.remove(0);
        }
        if anchored_end {
            tokens.pop();
        }

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
    syllabizer: Option<fn(word: &str) -> Vec<String>>,
    _lang: lang::Language,
}

impl<'a> Matcher<'a> {
    pub fn new(pattern: &'a Pattern, config: Option<&'a Config>, lang: lang::Language) -> Self {
        Self {
            pattern,
            config,
            syllabizer: lang.get_syllabizer(),
            _lang: lang,
        }
    }

    pub fn matches(&self, input: &str) -> bool {
        let mut chars = if let Some(syllabizer) = self.syllabizer {
            let syllables: Vec<String> = syllabizer(input);
            let mut tokens = Vec::new();

            for (i, syllable) in syllables.iter().enumerate() {
                for c in syllable.chars() {
                    tokens.push(Token::Char(c));
                }

                if i < syllables.len() - 1 {
                    tokens.push(Token::Boundary(Boundary::Syllable));
                }
            }
            tokens
        } else {
            input.chars().map(|c| Token::Char(c)).collect()
        };

        let mut chars_iter = chars.iter();
        let mut tokens_iter = self.pattern.tokens.iter();

        let (mut token, mut char) = (tokens_iter.next(), chars_iter.next());
        loop {
            match (char, token) {
                (None, None) => return true,
                (None, Some(_)) => return false,
                // input remains, pattern finished
                (Some(_), None) => {
                    if self.pattern.anchored_end {
                        tokens_iter = self.pattern.tokens.iter();
                        token = tokens_iter.next();
                        chars.remove(0);
                        chars_iter = chars.iter();
                        char = chars_iter.next();
                    } else {
                        return true;
                    }
                }
                (
                    Some(Token::Boundary(Boundary::Syllable)),
                    Some(Token::Group(_)) | Some(Token::Char(_)),
                ) => char = chars_iter.next(),
                (Some(Token::Char(_)), Some(Token::Boundary(Boundary::Syllable))) => {
                    if self.pattern.anchored_start {
                        return false;
                    }
                    tokens_iter = self.pattern.tokens.iter();
                    token = tokens_iter.next();
                    chars.remove(0);
                    chars_iter = chars.iter();
                    char = chars_iter.next();
                }
                (Some(Token::Char(ch)), Some(Token::Char(t_ch))) if ch != t_ch => {
                    if self.pattern.anchored_start {
                        return false;
                    }
                    tokens_iter = self.pattern.tokens.iter();
                    token = tokens_iter.next();
                    chars.remove(0);
                    chars_iter = chars.iter();
                    char = chars_iter.next();
                }
                (Some(Token::Char(ch)), Some(Token::Group(group))) => {
                    let Some(config) = self.config else {
                        return false;
                    };

                    let contains = config
                        .groups
                        .get(&group)
                        .map_or(false, |set| set.contains(ch));

                    if !contains {
                        if self.pattern.anchored_start {
                            return false;
                        }
                        tokens_iter = self.pattern.tokens.iter();
                        token = tokens_iter.next();
                        chars.remove(0);
                        chars_iter = chars.iter();
                        char = chars_iter.next();
                    } else {
                        char = chars_iter.next();
                        token = tokens_iter.next();
                    }
                }
                (Some(_), Some(_)) => {
                    char = chars_iter.next();
                    token = tokens_iter.next();
                }
            }
        }
    }
}
