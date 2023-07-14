use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    print!("Searching for {}", config.query);
    println!(", in file: {}", config.file_path);

    let file_read_error_msg = "Should be able to read the file";
    let file_contents = fs::read_to_string(config.file_path).expect(file_read_error_msg);

    println!("With text:\n{file_contents}");
}

fn parse_config(args: &[String]) -> Config {
    Config { query: args[1].clone(), file_path: args[2].clone() }
}
