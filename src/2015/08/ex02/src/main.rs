use std::io::{self, Read};
use std::fs::File;
use lazy_static::lazy_static;
use fancy_regex::Regex;

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

#[derive(Debug)]
struct Count {
    string: usize, // count of length in code
    memory: usize, // count of length in memory
    encode: usize // count of length of encoded string
}

fn parse(s: &str) -> Count {
    lazy_static! {
        static ref RE_BACKSLASH: Regex = Regex::new(r"[\\]{2}").unwrap();
        static ref RE_DOUBLEQUOTE: Regex = Regex::new(r#"\\\""#).unwrap();
        static ref RE_HEXIDECIMAL: Regex = Regex::new(r"\\x[\h]{2}").unwrap();
    }
    let mut trimmed = s.trim();
    let len = trimmed.len();
    let mut memory = len;
    let trimmed_no_doublequotes = &trimmed[1..len-1];
    memory -= 2;
    // count
    // - \\ -> \
    // - \" -> "
    // - \x## -> single character with that ASCII code, where # -> 0-9a-f
    if RE_BACKSLASH.is_match(trimmed_no_doublequotes).unwrap() {
        let backslash_count = RE_BACKSLASH.find_iter(trimmed_no_doublequotes).count();
        println!("{} has backslash counts {}", trimmed_no_doublequotes, backslash_count);
        memory -= backslash_count;
    }
    if RE_DOUBLEQUOTE.is_match(trimmed_no_doublequotes).unwrap() {
        let doublequote_count = RE_DOUBLEQUOTE.find_iter(trimmed_no_doublequotes).count();
        println!("{} has double quote counts {}", trimmed_no_doublequotes, doublequote_count);
        memory -= doublequote_count;
    }
    if RE_HEXIDECIMAL.is_match(trimmed_no_doublequotes).unwrap() {
        let hexidecimal_count = RE_HEXIDECIMAL.find_iter(trimmed_no_doublequotes).count();
        println!("{} has hexidecimal counts {}", trimmed_no_doublequotes, hexidecimal_count);
        memory -= hexidecimal_count * 3;
    }
    let mut encoded = trimmed.replace("\\", "\\\\");
    encoded = encoded.replace("\"", "\\\"");
    encoded = format!("\"{}\"", encoded);
    println!("encoded {}", encoded);
    // encode
    return Count {
        string: len,
        memory,
        encode: encoded.len()
    }
}

fn main() {
    println!("--- Day 8: Matchsticks ---");
    println!("--- Part 2 ---");
    const FILE_PATH : &str = "./input.txt";
    // Read input.txt
    let input = filename_to_string(FILE_PATH);
    let mut total_string = 0;
    let mut total_memory = 0;
    let mut total_encoded = 0;
    input.unwrap().split("\n").for_each(|line| {
        println!("{line}");
        let count = parse(line);
        total_string += count.string;
        total_memory += count.memory;
        total_encoded += count.encode;
    });
    println!("total string: {}, total memory: {}, total encoded: {}", total_string, total_memory, total_encoded);
    println!("difference: {}", total_string - total_memory); // 1371
    println!("encoded - string: {}", total_encoded - total_string); // 2117
}