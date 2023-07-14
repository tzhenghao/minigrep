use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    print!("Searching for {query}");
    println!(", in file: {file_path}");

    let file_read_error_msg = "Should be able to read the file";
    let file_contents = fs::read_to_string(file_path).expect(file_read_error_msg);
    
    println!("With text:\n{file_contents}");
}
