use std::collections::HashSet;
use std::f64::consts::SQRT_2;
use std::fs::read_to_string;
use std::str::Split;

// const FILE_PATH: &str = "./inputs/day9.txt";
const FILE_PATH: &str = "./inputs/test9.txt";

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn determine_direction(dir: &str) -> Direction {
    match dir {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("string {dir} is invalid!"),
    }
}

fn collect_movement_data(input: Split<&str>) -> Vec<(Direction, i32)> {
    let mut data: Vec<(Direction, i32)> = Vec::new();

    for s in input {
        if s.is_empty() {
            continue;
        }
        let movement_data = s.split(" ").collect::<Vec<&str>>();

        let dir = determine_direction(movement_data[0]);
        let amount = movement_data[1].parse::<i32>().unwrap();
        data.push((dir, amount));
    }

    data
}

fn euclidean_2d(first: (i32, i32), second: (i32, i32)) -> f64 {
    /*
    Gets the 2d euclidean distance from two tuples
    */

    let xs = (second.0 - first.0).pow(2);
    let ys = (second.1 - first.1).pow(2);
    let total = (xs + ys) as f64;

    total.sqrt()
}

fn determine_movement_vector(dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (0, 1),
        Direction::Down => (0, -1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    }
}

pub fn unique_positions() -> i32 {
    let input = read_to_string("input.txt").unwrap();
    let data = collect_movement_data(input.split("\n"));

    let mut unique_pos: HashSet<(i32, i32)> = HashSet::new();

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut prev_head_pos = (0, 0);
    // iterates over all directions and amounts
    for (dir, n) in data {
        // println!("({:?}, {n})", dir);
        // determines the way to move
        let movement = determine_movement_vector(dir);
        for _ in 0..n {
            // if the distance between the two is greater than the SQRT(2)
            //  move the tail
            if euclidean_2d(head_pos, tail_pos) > SQRT_2 {
                tail_pos = prev_head_pos;
            }
            // adds the current tail position to the set
            unique_pos.insert(tail_pos);

            // moves the head
            prev_head_pos = head_pos;
            head_pos.0 += movement.0;
            head_pos.1 += movement.1;
            // println!("\tHead: {:?}\n\tPrev: {:?}\n\tTail: {:?}", head_pos, prev_head_pos, tail_pos);
        }

        // checks to move the tail position one last time and adds it to the set
        if euclidean_2d(head_pos, tail_pos) > SQRT_2 {
            tail_pos = prev_head_pos;
        }
        unique_pos.insert(tail_pos);
    }

    unique_pos.len() as i32
}

pub fn unique_positions_ten_knots() -> i32 {
    let input = read_to_string("input.txt").unwrap();
    let data = collect_movement_data(input.split("\n"));

    let mut unique_pos: HashSet<(i32, i32)> = HashSet::new();

    // creates a vector of 10 rope knots, with different positions and different previous positions.
    // [0] is the head, [9] is the tail
    let x = vec![((0, 0), (0, 0))];
    // Generate a vec of (current,previous) position tuples
    let mut rope_pos = x.into_iter().cycle().take(10).collect::<Vec<_>>();
    for (dir, n) in data {
        println!("Moving {} steps {:?}", n, dir);
        // determines the way to move
        let movement = determine_movement_vector(dir);
        println!("Moving head this way: {:?}", movement);
        for _ in 0..n {
            // Save the old position in previous
            rope_pos[0].1 = rope_pos[0].0;

            // Update current position
            rope_pos[0].0 .0 += movement.0;
            rope_pos[0].0 .1 += movement.1;
            println!(
                "Head Position updated: {:?}->{:?}",
                rope_pos[0].1, rope_pos[0].0
            );

            for i in 1..rope_pos.len() {
                if euclidean_2d(rope_pos[i].0, rope_pos[i - 1].0) > SQRT_2 {
                    rope_pos[i].1 = rope_pos[i].0;
                    rope_pos[i].0 = rope_pos[i - 1].1;
                    println!(
                        "Node {} is too far away and must move: {:?}->{:?}",
                        i, rope_pos[i].1, rope_pos[i].0
                    );
                }
            }
            println!("Processed follower nodes");
            // println!("{:?}", rope_pos);
            unique_pos.insert(rope_pos[9].0);
        }
    }

    unique_pos.len() as i32
}

fn main() {
    let out = unique_positions_ten_knots();
    println!("{}", out);
}
