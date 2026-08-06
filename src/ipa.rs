#![allow(dead_code)]
use std::ops::Index;

use crate::feature::FeatureSet;
use crate::modifier::ModifierSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IPA {
    pub symbol: &'static str,
    pub features: FeatureSet,
    pub modifiers: ModifierSet,
}

impl IPA {
    pub fn new(symbol: &'static str, features: FeatureSet) -> Self {
        Self {
            symbol,
            features,
            modifiers: Default::default(),
        }
    }

    pub fn with_modifiers(ipa: &IPA, modifiers: ModifierSet) -> Self {
        let mut features = ipa.features.clone();
        features.apply_modifiers(&modifiers);

        Self {
            symbol: ipa.symbol,
            features,
            modifiers,
        }
    }
}

pub trait IPAMatches<T> {
    fn matches(&self, value: T) -> bool;
}

impl IPAMatches<FeatureSet> for IPA {
    fn matches(&self, value: FeatureSet) -> bool {
        value == self.features
    }
}

impl IPAMatches<&str> for IPA {
    fn matches(&self, value: &str) -> bool {
        value == self.symbol
    }
}

impl IPA {
    pub fn apply_modifiers(&mut self, modifiers: &ModifierSet) {
        self.features.apply_modifiers(modifiers);
    }

    pub fn to_string(&self) -> String {
        self.modifiers.to_string(self.symbol)
    }
}

#[derive(Debug)]
pub struct IPAInventory(pub &'static [IPA]);

impl Index<usize> for IPAInventory {
    type Output = IPA;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
