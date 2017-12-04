use std::env;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashSet;

const DEFAULT_FILENAME: &'static str = "../input";

extern crate regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = get_filename_from_args(&args);

    let contents = read_file(filename.unwrap_or(DEFAULT_FILENAME.to_string()));
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
    let original = text.trim().to_string();
    let mut split = original.split("\n");

    'line: while let Some(line) = split.next() {
        let re = Regex::new(r"\s+").unwrap();
        let mut result:regex::Split = re.split(line);

        let mut set: HashSet<String> = HashSet::new();
        let mut fail: bool = false;
        while let Some(word) = result.next() {
            if !set.insert(word.to_string()) {
                fail = true;
            }
        }

        if !fail {
            sum += 1;
        }
    }

    return sum;
}