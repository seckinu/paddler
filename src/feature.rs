#![allow(dead_code)]

use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

use crate::modifier::{Modifier, ModifierSet};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Feature {
    Syllabic,
    Sonorant,
    Consonantal,
    Continuant,
    DelayedRelease,
    Lateral,
    Nasal,
    Strident,
    Voice,
    SpreadGlottis,
    ConstrictedGlottis,
    Anterior,
    Coronal,
    Distributed,
    Labial,
    High,
    Low,
    Back,
    Round,
    Velaric,
    Tense,
    Long,
    HighTone,
    HighRegister,
}

impl Feature {
    pub fn to_string(&self) -> &'static str {
        match self {
            Feature::Syllabic => "syllabic",
            Feature::Sonorant => "sonorant",
            Feature::Consonantal => "consonantal",
            Feature::Continuant => "continuant",
            Feature::DelayedRelease => "delayed release",
            Feature::Lateral => "lateral",
            Feature::Nasal => "nasal",
            Feature::Strident => "strident",
            Feature::Voice => "voice",
            Feature::SpreadGlottis => "spread glottis",
            Feature::ConstrictedGlottis => "constricted glottis",
            Feature::Anterior => "anterior",
            Feature::Coronal => "coronal",
            Feature::Distributed => "distributed",
            Feature::Labial => "labial",
            Feature::High => "high",
            Feature::Low => "low",
            Feature::Back => "back",
            Feature::Round => "round",
            Feature::Velaric => "velaric",
            Feature::Tense => "tense",
            Feature::Long => "long",
            Feature::HighTone => "high tone",
            Feature::HighRegister => "high register",
        }
    }
}

impl Feature {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase() {
            s if s.starts_with("syl") => Some(Feature::Syllabic),
            s if s.starts_with("son") => Some(Feature::Sonorant),
            s if s.starts_with("cons") => Some(Feature::Consonantal),
            s if s.starts_with("cont") => Some(Feature::Continuant),

            s if s.starts_with("delayed") => Some(Feature::DelayedRelease),
            s if s.starts_with("delrel") => Some(Feature::DelayedRelease),

            s if s.starts_with("lat") => Some(Feature::Lateral),
            s if s.starts_with("nas") => Some(Feature::Nasal),
            s if s.starts_with("strid") => Some(Feature::Strident),

            s if s.starts_with("voi") => Some(Feature::Voice),

            // glottis
            s if s.starts_with("spread_glottis") => Some(Feature::SpreadGlottis),
            s if s.starts_with("spreadglottis") => Some(Feature::SpreadGlottis),
            s if s.starts_with("sg") => Some(Feature::SpreadGlottis),

            s if s.starts_with("constricted_glottis") => Some(Feature::ConstrictedGlottis),
            s if s.starts_with("constrictedglottis") => Some(Feature::ConstrictedGlottis),
            s if s.starts_with("cg") => Some(Feature::ConstrictedGlottis),

            s if s.starts_with("ant") => Some(Feature::Anterior),

            s if s.starts_with("cor") => Some(Feature::Coronal),

            s if s.starts_with("distr") => Some(Feature::Distributed),

            s if s.starts_with("lab") => Some(Feature::Labial),

            s if s.starts_with("hi") => Some(Feature::High),
            s if s.starts_with("lo") => Some(Feature::Low),

            s if s.starts_with("back") => Some(Feature::Back),
            s if s.starts_with("bck") => Some(Feature::Back),

            s if s.starts_with("round") => Some(Feature::Round),
            s if s.starts_with("rnd") => Some(Feature::Round),

            s if s.starts_with("velaric") => Some(Feature::Velaric),
            s if s.starts_with("tense") => Some(Feature::Tense),
            s if s.starts_with("long") => Some(Feature::Long),
            s if s.starts_with("hitone") => Some(Feature::HighTone),
            s if s.starts_with("hireg") => Some(Feature::HighRegister),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FeatureState {
    Positive,
    Negative,
    Neutral,
}

impl Default for FeatureState {
    fn default() -> Self {
        FeatureState::Neutral
    }
}

impl From<&str> for FeatureState {
    fn from(value: &str) -> Self {
        match value {
            "+" => FeatureState::Positive,
            "-" => FeatureState::Negative,
            "0" => FeatureState::Neutral,
            _ => FeatureState::Neutral,
        }
    }
}

#[derive(Debug, Eq, Copy, Clone, Default)]
pub struct FeatureSet(pub [FeatureState; 24]);

impl IndexMut<Feature> for FeatureSet {
    fn index_mut(&mut self, index: Feature) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl Index<Feature> for FeatureSet {
    type Output = FeatureState;

    fn index(&self, index: Feature) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<usize> for FeatureSet {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<usize> for FeatureSet {
    type Output = FeatureState;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl FeatureSet {
    pub fn enable(&mut self, feature: Feature) {
        self[feature] = FeatureState::Positive;
    }

    pub fn disable(&mut self, feature: Feature) {
        self[feature] = FeatureState::Negative;
    }

    pub fn negate(&mut self, feature: Feature) {
        let new_state = match self[feature] {
            FeatureState::Positive => FeatureState::Negative,
            FeatureState::Negative => FeatureState::Positive,
            FeatureState::Neutral => FeatureState::Neutral,
        };

        self[feature] = new_state;
    }
}

impl PartialEq for FeatureSet {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0).all(|(self_state, other_state)| {
            match (other_state, self_state) {
                (FeatureState::Neutral, _) => true,
                (FeatureState::Positive, FeatureState::Positive) => true,
                (FeatureState::Negative, FeatureState::Negative | FeatureState::Neutral) => true,
                (FeatureState::Positive, _) => false,
                (FeatureState::Negative, FeatureState::Positive) => false,
            }
        })
    }
}

impl FeatureSet {
    pub fn apply_modifier(&mut self, modifier: &Modifier) {
        for (feature, state) in modifier.feature() {
            match state {
                FeatureState::Neutral => (),
                _ => self[*feature] = *state,
            }
        }
    }

    pub fn apply_modifiers(&mut self, modifiers: &ModifierSet) {
        for modifier in modifiers.modifiers() {
            self.apply_modifier(&modifier);
        }
    }

    pub fn apply_features(&mut self, features: Vec<(Feature, FeatureState)>) {
        for (feature, state) in features {
            match state {
                FeatureState::Neutral => (),
                _ => self[feature] = state,
            }
        }
    }

    pub fn apply_feature_set(&mut self, other: &FeatureSet) {
        for (feature, state) in other.0.iter().enumerate() {
            match state {
                FeatureState::Neutral => (),
                _ => self[feature] = *state,
            }
        }
    }
}

impl FromStr for FeatureSet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut feature_set = FeatureSet::default();

        // "+feature -feature, feature"
        // if no sign is provided, assume positive
        for part in s.split([',', ' ']) {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            // let (sign, feature_str) = match part.chars().next() {
            //     Some('+') => (FeatureState::Positive, &part[1..]),
            //     Some('-') => (FeatureState::Negative, &part[1..]),
            //     _ => (FeatureState::Positive, part),
            // };

            let (sign, feature_str) = if part.starts_with('+') {
                (FeatureState::Positive, &part[1..])
            } else if part.starts_with('-') {
                (FeatureState::Negative, &part[1..])
            } else {
                (FeatureState::Positive, part)
            };

            let Some(feature) = Feature::from_str(feature_str) else {
                return Err(format!("Unknown feature: {feature_str}"));
            };
            feature_set[feature] = sign;
        }

        Ok(feature_set)
    }
}
