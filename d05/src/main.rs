use std::fs;

fn find_index(high: char, data: &[char]) -> i64 {
    let len = data.len();
    let mut index = 0;
    let mut pow = 1;
    for i in 0..len {
        if data[len - i - 1] == high {
            index += pow;
        }
        pow *= 2;
    }
    index
}

fn main() {
    let numbers: Vec<i64> = fs::read_to_string("inputs/input_d05.txt")
        .expect("File not found")
        .lines()
        .map(|x| {
            let row: Vec<char> = x.chars().collect();
            8 * find_index('B', &row[..7]) + find_index('R', &row[7..])
        })
        .collect();

    let min = numbers.iter().min().unwrap().clone();
    let max = numbers.iter().max().unwrap().clone();
    println!("max id {}", max);
    let mut empty = vec![true; (max - min + 1) as usize];
    numbers.iter().for_each(|x| empty[(x - min) as usize] = false);
    let empty_id: i64 = empty.iter().position(|seat| *seat).unwrap() as i64 + min;
    println!("my id {}", empty_id);
}
