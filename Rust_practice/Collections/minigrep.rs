use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::process;

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("expected a query and a file path; usage: minigrep <query> <file_path>");
        }

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: env::var_os("IGNORE_CASE").is_some(),
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("could not read '{}': {error}", config.file_path),
        )
    })?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENTS: &str = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

    #[test]
    fn case_sensitive_search_preserves_case() {
        assert_eq!(search("duct", CONTENTS), vec!["safe, fast, productive."]);
        assert_eq!(search("rust", CONTENTS), vec!["Trust me."]);
    }

    #[test]
    fn case_insensitive_search_returns_original_lines() {
        assert_eq!(
            search_case_insensitive("rUsT", CONTENTS),
            vec!["Rust:", "Trust me."]
        );
    }

    #[test]
    fn missing_query_returns_no_matches() {
        assert!(search("missing", CONTENTS).is_empty());
        assert!(search_case_insensitive("missing", CONTENTS).is_empty());
    }

    #[test]
    fn empty_contents_returns_no_matches() {
        assert!(search("Rust", "").is_empty());
        assert!(search_case_insensitive("Rust", "").is_empty());
    }

    #[test]
    fn empty_query_matches_every_line() {
        let lines: Vec<_> = CONTENTS.lines().collect();
        assert_eq!(search("", CONTENTS), lines);
        assert_eq!(search_case_insensitive("", CONTENTS), lines);
    }

    #[test]
    fn config_rejects_missing_and_extra_arguments() {
        for args in [
            vec!["minigrep"],
            vec!["minigrep", "query"],
            vec!["minigrep", "query", "file", "extra"],
        ] {
            let args: Vec<String> = args.into_iter().map(String::from).collect();
            assert!(Config::build(&args).is_err());
        }
    }

    #[test]
    fn config_stores_query_and_file_path() {
        let args = vec!["minigrep".into(), "Rust".into(), "poem.txt".into()];
        let config = Config::build(&args).unwrap();
        assert_eq!(config.query, "Rust");
        assert_eq!(config.file_path, "poem.txt");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|message| {
        eprintln!("Problem parsing arguments: {message}");
        process::exit(1);
    });

    if let Err(error) = run(config) {
        eprintln!("Application error: {error}");
        process::exit(1);
    }
}
