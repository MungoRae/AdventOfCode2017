use std::env;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

const DEFAULT_FILENAME: &'static str = "../input";

extern crate regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = get_filename_from_args(&args);

    let contents = read_file(filename.unwrap_or(DEFAULT_FILENAME.to_string()));
    let sum = parse_part_one(contents);
    // let sum = parse_part_two(contents);
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

fn parse_part_one(text: String) -> u32 {
    let mut sum = 0;
    let original = text.trim();

    let re = Regex::new(r"\s+").unwrap();
    let mut result: Vec<usize> = re.split(original)
        .map(|t: &str| t.to_string().parse().unwrap())
        .collect();

    println!("Array {:?}", result);

    let mut signature_set = HashSet::new();
    loop {
        let highest = highest_index(&result);
        let value = remove_value(highest, &mut result);
        reassign(&mut result, value, highest);
        println!("value: {}, index: {}, new {:?}", value, highest, result);
        sum += 1;
        let success = signature_set.insert(signature(&result));
        if !success {
            break;
        }
    }


    return sum;
}

fn parse_part_two(text: String) -> usize {
    let mut sum = 0;
    let original = text.trim();

    let re = Regex::new(r"\s+").unwrap();
    let mut result: Vec<usize> = re.split(original)
        .map(|t: &str| t.to_string().parse().unwrap())
        .collect();

    println!("Array {:?}", result);

    let mut signature_set: HashMap<String, usize> = HashMap::new();
    loop {
        let highest = highest_index(&result);
        let value = remove_value(highest, &mut result);
        reassign(&mut result, value, highest);
        println!("value: {}, index: {}, new {:?}", value, highest, result);
        sum += 1;
        let signature = signature(&result);
        let old: Option<usize> = signature_set.insert(signature, sum);
        if old.is_some() {
            return sum - old.unwrap();
        }
    }


    panic!("Should never get here");
}

fn highest_index(array: &Vec<usize>) -> usize {
    let mut index: usize = 0;
    let mut highest = 0;
    let mut iter = array.iter();
    let mut i: usize = 0;
    while let Some(&d) = iter.next() {
        if d > highest {
            index = i;
            highest = d;
        }
        i += 1;
    }

    return index;
}

fn remove_value(index: usize, array: &mut Vec<usize>) -> usize {
    let number: usize;
    {
        number = array.remove(index);
    }
    {
        array.insert(index, 0);
    }

    return number;
}

fn reassign(array: &mut Vec<usize>, number: usize, start: usize) {
    let mut i = start;
    for _ in 0..number {
        if i == array.len() - 1 {
            i = 0;
        } else {
            i += 1;
        }
        let item: usize;
        {
            item = array.remove(i);
        }
        {
            array.insert(i, item + 1);
        }
    }
}

fn signature(array: &Vec<usize>) -> String {
    let signature_array: Vec<String> = array.iter()
        .map(|x: &usize|x.to_string())
        .collect();
    return signature_array.join("");
}