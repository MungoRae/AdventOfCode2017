use std::env;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

const DEFAULT_FILENAME: &'static str = "../input";

#[macro_use]
extern crate lazy_static;
extern crate regex;

#[derive(Debug)]
struct Node {
    name: String,
    weight: usize,
    children: Vec<Node>,
}

lazy_static! {
        static ref RE: Regex = Regex::new(r"^([a-z]+)+\s\((\d+)\)").unwrap();
    }

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

fn parse(text: String) -> i32 {
    let original = text.trim();

    let mut lines: Vec<String> = original.split("\n")
        .map(|x| x.to_string())
        .collect();

    let bottom = get_bottom(&mut lines);

    println!("Bottom: {:?}", bottom);

    // get bottom node
    // if no children
    //   create node
    // if children
    //   loop over entries
    //   find child
    //

    let node = create_node(bottom, &mut lines);
    println!("NODE: {:?}", node);

    return get_weight_sum(node);
}

fn create_node(line: String, lines: &mut Vec<String>) -> Node {
    if has_children(&line) {
        let child_names = children(&line);
        let mut child_nodes: Vec<Node> = Vec::new();
        for child_name_text in child_names {
            let line = find_by_name(&child_name_text, lines);
            child_nodes.push(create_node(line, lines));
        }
        return node(&line, child_nodes);
    } else {
        return node(&line, Vec::new());
    }
}

fn has_children(line: &String) -> bool {
    return line.contains("->");
}

fn children(line: &String) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let halves: Vec<String> = line.split(" -> ")
        .map(|x| x.to_string())
        .collect();
    let children = halves.get(1).unwrap();
    let mut items = children.split(", ");
    while let Some(name) = items.next() {
        names.push(name.trim().to_string());
    }
    return names;
}

fn find_by_name(name: &String, lines: &mut Vec<String>) -> String {
    for i in 0..lines.len() {
        let item: String = lines.get(i).unwrap().clone();
        let captures = RE.captures(item.as_str()).unwrap();
        let name_match = captures.get(1).unwrap().as_str().to_string();
        if name_match == *name {
            return lines.remove(i);
        }
    }

    panic!("name not found in list of names");
}

fn node(line: &String, children: Vec<Node>) -> Node {
    //println!("line: {}", line);
    let captures = RE.captures(line).unwrap();
    let name: String = captures.get(1).unwrap().as_str().to_string();
    let number: usize = captures.get(2).unwrap().as_str().to_string().parse().unwrap();
    return Node {
        name,
        weight: number,
        children,
    };
}

fn get_bottom(lines: &Vec<String>) -> String {
    let mut copy = lines.clone();
    let mut line = copy.remove(0);

    loop {
        let halves: Vec<String> = line.split(" (")
            .map(|x| x.to_string())
            .collect();
        let name: String = halves.get(0).unwrap().clone();
        let newline = next_bottom(&name, &mut copy);
        if newline.is_none() {
            break;
        }
        line = newline.unwrap();
    }

    return line;
}

fn next_bottom(bottom: &String, lines: &mut Vec<String>) -> Option<String> {
    for i in 0..lines.len() {
        let line = lines[i].clone();
        if line.contains(bottom.as_str()) {
            lines.remove(i);
            return Some(line);
        }
    }

    return None;
}

fn get_weight_sum(node: Node) -> i32 {
    println!("Node: {}, weight: {}, children {}", node.name, node.weight, node.children.len());
    if node.children.len() > 0 {
        let mut weight: Option<i32> = None;
        let mut weights: Vec<i32> = Vec::new();
        for child in node.children {
            let child_weight: usize = child.weight;
            let child_weight_sum: i32 = get_weight_sum(child);
            weights.push(child_weight_sum);
            if weight.is_some() && weight.unwrap() != child_weight_sum {
                let diff: usize = (weight.unwrap() - child_weight_sum).abs() as usize;
                println!("Answer is: {}", child_weight - diff);
                println!("Answer is: {}", child_weight - diff);
                println!("Answer is: {}", child_weight - diff);
                println!("Answer is: {}", child_weight - diff);
                println!("Answer is: {}", child_weight - diff);
                println!();
                println!();
                println!();
                println!();
                panic!("Got answer")
            } else if weight.is_none() {
                weight = Some(child_weight_sum);
            }
        }
        return node_weight(node.weight as i32, weights);
    } else {
        return node.weight as i32;
    }
}

fn node_weight(node_weight: i32, child_weights: Vec<i32>) -> i32 {
    let sum: i32 = child_weights.iter().sum();
    return sum + node_weight;
}