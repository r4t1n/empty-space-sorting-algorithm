pub fn sort(input: String) -> String {
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
        };

        index_rev -= 1;
    }

    input_vec.join(separator)
}