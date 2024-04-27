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
    let output: String = sort(input.clone());
    println!("\nOutput:\n{}", output);
}

fn sort(input: String) -> String {
    let empty: &str = "X";
    let separator: &str = "-";

    let mut input_vec: Vec<&str> = input.split(separator).collect(); // Input
    let mut index_rev: u32 = input_vec.len() as u32 - 1; // Index from right to left, we subtract by 1 because indexing in Rust starts at 0
    let mut index: u32 = 0; // Index from left to right

    // Once index_rev == index we finished sorting
    'outer: while index_rev != index {
        if input_vec[index_rev as usize] != empty {
            while input_vec[index as usize] != empty {
                // We also need this check because index can move past index_rev when sorting
                if index_rev == index {
                    break 'outer;
                };

                index += 1;
            }

            input_vec.swap(index_rev as usize, index as usize);
            index += 1;
        };

        index_rev -= 1;
    }

    input_vec.join(separator)
}
