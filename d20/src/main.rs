use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Tile {
    id: i32,
    data: Vec<String>,
    e0: String,
    e1: String,
    e2: String,
    e3: String
}

impl Tile {
    fn edges(&self) -> [&str; 4] {
        [&self.e0, &self.e1, &self.e2, &self.e3]
    }
}


fn main() {
    let lines: Vec<Tile> = fs::read_to_string("inputs/input_d20.txt")
        .expect("File not found")
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut iter = s.lines();
            let id = iter.next().unwrap().chars().skip(5).take(4).collect::<String>().parse::<i32>().unwrap();
            let data: Vec<String> = iter.map(|s| s.to_owned()).collect();
            let e0 = data[0].clone();
            let e1 = data.iter().map(|r| r.chars().last().unwrap()).collect();
            let e2 = data.last().unwrap().clone();
            let e3 = data.iter().map(|r| r.chars().next().unwrap()).collect();
            Tile { id, data, e0, e1, e2, e3 }
        })
        .collect();

    let mut counter: HashMap<String, Vec<i32>> = HashMap::new();
    lines.iter().for_each(|tile| {
        for &edge in tile.edges().iter() {
            let rev = edge.clone().chars().rev().collect::<String>();
            counter.entry(edge.to_owned()).and_modify(|e| { e.push(tile.id) }).or_insert(vec![tile.id]);
            counter.entry(rev).and_modify(|e| { e.push(tile.id) }).or_insert(vec![tile.id]);
        }
    });

    let mut single: HashMap<i32, i32> = HashMap::new();
    for value in counter.values() {
        if value.len() == 1 {
        if let Some(id) = value.get(0) {
            single.entry(*id).and_modify(|e| { *e += 1 }).or_insert(1);
        }
    }
    }

    println!("single {:?}", single.iter().filter(|(_, &v)| v == 4).fold(1i64, |a, (k, _)| a * *k as i64));

}
