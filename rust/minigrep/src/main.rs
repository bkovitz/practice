use std::{env, fs, process, error::Error};
use minigrep::{search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(2);
    });
    if let Err(err) = run(config) {
        eprintln!("{err}");
        process::exit(2);
    }
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("usage: minigrep query file_path");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let search_fn = if config.ignore_case {
        search_case_insensitive
    } else {
        search
    };
    for line in search_fn(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}