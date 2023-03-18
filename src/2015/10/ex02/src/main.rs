fn look_and_say(s: &str) -> String {
    let mut cur = s.chars().nth(0).unwrap();
    let mut count = 0;
    let mut result = String::new();
    for c in s.chars() {
        if cur == c {
            count += 1;
        } else {
            result = format!("{}{}{}", result, count, cur);
            cur = c;
            count = 1;
        }
    }
    result = format!("{}{}{}", result, count, cur);
    return result;
}

fn main() {
    const INPUT: &str = "1321131112";
    const ITERATIONS: i32 = 50;
    let mut input = String::from(INPUT);
    for i in 0..ITERATIONS {
        println!("Iteration {}", i);
        input = look_and_say(&input);
    }
    println!("{}", input);
    println!("length of output is: {}", input.len()); // 6989950
}