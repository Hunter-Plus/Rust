use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for string: {query}In file path: {file_path}");

    let contents = fs::read_to_string(file_path).expect("File Reading should be ok");

    println!("The content in tha file:\n{contents}");
}
