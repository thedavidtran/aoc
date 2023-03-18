use std::io::{self, Read};
use std::fs::File;
use lazy_static::lazy_static;
use fancy_regex::Regex;
use std::collections::HashMap;

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

#[derive(Debug,Clone)]
enum Operator {
    INIT,
    NOT,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    ECHO
}

#[derive(Debug,Clone)]
struct Wire {
    name: String,
    op: Operator,
    inputs: Vec<String>,
    val: Option<u16>
}

impl Wire {
    // Returns true if all inputs have values resolved on the value_map.
    fn can_evaluate(&self, values_map: &HashMap<String, u16>) -> bool {
        for input in self.inputs.iter() {
            // println!("{:#?}", input);
            let val = values_map.get(input);
            if val.is_none() {
                return false;
            }
        }
        true
    }

    fn evaluate(&self, values_map: &HashMap<String, u16>) -> u16 {
        let get_value = |index: usize| -> u16 {
            let source_value = values_map.get(self.inputs.get(index).unwrap()).unwrap();
            *source_value
        };
        return match self.op {
            Operator::INIT => {
               self.val.unwrap()
            },
            Operator::NOT => {
                let source_value = get_value(0);
                println!("NOT {:#?} -> {:#?}", source_value, !source_value);
                !source_value
            }
            Operator::AND => {
                return if self.inputs.len() == 2 {
                    let source_a_value = get_value(0);
                    let source_b_value = get_value(1);
                    println!("AND {:#?} {:#?} -> {:#?}", source_a_value, source_b_value, source_a_value & source_b_value);
                    source_a_value & source_b_value
                } else {
                    let source_value = get_value(0);
                    let value = self.val.unwrap();
                    println!("AND_1 {:#?} {:#?} -> {:#?}", source_value, value, source_value & value);
                    source_value & value
                }
            },
            Operator::OR => {
                let source_a_value = get_value(0);
                let source_b_value = get_value(1);
                println!("OR {:#?} {:#?} -> {:#?}", source_a_value, source_b_value, source_a_value | source_b_value);
                source_a_value | source_b_value
            },
            Operator::RSHIFT => {
                let source_value = get_value(0);
                println!("RSHIFT {:#?} {:#?} -> {:#?}", source_value, self.val.unwrap(), source_value >> self.val.unwrap());
                source_value >> self.val.unwrap()
            },
            Operator::LSHIFT => {
                let source_value = get_value(0);
                println!("LSHIFT {:#?} {:#?} -> {:#?}", source_value, self.val.unwrap(), source_value << self.val.unwrap());
                source_value << self.val.unwrap()
            },
            Operator::ECHO => {
                let source_value = get_value(0);
                println!("ECHO {:#?} -> {:#?}", source_value, source_value);
                source_value
            }
        }
    }
}

