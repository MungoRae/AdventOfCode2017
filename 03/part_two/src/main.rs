use std::env;
use std::collections::VecDeque;

const CAPACITY: usize = 35;
const HALF: usize = (CAPACITY + 1) / 2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let number = number_from_args(&args).unwrap();

    let answer = create(number);
    println!("Answer: {}", answer);
}

fn number_from_args(args: &Vec<String>) -> Option<usize> {
    let a: Option<&String> = args.get(1);
    if a.is_some() {
        let b: usize = a.unwrap().parse().unwrap();
        return Some(b);
    }
    return None;
}

fn create(input: usize) -> usize {
    let mut columns: VecDeque<VecDeque<usize>> = VecDeque::with_capacity(CAPACITY);

    for _ in 0..CAPACITY {
        let mut rows = VecDeque::new();
        rows.resize(CAPACITY, 0);
        columns.push_back(rows);
    }

    let mut x: usize = HALF;
    let mut y: usize = HALF;
    let mut direction = Direction::DOWN;
    for _ in 0..CAPACITY * CAPACITY - 1 {
        let value = value(x, y, &columns);
        println!("Value: {}", value);

        {
            let column = columns.get_mut(x).unwrap();
            column.remove(y).unwrap();
            column.insert(y, value);

            if value > input {
                return value;
            }
        }

        let (newx, newy, new_direction) = next_coordinates(x, y, direction, &columns);
        x = newx;
        y = newy;
        direction = new_direction;

        println!("Next {},{}, dir {:?}", x, y, direction);
    }

    panic!("Should of found it by now");
}

fn value(x: usize, y: usize, table: &VecDeque<VecDeque<usize>>) -> usize {
    if x == HALF && y == HALF {
        return 1;
    }

    let mut sum = 0;
    sum += table.get(x - 1).unwrap().get(y + 1).unwrap();
    sum += table.get(x + 0).unwrap().get(y + 1).unwrap();
    sum += table.get(x + 1).unwrap().get(y + 1).unwrap();
    sum += table.get(x - 1).unwrap().get(y + 0).unwrap();
    sum += table.get(x + 0).unwrap().get(y + 0).unwrap();
    sum += table.get(x + 1).unwrap().get(y + 0).unwrap();
    sum += table.get(x - 1).unwrap().get(y - 1).unwrap();
    sum += table.get(x + 0).unwrap().get(y - 1).unwrap();
    sum += table.get(x + 1).unwrap().get(y - 1).unwrap();
    return sum;
}

fn next_coordinates(x: usize,
                    y: usize,
                    direction: Direction,
                    table: &VecDeque<VecDeque<usize>>) -> (usize, usize, Direction) {


    // get next direction
    // get coords for direction
    // if coords available
    //   return direction
    // else
    //   loop
    let next: Direction = next_direction(&direction);
    let (newx, newy): (usize, usize) = space(x, y, &next);
    if space_available(newx, newy, table) {
        return (newx, newy, next);
    } else {
        return next_coordinates(x, y, reverse_direction(direction), table);
    }
}

fn space(x: usize, y: usize, direction: &Direction) -> (usize, usize) {
    match direction {
        &Direction::LEFT => {
            println!("LEFT");
            return (x - 1, y);
        }
        &Direction::UP => {
            println!("UP");
            return (x, y + 1);
        }
        &Direction::RIGHT => {
            println!("RIGHT");
            return (x + 1, y);
        }
        &Direction::DOWN => {
            println!("DOWN");
            return (x, y - 1);
        }
    }
}

fn space_available(x: usize, y: usize, table: &VecDeque<VecDeque<usize>>) -> bool {
    return *table.get(x).unwrap().get(y).unwrap() == 0;
}

fn next_direction(current: &Direction) -> Direction {
    match current {
        &Direction::LEFT => return Direction::DOWN,
        &Direction::UP => return Direction::LEFT,
        &Direction::RIGHT => return Direction::UP,
        &Direction::DOWN => return Direction::RIGHT,
    }
}

fn reverse_direction(direction: Direction) -> Direction {
    match direction {
        Direction::DOWN => return Direction::LEFT,
        Direction::LEFT => return Direction::UP,
        Direction::UP => return Direction::RIGHT,
        Direction::RIGHT => return Direction::DOWN,
    }
}

#[derive(Debug)]
enum Direction {
    LEFT,
    UP,
    RIGHT,
    DOWN
}