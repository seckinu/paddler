use std::env;
use std::fs;
use std::path::Path;

#[path = "src/feature.rs"]
mod feature;
#[path = "src/ipa.rs"]
mod ipa;
#[path = "src/modifier.rs"]
mod modifier;

use feature::FeatureState;

use feature::FeatureSet;
use ipa::IPA;
use modifier::ModifierSet;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let csv_data = include_str!("./data/ipa_base.csv");

    let mut ipa_inv_codegen = String::new();
    ipa_inv_codegen.push_str(
        "use crate::feature::{FeatureSet, FeatureState};
use crate::modifier::ModifierSet;
use crate::ipa::{IPA, IPAInventory};\n\n",
    );
    ipa_inv_codegen.push_str("pub static IPA_INVENTORY: IPAInventory = IPAInventory(&[\n");

    let mut symbols_codegen = String::new();
    symbols_codegen.push_str("static IPA_SYMBOLS: ::phf::Map<&'static str, usize> = ");

    let mut symbols_phf = phf_codegen::Map::new();

    let mut ipa_vector: Vec<(IPA, usize)> = Vec::new();
    for (index, line) in csv_data.lines().enumerate() {
        if index == 0 || line.trim().is_empty() {
            continue;
        }

        let mut cols = line.split(',');

        let Some(symbol) = cols.next() else {
            continue;
        };

        let mut features = FeatureSet::default();
        let mut features_str = String::new();
        features_str.push_str("FeatureSet([");
        for idx in 0..24 {
            let Some(sign) = cols.next() else {
                break;
            };

            features_str.push_str(match FeatureState::from(sign.trim()) {
                FeatureState::Negative => "FeatureState::Negative,",
                FeatureState::Positive => "FeatureState::Positive,",
                FeatureState::Neutral => "FeatureState::Neutral,",
            });

            features.0[idx] = FeatureState::from(sign.trim());
        }

        features_str.push_str("])");

        ipa_inv_codegen.push_str(&format!(
            "IPA {{symbol: \"{symbol}\", features: {features_str}, modifiers: ModifierSet::blank()}},\n"
        ));

        let inv_index = index - 1;
        // symbols_phf.entry(symbol, format!("&IPA_INVENTORY.0[{inv_index}]")); // Somehow this is slower
        symbols_phf.entry(symbol, format!("{inv_index}usize"));

        ipa_vector.push((
            IPA {
                symbol,
                features,
                modifiers: ModifierSet::default(),
            },
            inv_index,
        ));
    }

    let mut symbols_to_matches = String::new();
    symbols_to_matches.push_str("static SYMBOLS_TO_MATCHES: ::phf::Map<&'static str, &[usize]> = ");
    let mut symbols_to_matches_phf = phf_codegen::Map::new();
    for ipa in ipa_vector.iter().map(|(item, _)| item.symbol) {
        let matches = ipa_vector
            .iter()
            .filter(|(item, _)| item.symbol.starts_with(ipa))
            .map(|(_, idx)| idx.to_string())
            .collect::<Vec<_>>();

        symbols_to_matches_phf.entry(ipa, format!("&[{}]", matches.join(", ")));
    }
    symbols_to_matches.push_str(&symbols_to_matches_phf.build().to_string());
    symbols_to_matches.push_str(";\n");

    symbols_codegen.push_str(&symbols_phf.build().to_string());
    symbols_codegen.push_str(";\n");

    ipa_inv_codegen.push_str("]);\n");

    ipa_inv_codegen.push_str(&symbols_codegen);
    ipa_inv_codegen.push_str(&symbols_to_matches);

    ipa_inv_codegen.push_str(
        "
impl IPAInventory {
    pub fn find_exact_match(&self, value: &str) -> Option<&IPA> {
        // IPA_SYMBOLS.get(value).map(|v| *v)
        IPA_SYMBOLS.get(value).map(|v| &IPA_INVENTORY.0[*v])
    }

    pub fn find_possible_matches(&self, value: &str) -> &[usize] {
        SYMBOLS_TO_MATCHES.get(value).map(|v| *v).unwrap_or_default()
    }
}",
    );

    let dest_path = Path::new(&out_dir).join("ipa_codegen.rs");
    fs::write(dest_path, format!("{ipa_inv_codegen}")).unwrap();

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=ipa_base.csv");
}
