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
    let mut numbers: Vec<i64> = fs::read_to_string("inputs/input_d05.txt")
        .expect("File not found")
        .lines()
        .map(|x| {
            let row: Vec<char> = x.chars().collect();
            8 * find_index('B', &row[..7]) + find_index('R', &row[7..])
        })
        .collect();
    numbers.sort();
    println!("max id {}", numbers.last().unwrap());

    let mut seen = numbers[0];
    for n in &numbers[1..] {
        if n - 1 != seen {
            println!("my id {}", n - 1);
            break;
        } else {
            seen = *n;
        }
    }
}
