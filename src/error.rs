use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Invalid character in pattern: '{0}'")]
    InvalidChar(char),

    #[error("'^' anchor must be at the start of the pattern")]
    MisplacedWordBegin,

    #[error("'$' anchor must be at the end of the pattern")]
    MisplacedWordEnd,

    #[error("Unknown group '{0}' — not defined in the provided configuration")]
    UnknownGroup(char),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}
