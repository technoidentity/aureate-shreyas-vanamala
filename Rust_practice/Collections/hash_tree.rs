use std::env;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::build(&args) {
        Ok(config) => config,
        Err(message) => {
            eprintln!("Problem parsing arguments: {message}");
            return;
        }
    };

    println!("Searching for '{}' in '{}'", config.query, config.file_path);
}
