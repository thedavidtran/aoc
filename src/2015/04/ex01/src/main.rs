extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let mut hasher = Md5::new();
    let key = "ckczppom";
    let mut num = 0;
    let target_prefix = "00000";
    loop {
        let input = key.to_owned() + &num.to_string();
        hasher.input_str(&input);
        let hash = hasher.result_str();
        if &hash[0..target_prefix.len()] == target_prefix {
            break;
        }
        num += 1;
        hasher.reset();
    }
    println!("{} found with num {}", target_prefix, num); // 117946
}
