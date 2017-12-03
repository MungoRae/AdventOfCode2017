use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut contents = String::new();
    File::open("input")
        .expect("file not found")
        .read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    parse(contents);
}

fn parse(text: String) {
    let mut sum = 0;

    let first = text.chars().nth(0).unwrap().to_digit(10);
    let mut chars = text.chars().fuse();
    let mut next = text.chars();
    next.next();
    while let Some(x) = chars.next() {
        let possible_x = x.to_digit(10);
        if possible_x.is_some() {
            let digit = possible_x.unwrap();
            let possible_y = next.next();
            if possible_y.is_some() {
                let compare = possible_y.unwrap().to_digit(10).or(first).unwrap();
                if digit == compare {
                    sum += digit;
                }
            }
        }
    }

    println!("Final Sum: {}", sum);
}