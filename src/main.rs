use clap::Parser;
use paddler::{dictionary::Dictionary, pattern::Pattern};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    pattern: Pattern,

    #[arg(short, long, default_value = "en_US.txt")]
    dict: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    let dictionary = Dictionary::from_file(args.dict)?;

    let matches = dictionary.find_matches(args.pattern);
    for word in matches {
        println!("{}", word);
    }

    Ok(())
}
