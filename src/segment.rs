use std::{iter::Peekable, str::FromStr};

use thiserror::Error;

use crate::{
    feature::FeatureSet,
    ipa::{IPA, IPAInventory},
    modifier::{Modifier, ModifierSet},
};

#[derive(Debug, PartialEq, Eq, Error)]
pub enum SegmentError {
    #[error("No matching IPA symbol found: {0}")]
    NoMatchingIPASymbol(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum SegmentMatchResult {
    Match,
    NoMatch,
    Skip,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Segment {
    IPA(IPA),
    FeatureSet(FeatureSet),
    Any,
    Stress,
    SecondaryStress,
    Syllable,
}

impl Segment {
    pub fn to_string(&self) -> Option<String> {
        match self {
            Segment::IPA(ipa) => Some(ipa.to_string()),
            Segment::FeatureSet(_) => None,
            Segment::Any => None,
            Segment::Stress => Some("'".to_string()),
            Segment::SecondaryStress => Some(",".to_string()),
            Segment::Syllable => Some(".".to_string()),
        }
    }
}

impl Segment {
    pub fn matches(&self, other: &Segment) -> SegmentMatchResult {
        match (self, other) {
            (Segment::Any, _) => SegmentMatchResult::Match,
            (Segment::IPA(ipa1), Segment::IPA(ipa2)) => {
                if ipa1 == ipa2 {
                    SegmentMatchResult::Match
                } else {
                    SegmentMatchResult::NoMatch
                }
            }
            (Segment::FeatureSet(fs1), Segment::FeatureSet(fs2)) => {
                if fs2 == fs1 {
                    SegmentMatchResult::Match
                } else {
                    SegmentMatchResult::NoMatch
                }
            }
            (Segment::FeatureSet(fs), Segment::IPA(ipa)) => {
                if &ipa.features == fs {
                    SegmentMatchResult::Match
                } else {
                    SegmentMatchResult::NoMatch
                }
            }
            (Segment::Stress, Segment::Stress) => SegmentMatchResult::Match,
            (Segment::Syllable, Segment::Syllable) => SegmentMatchResult::Match,
            (Segment::Syllable, Segment::Stress | Segment::SecondaryStress) => {
                SegmentMatchResult::Match
            }
            _ => {
                if other.is_skippable() {
                    SegmentMatchResult::Skip
                } else {
                    SegmentMatchResult::NoMatch
                }
            }
        }
    }

    pub fn is_skippable(&self) -> bool {
        matches!(
            self,
            Segment::Stress | Segment::Syllable | Segment::SecondaryStress
        )
    }

    pub fn parse(
        iter: &mut Peekable<impl Iterator<Item = char>>,
    ) -> Result<Option<Self>, SegmentError> {
        let Some(peek) = iter.peek() else {
            return Ok(None);
        };

        let ipa_inventory = IPAInventory::global();

        if matches!(peek, 'ˈ' | '\'') {
            iter.next();
            Ok(Some(Segment::Stress))
        } else if matches!(peek, '.') {
            iter.next();
            Ok(Some(Segment::Syllable))
        } else if matches!(peek, '_') {
            iter.next();
            Ok(Some(Segment::Any))
        } else if matches!(peek, 'ˌ') {
            iter.next();
            Ok(Some(Segment::SecondaryStress))
        } else if matches!(peek, '[') {
            iter.next();

            let mut features_str = String::new();

            while let Some(c) = iter.peek() {
                if *c == ']' {
                    iter.next();
                    break;
                }

                features_str.push(*c);
                iter.next();
            }

            let features = FeatureSet::from_str(&features_str)
                .map_err(|_| SegmentError::NoMatchingIPASymbol(features_str.clone()))?;

            Ok(Some(Segment::FeatureSet(features)))
        } else {
            let mut modifiers: ModifierSet = Default::default();
            let mut possible_ipa_symbol = String::with_capacity(4);
            let mut matches: Vec<&IPA> = Vec::new();

            let mut base_ipa: Option<&IPA> = None;

            while let Some(c) = iter.peek() {
                if ['ˈ', '\'', '.', 'ˌ', '_'].contains(c) {
                    break;
                }

                if base_ipa.is_none() {
                    if *c == '\u{361}' {
                        iter.next();
                        continue;
                    }

                    possible_ipa_symbol.push(*c);
                    if matches.len() == 0 {
                        matches = ipa_inventory.find_possible_matches(&possible_ipa_symbol)
                    } else {
                        matches.retain(|ipa| ipa.symbol.starts_with(&possible_ipa_symbol))
                    };

                    match matches.len() {
                        // no matches
                        0 => {
                            // beginning of symbol, no matches -> 0 from start
                            // continuation of a symbol, more than 1 matches to 0

                            if possible_ipa_symbol.len() == 1 {
                                // might also be a pre-modifier
                                if let Some(modifier) = Modifier::from_str(&possible_ipa_symbol)
                                    && modifier.is_pre()
                                {
                                    iter.next();
                                    possible_ipa_symbol.pop();

                                    modifiers.enable(modifier);

                                    continue;
                                }

                                return Err(SegmentError::NoMatchingIPASymbol(possible_ipa_symbol));
                            } else {
                                possible_ipa_symbol.pop();

                                let Some(exact_match) =
                                    ipa_inventory.find_exact_match(&possible_ipa_symbol)
                                else {
                                    return Err(SegmentError::NoMatchingIPASymbol(
                                        possible_ipa_symbol,
                                    ));
                                };

                                base_ipa = Some(exact_match);
                            }

                            continue;
                        }
                        1 => {
                            iter.next();
                            // either matches perfectly, or the only symbol that begins the same
                            if matches[0].symbol == possible_ipa_symbol {
                                base_ipa = Some(matches[0]);
                            } else {
                                while let Some(inner_c) = iter.peek() {
                                    possible_ipa_symbol.push(*inner_c);

                                    if matches[0].symbol == possible_ipa_symbol {
                                        iter.next();
                                        base_ipa = Some(matches[0]);
                                        break;
                                    } else if matches[0].symbol.starts_with(&possible_ipa_symbol) {
                                        iter.next();
                                        continue;
                                    } else {
                                        return Err(SegmentError::NoMatchingIPASymbol(
                                            possible_ipa_symbol,
                                        ));
                                    }
                                }
                            }

                            continue;
                        }
                        // 2 or more possible matches
                        _ => {
                            iter.next();

                            if iter.peek().is_none() {
                                base_ipa = ipa_inventory.find_exact_match(&possible_ipa_symbol);
                            }

                            continue;
                        }
                    };
                }

                // a modifier
                // can only be a post-modifier, since we cannot get here without finding an ipa symbol
                if let Some(modifier) = Modifier::from_str(&c.to_string()) {
                    modifiers.enable(modifier);

                    iter.next();
                } else {
                    break;
                }
            }

            let ipa: IPA = IPA::with_modifiers(
                base_ipa.ok_or_else(|| {
                    SegmentError::NoMatchingIPASymbol(possible_ipa_symbol.clone())
                })?,
                modifiers,
            );

            Ok(Some(Segment::IPA(ipa)))
        }
    }

    pub fn parse_all(input: &str) -> Result<Vec<Self>, SegmentError> {
        let mut iter = input.chars().peekable();
        let mut segments: Vec<Segment> = Default::default();
        while let Some(_) = iter.peek() {
            let segment = Segment::parse(&mut iter)?;
            match segment {
                Some(segment) => segments.push(segment),
                None => break,
            }
        }

        Ok(segments)
    }
}
