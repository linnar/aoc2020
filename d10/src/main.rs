use std::{collections::HashMap, fs};

fn main() {
    let values: Vec<i64> = fs::read_to_string("inputs/input_d10.txt")
        .expect("File not found")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut sorted = values.clone();
    sorted.sort();
    sorted.insert(0, 0);
    sorted.insert(sorted.len(), sorted.last().unwrap_or(&0) + 3);

    let mut counter: HashMap<i64,i64> = HashMap::new();
    let diffs: Vec<i64> = sorted.windows(2).map(|x| x[1] - x[0]).collect();
    diffs.iter().for_each(|&x| *counter.entry(x).or_insert(0) += 1);
    
    println!("checksum {:?}", counter.get(&1).unwrap_or(&0) * counter.get(&3).unwrap_or(&0));

    let mut chain_counter: HashMap<i64,i64> = HashMap::new();
    let mut chain = -1;
    for i in diffs {
        if i == 1 {
            chain += 1;
        }
        if i == 3 {
            *chain_counter.entry(chain).or_insert(0) += 1;
            chain = -1;
        }
    }
    *chain_counter.entry(chain).or_insert(0) += 1;

    let count: i64 = chain_counter.iter().filter(|&(&chain, _)| chain > 0).map(|(&chain, &count)| {
        let res: i64 = if chain == 1 {
            2
        } else if chain == 2 {
            4
        } else if chain == 3 {
            7
        } else {
            panic!("unexpected chain length");
        };
        res.pow(count as u32)
    }).fold(1i64, |a, b| a * b);

    println!("count {}", count);
}
