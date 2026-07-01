pub mod dictionary;
pub mod feature;
pub mod ipa;
pub mod modifier;
pub mod pattern;
pub mod segment;
pub mod word;

include!(concat!(env!("OUT_DIR"), "/ipa_codegen.rs"));

#[cfg(test)]
mod test;
