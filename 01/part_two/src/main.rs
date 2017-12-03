use std::env;
use std::fs::File;
use std::io::prelude::*;

const DEFAULT_FILENAME: &'static str = "../input";

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = get_filename_from_args(&args);

    let contents = read_file(filename.unwrap_or(DEFAULT_FILENAME.to_string()));
    println!("Contents is: {}", contents);
    let sum = parse(contents);
    println!("Sum is {}", sum);
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

    return contents;
}

fn parse(text: String) -> u32 {
    let mut sum = 0;

    let trimmed = text.trim().to_string();
    let length = trimmed.to_string().len();
    let mut top = trimmed.chars();
    let mut bottom = trimmed.chars();

    bottom.nth(length / 2 - 1);
    while let Some(c) = top.next() {
        let is_end = bottom.next();
        if is_end.is_none() {
            break;
        }
        let c2 = is_end.unwrap();
        if c == c2 {
            sum += c.to_digit(10).unwrap();
        }
    }

    return sum * 2;
}