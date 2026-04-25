use clap::Parser;
use paddler::lang::Language;
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

    #[arg(long)]
    count: Option<i16>,

    #[arg(value_enum, short, long)]
    language: Language,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config = Config::load_from_path(args.config)?;

    let pattern =
        Pattern::new(&args.pattern, Some(&config)).map_err(|e| format!("Pattern Error: {}", e))?;

    let matcher = Matcher::new(&pattern, Some(&config), args.language);

    let mut matched_count: i16 = 0;
    if let Some(file_path) = args.file {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let word = line?;
            let trimmed_word = word.trim().to_lowercase();
            if trimmed_word.is_empty() {
                continue;
            }

            if matcher.matches(trimmed_word.as_str()) {
                println!("{}", trimmed_word);
                matched_count += 1;
                if let Some(count) = args.count
                    && matched_count == count
                {
                    break;
                }
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
                matched_count += 1;
                if let Some(count) = args.count
                    && matched_count == count
                {
                    break;
                }
            }
        }
    } else {
        eprintln!("Error: You must provide either --input or --file");
        std::process::exit(1);
    }

    Ok(())
}
