use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone)]
struct Field {
    name: String,
    min1: i32,
    max1: i32,
    min2: i32,
    max2: i32,
}

impl Field {
    fn contains(&self, val: i32) -> bool {
        (self.min1 <= val && val <= self.max1) || (self.min2 <= val && val <= self.max2)
    }
}

#[derive(Debug)]
struct Node {
    index: i32,
    values: HashSet<i32>,
}

fn main() {
    let re = Regex::new(r"^(.*): (\d*)-(\d*) or (\d*)-(\d*)").unwrap();
    let input: Vec<String> = fs::read_to_string("inputs/input_d16.txt")
        .expect("File not found")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut iter = input.into_iter();

    let fields: Vec<Field> = iter
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(|s| match re.captures(&s) {
            Some(cap) => Field {
                name: cap.get(1).unwrap().as_str().to_string(),
                min1: cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                max1: cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                min2: cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                max2: cap.get(5).unwrap().as_str().parse::<i32>().unwrap(),
            },
            _ => panic!("wrong format"),
        })
        .collect();
    iter.next();

    let my_ticket: Vec<i32> = iter
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    iter.next();

    iter.next();
    let other_tickets: Vec<Vec<i32>> = iter
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut error = 0;

    let valid_tickets: Vec<Vec<i32>> = other_tickets
        .into_iter()
        .filter(|ticket| {
            ticket
                .iter()
                .map(|number| {
                    let valid = fields.iter().any(|f| f.contains(*number));
                    if !valid {
                        error += number;
                    }
                    valid
                })
                .fold(true, |a, b| a && b)
        })
        .collect();

    println!("error rate {}", error);

    let mut node_builder: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut names: HashMap<i32, String> = HashMap::new();

    for i in 0..fields.len() {
        for (n, field) in fields.iter().enumerate() {
            names.entry(n as i32).or_insert(field.name.clone());
            if valid_tickets.iter().all(|t| field.contains(t[i])) {
                node_builder
                    .entry(i as i32)
                    .or_insert_with(|| HashSet::new())
                    .insert(n as i32);
            }
        }
    }

    let mut final_cols: HashMap<i32, i32> = HashMap::new();

    let mut nodes: Vec<Node> = node_builder
        .into_iter()
        .map(|(i, x)| Node {
            index: i,
            values: x.clone(),
        })
        .collect();

    while nodes.len() > 0 {
        nodes.sort_unstable_by_key(|n| n.values.len());

        let head = nodes.remove(0);
        if head.values.len() != 1 {
            panic!("unexpected value");
        }

        let fixed = head.values.iter().next().unwrap();

        final_cols.insert(head.index, fixed.clone());

        for node in nodes.iter_mut() {
            node.values.remove(&fixed);
        }
    }

    let res: i64 = final_cols
        .iter()
        .filter_map(|(index, name_i)| {
            names
                .get(name_i)
                .filter(|name| name.starts_with("departure"))
                .map(|_| index)
        })
        .map(|&i| my_ticket[i as usize] as i64)
        .fold(1, |a, b| a * b);
    println!("result {:?}", res);
}
