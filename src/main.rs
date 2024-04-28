use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <input>", args[0]);
        process::exit(1);
    };

    let input_filepath: &String = &args[1];
    println!("Reading input...");
    let input: String = match fs::read_to_string(input_filepath) {
        Ok(input) => input.trim_end().to_string(),
        Err(error) => {
            eprintln!("Error reading file \"{}\": {}", input_filepath, error);
            process::exit(1);
        }
    };

    println!("Sorting input...");
    let output: String = empty_space_sorting_algorithm::sort(input.clone());
    println!("\nOutput:\n{}", output);
}
