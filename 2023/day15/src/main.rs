#![feature(ascii_char)]

use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct LensBoxes {
    buckets: Vec<VecDeque<(String, u8)>>,
}
impl LensBoxes {
    fn new() -> LensBoxes {
        let mut arr = Vec::with_capacity(u8::MAX as usize + 1);
        for _ in 0..=(u8::MAX as usize) {
            arr.push(VecDeque::new())
        }
        LensBoxes { buckets: arr }
    }

    fn set(&mut self, label: String, focal_length: u8) {
        let hash = self.hash(&label);
        let bucket = &mut self.buckets[hash as usize];
        if let Some(idx) = bucket.iter().position(|e| e.0 == label) {
            bucket[idx] = (label, focal_length);
        } else {
            bucket.push_back((label, focal_length));
        }
    }

    fn remove(&mut self, label: String) {
        let hash = self.hash(&label);
        let bucket = &mut self.buckets[hash as usize];
        if let Some(idx) = bucket.iter().position(|e| e.0 == label) {
            bucket.remove(idx);
        }
    }
    fn hash(&self, input: &str) -> u8 {
        input
            .chars()
            .map(|c| c.as_ascii().expect("Received non-ascii character").to_u8())
            .fold(0, |acc, elem| {
                u8::try_from(((acc as usize + usize::from(elem)) * 17) % (u8::MAX as usize + 1))
                    .expect("Too large of a number!")
            })
    }
    fn apply_instruction(&mut self, input: &str) {
        if input.contains('=') {
            let (label, focal_length) = {
                let a = input.split_once('=').expect("Malformed input!");
                (a.0, a.1.parse::<u8>().expect("Focal length invalid!"))
            };
            self.set(label.to_string(), focal_length);
        } else if input.contains('-') {
            let label = input.strip_suffix('-').expect("Malformed input");
            self.remove(label.to_string());
        }
    }
    fn apply_instructions(&mut self, input: &str) {
        for action in input.lines().next().expect("No input?").split(',') {
            self.apply_instruction(action);
        }
    }
    fn focusing_power(&self) -> usize {
        self.buckets
            .iter()
            .enumerate()
            .map(|(bucket_num, bucket)| {
                bucket
                    .iter()
                    .enumerate()
                    .map(|(lens_slot, lens)| (1 + bucket_num) * (lens_slot + 1) * lens.1 as usize)
                    .sum::<usize>()
            })
            .sum()
    }
}

// part1
fn hash(input: &str) -> u8 {
    input
        .chars()
        .map(|c| c.as_ascii().expect("Received non-ascii character").to_u8())
        .fold(0, |acc, elem| {
            u8::try_from(((acc as usize + usize::from(elem)) * 17) % (u8::MAX as usize + 1))
                .expect("Too large of a number!")
        })
}
fn hash_all(input: &str) -> u64 {
    input
        .lines()
        .next()
        .expect("not a single line?")
        .split(',')
        .map(|str| hash(str) as u64)
        .sum()
}

fn main() {
    let test = include_str!("test.txt");
    let input = include_str!("input.txt");

    println!("Test sum: {}", hash_all(test));
    println!("Input sum: {}", hash_all(input));

    let mut test_map = LensBoxes::new();
    test_map.apply_instructions(test);
    println!("Test focusing power: {}", test_map.focusing_power());

    let mut input_map = LensBoxes::new();
    input_map.apply_instructions(input);
    println!("Input focusing power: {}", input_map.focusing_power());
}