fn parse(s: &str) -> Wire {
    lazy_static! {
        static ref RE_INIT: Regex = Regex::new(r"^(?<val>(\d)+) -> (?<wire>([a-z])+)").unwrap();
        static ref RE_NOT: Regex = Regex::new(r"^NOT (?<input>([a-z])+) -> (?<wire>([a-z])+)").unwrap();
        static ref RE_AND: Regex = Regex::new(r"^(?<inputa>([a-z])+) AND (?<inputb>([a-z])+) -> (?<wire>([a-z])+)").unwrap();
        static ref RE_AND_1: Regex = Regex::new(r"^(?<val>(\d)+) AND (?<input>([a-z])+) -> (?<wire>([a-z])+)").unwrap();
        static ref RE_OR: Regex = Regex::new(r"^(?<inputa>([a-z])+) OR (?<inputb>([a-z])+) -> (?<wire>([a-z])+)").unwrap();
        static ref RE_LSHIFT: Regex = Regex::new(r"^(?<input>([a-z])+) LSHIFT (?<val>(\d)+) -> (?<wire>([a-z])+)").unwrap();
        static ref RE_RSHIFT: Regex = Regex::new(r"^(?<input>([a-z])+) RSHIFT (?<val>(\d)+) -> (?<wire>([a-z])+)").unwrap();
        static ref RE_ECHO: Regex = Regex::new(r"^(?<input>([a-z])+) -> (?<wire>([a-z])+)").unwrap();
    }
    if RE_INIT.is_match(s).unwrap() {
        let binding = RE_INIT.captures(s);
        let captures = binding.as_ref().unwrap();
        let wire: &str = captures.as_ref().expect("Matches").name("wire").expect("").as_str();
        let val: u16 = captures.as_ref().expect("Matches").name("val").expect("").as_str().trim().parse().unwrap();
        return Wire {
            name: String::from(wire),
            op: Operator::INIT,
            inputs: vec![],
            val: Some(val)
        };
    } else if RE_NOT.is_match(s).unwrap() {
        let binding = RE_NOT.captures(s);
        let captures = binding.as_ref().unwrap();
        let wire: &str = captures.as_ref().expect("Matches").name("wire").expect("").as_str();
        let input: &str = captures.as_ref().expect("Matches").name("input").expect("").as_str();
        return Wire {
            name: String::from(wire),
            op: Operator::NOT,
            inputs: vec![String::from(input)],
            val: None
        };
    } else if RE_AND.is_match(s).unwrap() {
        let binding = RE_AND.captures(s);
        let captures = binding.as_ref().unwrap();
        let wire: &str = captures.as_ref().expect("Matches").name("wire").expect("").as_str();
        let inputa: &str = captures.as_ref().expect("Matches").name("inputa").expect("").as_str();
        let inputb: &str = captures.as_ref().expect("Matches").name("inputb").expect("").as_str();
        return Wire {
            name: String::from(wire),
            op: Operator::AND,
            inputs: vec![String::from(inputa), String::from(inputb)],
            val: None
        };
    } else if RE_AND_1.is_match(s).unwrap() {
        let binding = RE_AND_1.captures(s);
        let captures = binding.as_ref().unwrap();
        let wire: &str = captures.as_ref().expect("Matches").name("wire").expect("").as_str();
        let input: &str = captures.as_ref().expect("Matches").name("input").expect("").as_str();
        let val: u16 = captures.as_ref().expect("Matches").name("val").expect("").as_str().trim().parse().unwrap();
        return Wire {
            name: String::from(wire),
            op: Operator::AND,
            inputs: vec![String::from(input)],
            val: Some(val)
        }
    } else if RE_OR.is_match(s).unwrap() {
        let binding = RE_OR.captures(s);
        let captures = binding.as_ref().unwrap();
        let wire: &str = captures.as_ref().expect("Matches").name("wire").expect("").as_str();
        let inputa: &str = captures.as_ref().expect("Matches").name("inputa").expect("").as_str();
        let inputb: &str = captures.as_ref().expect("Matches").name("inputb").expect("").as_str();
        return Wire {
            name: String::from(wire),
            op: Operator::OR,
            inputs: vec![String::from(inputa), String::from(inputb)],
            val: None
        };
    } else if RE_LSHIFT.is_match(s).unwrap() {
        let binding = RE_LSHIFT.captures(s);
        let captures = binding.as_ref().unwrap();
        let wire: &str = captures.as_ref().expect("Matches").name("wire").expect("").as_str();
        let input: &str = captures.as_ref().expect("Matches").name("input").expect("").as_str();
        let val: u16 = captures.as_ref().expect("Matches").name("val").expect("").as_str().trim().parse().unwrap();
        return Wire {
            name: String::from(wire),
            op: Operator::LSHIFT,
            inputs: vec![String::from(input)],
            val: Some(val)
        };
    } else if RE_RSHIFT.is_match(s).unwrap() {
        let binding = RE_RSHIFT.captures(s);
        let captures = binding.as_ref().unwrap();
        let wire: &str = captures.as_ref().expect("Matches").name("wire").expect("").as_str();
        let input: &str = captures.as_ref().expect("Matches").name("input").expect("").as_str();
        let val: u16 = captures.as_ref().expect("Matches").name("val").expect("").as_str().trim().parse().unwrap();
        return Wire {
            name: String::from(wire),
            op: Operator::RSHIFT,
            inputs: vec![String::from(input)],
            val: Some(val)
        };
    } else if RE_ECHO.is_match(s).unwrap() {
        let binding = RE_ECHO.captures(s);
        let captures = binding.as_ref().unwrap();
        let wire: &str = captures.as_ref().expect("Matches").name("wire").expect("").as_str();
        let input: &str = captures.as_ref().expect("Matches").name("input").expect("").as_str();
        return Wire {
            name: String::from(wire),
            op: Operator::ECHO,
            inputs: vec![String::from(input)],
            val: None
        };

    }
    println!("------ THIS SHOULD NOT HAPPEN {}", s);
    Wire {
        name: String::from("test"),
        op: Operator::AND,
        inputs: vec![],
        val: None
    }
}

fn main() {
    println!("--- Day 7: Some Assembly Required ---");
    println!("--- Part 2 ---");
    const FILE_PATH : &str = "./input.txt";
    // Read input.txt
    let input = filename_to_string(FILE_PATH);
    let mut value = 0;
    let mut wires: HashMap<String, Wire> = HashMap::new(); // map of wires to evaluate
    let mut values_map: HashMap<String, u16> = HashMap::new();
    input.unwrap().split("\n").for_each(|line| {
        println!("{line}");
        let wire = parse(line);
        if wire.val.is_some() {
            println!("{:#?}", wire.val.unwrap());
        }
        wires.insert(wire.name.clone(), wire.clone());
    });
    // OVERRIDE wire b with 3176
    let wire = wires.get("b").unwrap();
    let override_b = Wire {
        val: Some(3176),
        ..wire.clone()
    };
    wires.insert(override_b.name.clone(), override_b);

    while wires.len() > 0 {
        for (name, wire) in wires.clone().iter() {
            println!("{:#?} can evaluate {:#?}", name, wire.can_evaluate(&values_map));
            if wire.can_evaluate(&values_map) {
                values_map.insert(name.to_string(), wire.evaluate(&values_map));
                wires.remove(name);
            }
        }
        println!("wires size {} value map size {}", wires.len(), values_map.len());
    }
    println!("value map is {:#?}", values_map);
    println!("wire a value is {:#?}", values_map.get("a").unwrap()); // 14710
}