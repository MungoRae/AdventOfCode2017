use std::env;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

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
        let mut result: Vec<u32> = re.split(line)
            .map(|t| t.parse().unwrap())
            .collect();
        result.sort_by(|a, b| a.cmp(b));

        println!("Sorted {:?}", result);

        let mut divisors = result.iter().fuse();
        'divisors: while let Some(a) = divisors.next() {
            println!("a: {}", a);
            let mut other = result.iter().rev().fuse();
            while let Some(b) = other.next() {
                println!("b: {}", b);
                if a == b {
                    break;
                }

                if b % a == 0 {
                    sum += b / a;
                    break 'divisors;
                }
            }
        }
    }

    return sum;
}