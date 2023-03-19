use std::fs;
use serde_json::Value;
use serde_json::Value::{Number, String, Object, Array};

fn process(data: &str) -> i32 {
    let v: Value = serde_json::from_str(data).unwrap();
    match &v {
        Number(val) => {
            // println!("v is a number {:#?}", val);
            return serde_json::from_value(v).unwrap();
        },
        String(val) => {
            // println!("v is a string {:#?}", val);
            return 0;
        },
        Object(val) => {
            let mut result = 0;
            for key in v.as_object().unwrap().keys().collect::<Vec<_>>() {
                // println!("key? {:#?}", key);
                let value = v.get(key);
                match &value {
                    Some(val) => {
                        // println!("val is {:#?}", val);
                        match &val {
                            String(z) => {
                                // println!("is string {:#?}", z);
                                if z == "red" {
                                    return 0;
                                }
                            },
                            _ => {
                                let serialize = serde_json::to_string(&value).unwrap();
                                result += process(&serialize);
                            }
                        }
                    },
                    None => todo!()
                }
            }
            return result;
        },
        Array(val) => {
            // println!("v is an array {:#?}", val);
            let mut result = 0;
            for value in val.iter() {
                // println!("element is {}", &value);
                let serialize = serde_json::to_string(&value).unwrap();
                result += process(&serialize);
            }
            return result;
        }
        _ => {
            println!("v is unknown type");
            return 0;
        }
    };
}

fn main() {
    println!("--- Day 12: JSAbacusFramework.io ---");
    println!("--- Part 2 ---");
    const FILE_PATH : &str = "./input.txt";
    // Read input.txt
    let input = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");
    let mut total: i32 = 0;
    total = process(&input);
    println!("Total: {}", total); // 96852
}