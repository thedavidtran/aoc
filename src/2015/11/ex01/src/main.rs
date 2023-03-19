use lazy_static::lazy_static;
use fancy_regex::Regex;

fn next_password(s: &str) -> String {
    let tmp = s.clone();
    let rev = tmp.chars().rev().collect::<String>();
    let mut result: Vec<u8> = vec![];
    let mut carry = 1;
    for b in rev.as_bytes() {
        if carry == 1 {
            let r = b + carry;
            if r > 122 {
                carry = 1;
                result.push(97); // wrap back to 'a'
            } else {
                carry = 0;
                result.push(r);
            }
        } else {
            result.push(*b);
        }
    }
    String::from_utf8_lossy(&result).chars().rev().collect::<String>()
}

fn validate_password(s: &str) -> bool {
    lazy_static! {
        static ref RE_HAS_STRAIGHT: Regex = Regex::new(r"(abc|bcd|cde|def|efg|fgh|ghi|hij|ijk|jkl|klm|lmn|mno|nop|opq|pqr|qrs|rst|stu|tuv|uvw|vwx|wxy|xyz)").unwrap();
        static ref RE_HAS_INVALID_CHARS: Regex = Regex::new(r"[iol]").unwrap();
        static ref RE_HAS_DOUBLE_PAIR: Regex = Regex::new(r"([a-z])\1.*?([a-z])\2").unwrap();
    }
    println!("validating {}", s);
    if !RE_HAS_STRAIGHT.is_match(s).unwrap() {
        return false;
    }
    if RE_HAS_INVALID_CHARS.is_match(s).unwrap() {
        return false;
    }
    if !RE_HAS_DOUBLE_PAIR.is_match(s).unwrap() {
        return false;
    }
    true
}

fn main() {
    const INPUT: &str = "cqjxjnds";
    println!("--- Day 11: Corporate Policy ---");
    println!("Prev password: {}", INPUT);
    println!("{}", next_password(INPUT));
    let mut pwd = String::from(INPUT);
    while !validate_password(&pwd) {
       pwd = next_password(&pwd);
    }
    println!("next password should be: {}", &pwd); // cqjxxyzz
}
