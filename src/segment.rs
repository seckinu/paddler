use std::iter::Peekable;
use std::str::FromStr;

use smol_str::ToSmolStr;
use thiserror::Error;

use crate::{
    ipa::{FeatureSet, IPA, IPAInventory},
    modifier::{Modifier, ModifierPosition},
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
            let mut modifiers: Vec<Modifier> = Default::default();
            let mut possible_ipa_symbol = String::with_capacity(8);
            let mut last_exact_match: Option<&IPA> = None;

            while let Some(c) = iter.peek() {
                if ['ˈ', '\'', '.', 'ˌ', '_'].contains(c) {
                    break;
                }

                possible_ipa_symbol.push(*c);

                // 3 cases:
                // - the character is a modifier (e.g. ejective, voiceless, etc.)
                // - the character is part of the previous IPA symbol (e.g. 'ʃ' for 'tʃ')
                // - the character is a new IPA symbol (e.g. 'k' after 'k')

                // part of the previous IPA
                if let Some(m) = ipa_inventory.find_exact_match(&possible_ipa_symbol) {
                    last_exact_match = Some(m);
                    iter.next();
                    continue;
                } else if ipa_inventory.find_exact_match(&c.to_smolstr()).is_some() {
                    // a new IPA
                    break;
                };

                // a modifier
                if let Some(modifier) = Modifier::from_str(&c.to_string()) {
                    possible_ipa_symbol.pop();

                    let modifier_position = modifier.position();
                    let Some(last_modifier) = modifiers.last() else {
                        iter.next();
                        modifiers.push(modifier);
                        continue;
                    };

                    match (modifier_position, last_modifier.position()) {
                        (ModifierPosition::Pre, ModifierPosition::Pre) => modifiers.push(modifier),
                        (ModifierPosition::Post, ModifierPosition::Post) => {
                            modifiers.push(modifier)
                        }
                        (ModifierPosition::Pre, ModifierPosition::Post) => modifiers.push(modifier),
                        (ModifierPosition::Post, ModifierPosition::Pre) => {
                            break;
                        }
                    }

                    iter.next();
                } else {
                    // not a modifier, not part of the previous IPA, and not a new IPA => error
                    break;
                    // return Err(SegmentError::NoMatchingIPASymbol(possible_ipa_symbol));
                }
            }

            let mut ipa: IPA = last_exact_match
                .ok_or_else(|| SegmentError::NoMatchingIPASymbol(possible_ipa_symbol.clone()))?
                .clone();

            ipa.apply_modifiers(modifiers);

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
