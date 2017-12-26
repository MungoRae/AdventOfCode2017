use std::env;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;

const DEFAULT_FILENAME: &'static str = "../input";

#[macro_use]
extern crate lazy_static;
extern crate regex;

lazy_static! {
        static ref RE: Regex = Regex::new(r"^([a-z]+)\s(inc|dec)\s([-0-9]+)\sif\s([a-z]+)\s([<>=!]+)\s([-0-9]+)$").unwrap();
    }

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = get_filename_from_args(&args);

    let contents = read_file(filename.unwrap_or(DEFAULT_FILENAME.to_string()));
    let sum = parse(text_to_lines(contents));
    println!("Answer is {}", sum);
}

fn get_filename_from_args(args: &Vec<String>) -> Option<String> {
    let a: Option<&String> = args.get(1);
    if a.is_some() {
        let b: &String = a.unwrap();
        return Some(b.clone());
    }
    return None;
}

fn read_file(filename: String) -> String {
    let mut contents = String::new();
    File::open(filename)
        .expect("file not found")
        .read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents.trim().to_string();
}

fn text_to_lines(text: String) -> Vec<String> {
    return text.split("\n")
        .map(|x| x.to_string())
        .collect();
}

fn parse(lines: Vec<String>) -> i32 {
    let mut it = lines.iter().fuse();
    let mut registers: HashMap<&str, i32> = HashMap::new();
    let mut max: i32 = 0;
    while let Some(line) = it.next() {
        let result = RE.captures(line).unwrap();
        println!("Result: {:?}", result);

        let register = result.get(1).unwrap().as_str();
        println!("Register: {}", register);
        if !registers.contains_key(register) {
            registers.insert(register, 0);
        }

        let inc = result.get(2).unwrap().as_str();
        let amount: i32 = result.get(3).unwrap().as_str().parse().unwrap();
        let change: i32 = get_change(inc, amount);
        println!("Change: {}", change);

        let comparator = result.get(4).unwrap().as_str();
        println!("comparator: {}", comparator);

        if !registers.contains_key(comparator) {
            registers.insert(comparator, 0);
        }

        let operation = result.get(5).unwrap().as_str();
        let check: i32 = result.get(6).unwrap().as_str().parse().unwrap();
        println!("Operation: {}, Check: {}", operation, check);

        let valid = compute(operation, check, comparator, &registers);
        println!("Valid: {}", valid);

        if valid {
            let current: i32 = *(registers.get(register).unwrap());
            registers.insert(register, current + change);
            println!("Updated {} by {}", register, change);
        }

        let current_max = *(registers.values().max().unwrap());
        if current_max > max {
            max = current_max;
        }
    }

    println!("All time max {}", max);

    return *(registers.values().max().unwrap());
}

fn get_change(inc: &str, amount: i32) -> i32 {
    if inc == "dec" {
        return -amount;
    }

    return amount;
}

fn compute(operation: &str, check: i32, register: &str, registers: &HashMap<&str, i32>) -> bool {
    match operation {
        ">" => {
            return *(registers.get(register).unwrap()) > check;
        },
        "<" => {
            return *(registers.get(register).unwrap()) < check;
        },
        ">=" => {
            return *(registers.get(register).unwrap()) >= check;
        },
        "<=" => {
            return *(registers.get(register).unwrap()) <= check;
        },
        "!=" => {
            return *(registers.get(register).unwrap()) != check;
        },
        "==" => {
            return *(registers.get(register).unwrap()) == check;
        },
        _ => {
            panic!("{} Not implemented yet", operation);
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_change_returns_negative_for_dec_value_and_positive_amount() {
        assert_eq!(-45, get_change("dec", 45))
    }

    #[test]
    fn get_change_returns_positive_for_dec_value_and_negative_amount() {
        assert_eq!(36, get_change("dec", -36))
    }

    #[test]
    fn get_change_returns_negative_for_inc_value_and_negative_amount() {
        assert_eq!(-26, get_change("inc", -26))
    }

    #[test]
    fn get_change_returns_positive_for_inc_value_and_positive_amount() {
        assert_eq!(36, get_change("inc", 36))
    }
}

