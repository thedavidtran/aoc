use std::fs;

fn main() {
    println!("--- Day 1: Not Quite Lisp ---");
    println!("--- Part 2 ---");

    // Approach #1: Use a counter, increment count if '(' and decrement otherwise.
    // - Store the value of index of enumeration of chars the first time count goes negative

    const FILE_PATH : &str = "./input.txt";
    // Read input.txt
    let input = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");
    println!("Input from '{FILE_PATH}':\n{input}");

    let mut count : i32 = 0;
    let mut entered_basement_first_time : usize = usize::MIN; // start usize::MIN to indicate it hasn't entered the basement initially
    for (pos, c) in input.chars().enumerate() {
        if c == '(' {
            count += 1;
        } else {
            count -= 1;
        }
        if entered_basement_first_time == usize::MIN && count < 0 {
           entered_basement_first_time = pos + 1; // We add one as pos is indexed 0
        }
    }
    println!("Santa entered the basement for the first time at position {entered_basement_first_time}"); // 1783
    println!("Santa is on floor {count}"); // 232
}
