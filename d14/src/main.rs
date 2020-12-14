use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fs;

#[derive(Clone, Copy)]
enum Input {
    Command { address: u64, value: u64 },
    Mask { mask: u64, value: u64 },
}

impl fmt::Debug for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Input::Command { address, value } => write!(
                f,
                "Command {{ address = {:#038b}, value = {:#038b}}}",
                address, value
            ),
            Input::Mask { mask, value } => write!(
                f,
                "Mask {{ mask = {:#038b}, value = {:#038b}}}",
                mask, value
            ),
        }
    }
}

fn generate_floating(slice: &[u64]) -> Vec<u64> {
    fn helper(slice: &[u64], acc: Vec<u64>) -> Vec<u64> {
        if let Some((first, rest)) = slice.split_first() {
            let mut vec = Vec::new();
            acc.iter().for_each(|a| {
                vec.push(a | first); // set bit to 1
                vec.push(a.clone()); // set bit to 0
            });
            if rest.is_empty() {
                vec
            } else {
                helper(rest, vec)
            }
        } else {
            acc
        }
    }
    helper(slice, vec![0])
}

fn main() {
    use crate::Input::*;

    let bit_len = 36;
    let re_command = Regex::new(r"^mem\[(\d*)\] = (\d*)").unwrap();
    let re_mask = Regex::new(r"^mask = (.*)").unwrap();
    let input: Vec<Input> = fs::read_to_string("inputs/input_d14.txt")
        .expect("File not found")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            if let Some(cap) = re_command.captures(line) {
                let address = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
                let value = u64::from_str_radix(cap.get(2).unwrap().as_str(), 10).unwrap();
                return Command { address, value };
            }

            if let Some(cap) = re_mask.captures(line) {
                let mut mask = 0u64;
                let mut value = 0u64;
                for (i, c) in cap.get(1).unwrap().as_str().chars().rev().enumerate() {
                    if c == '0' || c == '1' {
                        mask += 1 << i;
                        if c == '1' {
                            value |= 1 << i;
                        }
                    }
                }
                return Mask { mask, value };
            }

            panic!("unexpected input");
        })
        .collect();

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut active_mask = 0u64;
    let mut active_value = 0u64;

    input.iter().for_each(|input| {
        match input {
            Mask { mask, value } => {
                active_mask = mask.clone();
                active_value = value.clone();
            }
            Command { address, value } => {
                memory.insert(
                    address.clone(),
                    (active_mask & active_value) | (!active_mask & value),
                );
            }
        };
    });

    println!("memory {:?}", memory.values().sum::<u64>());

    let mut memory: HashMap<u64, u64> = HashMap::new();
    input.iter().for_each(|input| {
        match input {
            Mask { mask, value } => {
                active_mask = mask.clone();
                active_value = value.clone();
            }
            Command { address, value } => {
                let mut q = Vec::new();
                for i in 0..bit_len {
                    let mask = 1 << i;
                    if active_mask & mask == 0 {
                        q.push(mask);
                    }
                }

                for floating in generate_floating(q.as_slice()) {
                    let addr = (active_mask & active_value) | (active_mask & address) | floating;
                    memory.insert(addr, value.clone());
                }
            }
        };
    });

    println!("floating {:?}", memory.values().sum::<u64>());
}
