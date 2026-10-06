use clap::Parser;
use paddler::{dictionary::Dictionary, pattern::Pattern};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    pattern: Pattern,

    #[arg(short, long, default_value = "en_US.txt")]
    dict: PathBuf,

    #[arg(short, long)]
    segmentize: bool,

    #[arg(long)]
    strict: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    if args.segmentize {
        let segments = args
            .pattern
            .segments
            .iter()
            .map(|segment| format!("{segment}"))
            .collect::<Vec<_>>()
            .join(", ");

        println!("{segments}");

        return Ok(());
    }

    let dictionary = Dictionary::from_file(args.dict)?;

    let matches = dictionary.find_matches(args.pattern, args.strict);
    for word in matches {
        println!("{}", word);
    }

    Ok(())
}
