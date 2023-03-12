use std::fs;

fn main() {
    println!("--- Day 1: Not Quite Lisp ---");

    // Approach #1: Use a counter, increment count if '(' and decrement otherwise.

    const FILE_PATH : &str = "./input.txt";
    // Read input.txt
    let input = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");
    println!("Input from '{FILE_PATH}':\n{input}");

    let mut count : i32 = 0;
    for c in input.chars() {
        if c == '(' {
            count += 1;
        } else {
            count -= 1;
        }
    }
    println!("Santa is on floor {count}"); // 232
}
