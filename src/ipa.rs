use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

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
pub struct IPAInventory(Arc<[IPA]>);

pub static IPA_INVENTORY: OnceLock<IPAInventory> = OnceLock::new();
pub static IPA_SYMBOLS: OnceLock<HashMap<&str, &IPA>> = OnceLock::new();

impl IPAInventory {
    pub fn global() -> &'static Self {
        let result = IPA_INVENTORY.get_or_init(|| Self::get_from_embedded_csv());

        IPA_SYMBOLS.get_or_init(|| {
            result
                .0
                .iter()
                .map(|ipa| (ipa.symbol, ipa))
                .collect::<HashMap<_, _>>()
        });

        result
    }

    fn get_from_embedded_csv() -> Self {
        let csv_data = include_str!("../ipa_base.csv");
        let mut ipa_list = Vec::new();

        for (index, line) in csv_data.lines().enumerate() {
            if index == 0 || line.trim().is_empty() {
                continue;
            }

            let mut cols = line.split(',');

            let Some(symbol) = cols.next() else {
                continue;
            };
            let mut features = FeatureSet::default();

            for i in 0..24 {
                let Some(sign) = cols.next() else {
                    break;
                };
                features[i] = sign.trim().into()
            }

            ipa_list.push(IPA::new(symbol, features));
        }
        IPAInventory(ipa_list.into())
    }

    pub fn find_exact_match(&self, value: &str) -> Option<&IPA> {
        IPA_SYMBOLS
            .get()
            .and_then(|symbols| symbols.get(value))
            .map(|v| *v)
    }

    pub fn find_possible_matches(&self, value: &str) -> Vec<&IPA> {
        self.0
            .iter()
            .filter(|ipa| ipa.symbol.starts_with(value))
            .collect()
    }
}
