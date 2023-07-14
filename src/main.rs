use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

fn run(config: Config) {
    let file_read_error_msg = "Should be able to read the file";
    let file_contents = fs::read_to_string(config.file_path).expect(file_read_error_msg);

    println!("With text:\n{file_contents}");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    print!("Searching for {}", config.query);
    println!(", in file: {}", config.file_path);

    run(config);
}
