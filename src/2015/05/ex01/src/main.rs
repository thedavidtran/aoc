use std::io::{self, Read};
use std::fs::File;
use lazy_static::lazy_static;
use regex::Regex;

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn has_at_least_three_vowels(s: &str) -> bool {
    lazy_static! {
        static ref re: Regex = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    }
    return re.is_match(s);
}

fn has_double_consecutive_letter(s: &str) -> bool {
    lazy_static! {
        static ref re: Regex = Regex::new(r"(aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz)").unwrap();
    }
    return re.is_match(s);
}

fn contains_pattern(s: &str) -> bool {
    // ab, cd, pq, or xy
    lazy_static! {
        static ref re: Regex = Regex::new(r"(ab|cd|pq|xy)").unwrap();
    }
    return re.is_match(s);
}

fn main() {
    println!("--- Day 5: Doesn't He Have Intern-Elves For This? ---");
    const FILE_PATH : &str = "./input.txt";
    // Read input.txt
    let input = filename_to_string(FILE_PATH);
    let mut nice_count = 0;
    input.unwrap().split("\n").for_each(|line| {
        println!("{line}");
        if has_double_consecutive_letter(line) && has_at_least_three_vowels(line) &&
            !contains_pattern(line) {
            nice_count += 1;
        }
    });

    println!("Number of nice strings: {nice_count}"); // 255
}