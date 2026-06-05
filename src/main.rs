use clap::Parser;
use paddler::{
    pattern::Pattern,
    word::{Word, parse_words_from_tsv},
};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    pattern: String,

    #[arg(short, long, default_value = "en_US.txt")]
    dict: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let pattern = Pattern::new(&args.pattern)?;

    let words: Vec<Word> = parse_words_from_tsv(args.dict)?;

    let matches = pattern.find_matches(&words);

    for word in matches {
        println!("{}", word);
    }

    Ok(())
}
