use clap::Parser;
use paddler::{dictionary::Dictionary, pattern::Pattern};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

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

    #[arg(short, long)]
    out: Option<PathBuf>,
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

    if let Some(out_file) = args.out {
        let file = File::create_new(out_file)?;
        let mut writer = BufWriter::new(file);

        for word in matches {
            writeln!(writer, "{}", word)?;
        }

        writer.flush()?;
    } else {
        for word in matches {
            println!("{}", word);
        }
    }

    Ok(())
}
