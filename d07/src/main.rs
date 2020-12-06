use regex::Regex;
use std::{collections::HashMap, collections::HashSet, fs};

fn find_size(
    rules: &HashMap<String, HashMap<String, i32>>,
    memory: &mut HashMap<String, i32>,
    key: String,
) -> i32 {
    let val = memory.get(&key);
    if val.is_some() {
        return val.unwrap().clone();
    }
    let computed = match rules.get(&key) {
        None => 0,
        Some(components) => components
            .iter()
            .map(|e| e.1 * (1 + find_size(rules, memory, e.0.clone())))
            .sum(),
    };
    memory.insert(key, computed.clone());
    computed
}

fn find_outer(index: &HashMap<String, HashSet<String>>, key: String) -> i32 {
    let mut queue: Vec<String> = Vec::new();
    queue.push(key);
    let mut outer: HashSet<String> = HashSet::new();
    while let Some(v) = queue.pop() {
        match index.get(&v) {
            Some(value) => value.iter().for_each(|n| {
                if !outer.contains(n) {
                    queue.push(n.clone());
                    outer.insert(n.to_string());
                }
            }),
            None => {}
        }
    }
    outer.len() as i32
}

fn main() {
    let re = Regex::new(r"^(.*?) bags contain (.*)").unwrap();
    let re2 = Regex::new(r"^(\d+) (.*) bag.*").unwrap();
    let mut rules: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut index: HashMap<String, HashSet<String>> = HashMap::new();
    let mut memory: HashMap<String, i32> = HashMap::new();

    fs::read_to_string("inputs/input_d07.txt")
        .expect("File not found")
        .lines()
        .for_each(|x| {
            let captures = re.captures(x).unwrap();
            let key = captures.get(1).unwrap().as_str().to_string();
            let rest = captures.get(2).unwrap().as_str();

            if rest != "no other bags." {
                rest.split(", ").for_each(|s| {
                    let inner_captures = re2.captures(s).unwrap();
                    let count: i32 = inner_captures.get(1).unwrap().as_str().parse().unwrap();
                    let name = inner_captures.get(2).unwrap().as_str().to_string();
                    rules
                        .entry(key.clone())
                        .or_insert_with(HashMap::new)
                        .insert(name.clone(), count);
                    index
                        .entry(name)
                        .or_insert_with(HashSet::new)
                        .insert(key.clone());
                })
            }
        });

    println!(
        "outer bags {:?}",
        find_outer(&index, "shiny gold".to_string())
    );
    println!(
        "inner bags {:?}",
        find_size(&rules, &mut memory, "shiny gold".to_string())
    );
}
