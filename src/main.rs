use clap::Parser;
use paddler::{Config, Matcher, Pattern};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
    clap::ArgGroup::new("source")
        .required(true)
		.multiple(false)
        .args(["file", "input"]),
))]
struct Args {
    #[arg(short, long)]
    config: Option<PathBuf>,

    #[arg(short, long)]
    pattern: String,

    input: Vec<String>,

    #[arg(short, long)]
    file: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config = Config::load_from_path(args.config)?;

    let pattern =
        Pattern::new(&args.pattern, Some(&config)).map_err(|e| format!("Pattern Error: {}", e))?;

    let matcher = Matcher::new(&pattern, Some(&config));

    if let Some(file_path) = args.file {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let word = line?;
            let trimmed_word = word.trim();
            if trimmed_word.is_empty() {
                continue;
            }

            if matcher.matches(trimmed_word) {
                println!("{}", trimmed_word);
            }
        }
    } else if args.input.len() > 0 {
        for word in args.input {
            let trimmed_word = word.trim();
            if trimmed_word.is_empty() {
                continue;
            }

            if matcher.matches(trimmed_word) {
                println!("{}", trimmed_word);
            }
        }
    } else {
        eprintln!("Error: You must provide either --input or --file");
        std::process::exit(1);
    }

    Ok(())
}
