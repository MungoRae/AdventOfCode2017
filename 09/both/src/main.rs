use std::env;
use std::fs::File;
use std::io::prelude::*;

const DEFAULT_FILENAME: &'static str = "../input";

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = get_filename_from_args(&args);

    let contents = read_file(filename.unwrap_or(DEFAULT_FILENAME.to_string()));
    let sum = parse(contents);
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

fn parse(stream: String) -> i32 {
    let mut it = stream.chars();
    let mut total = 0;
    let mut level = 0;
    let mut cancel = false;
    let mut comment = false;
    let mut comment_count = 0;
    while let Some(c) = it.next() {
        if cancel {
            cancel = false;
        } else if comment {
            if c == '!' {
                cancel = true;
            } else if c == '>' {
                comment = false;
            } else {
                comment_count += 1;
            }
        } else if c == '<' {
            comment = true;
        } else if c == '{' {
            level += 1;
            total += level;
        } else if c == '}' {
            level -= 1;
        }
    }

    println!("Comment count: {}", comment_count);
    return total;
}
