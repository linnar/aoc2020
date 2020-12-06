use std::collections::HashMap;
use std::fs;

fn main() {
    let mut any: usize = 0;
    let mut all: usize = 0;
    fs::read_to_string("inputs/input_d06.txt")
        .expect("File not found")
        .split("\n\n")
        .for_each(|x| {
            let mut counter = HashMap::new();
            let mut lines = 0;
            for line in x.trim().lines() {
                lines += 1;
                for c in line.chars() {
                    *counter.entry(c).or_insert(0) += 1;
                }
            }
            any += counter.len();
            all += counter.values().filter(|x| **x == lines).count();
        });

    println!("any {}", any);
    println!("all {}", all);
}
