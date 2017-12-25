use std::env;
use std::fs::File;
use std::io::prelude::*;

const DEFAULT_FILENAME: &'static str = "../input";

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

    let mut array: Vec<i32> = Vec::new();
    let mut split = original.split("\n");

    while let Some(line) = split.next() {
        let number: i32 = line.to_string().parse().unwrap();
        array.push(number);
    }

    let mut index: i32 = 0;
    loop {
        sum += 1;
        let item: i32;
        {
            item = array.remove(index as usize);
        }
        {
            let increase: i32;
            if item >= 3 {
                increase = -1;
            } else {
                increase = 1;
            }
            array.insert(index as usize, item + increase);
        }
        index += item;
        if index >= array.len() as i32 || index < 0 {
            return sum;
        }
    }
}