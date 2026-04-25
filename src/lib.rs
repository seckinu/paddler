pub mod config;
pub mod engine;
pub mod error;
pub mod lang;

pub use config::Config;
pub use engine::{Matcher, Pattern};
pub use error::EngineError;

#[cfg(test)]
mod test;
