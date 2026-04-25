use clap::ValueEnum;

pub(crate) mod tr;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Language {
    TR,
    EN,
}

impl Language {
    pub fn get_syllabizer(&self) -> Option<for<'a> fn(&'a str) -> Vec<String>> {
        match self {
            Language::TR => Some(tr::syllabize),
            _ => None,
        }
    }
}
