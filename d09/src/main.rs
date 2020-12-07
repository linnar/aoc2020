use std::fs;

fn precompute(values: &Vec<i64>, preamble: i32, index: i32) -> Vec<i64> {
    let mut vec = vec![0; (preamble - 1) as usize];
    for i in (0..preamble - 1).rev() {
        let previous_index = index - 1 - i;
        if previous_index >= 0 {
            vec[i as usize] = values[index as usize] + values[previous_index as usize];
        }
    }
    vec
}

fn main() {
    let preamble: usize = 25;
    let values: Vec<i64> = fs::read_to_string("inputs/input_d09.txt")
        .expect("File not found")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut memory: Vec<i64> = Vec::new();

    let memory_width: i32 = preamble as i32 - 1;
    for i in 0..values.len() {
        memory.append(&mut precompute(&values, preamble as i32, i as i32));
    }

    let offsets: Vec<i32> = (0..memory_width)
        .flat_map(|i| (i..memory_width).map(move |j| -(i * memory_width + j + 1)))
        .collect();

    let mut invalid = 0;
    for i in preamble..values.len() {
        let current = values[i];
        if !offsets
            .iter()
            .any(|&offset| memory[(i as i32 * memory_width + offset) as usize] == current)
        {
            invalid = current;
            break;
        }
    }
    println!("first invalid {}", invalid);

    let mut table = values.clone();
    let mut result = (0, 0);
    'outer: for w_len in 1..values.len() {
        let top = table.len() - w_len;
        for i in 0..top {
            table[i] += values[i + w_len];
        }
        for i in 0..top {
            if table[i] == invalid {
                result = (i, i + w_len);
                break 'outer;
            }
        }
    }
    let weakness = values[result.0..result.1].iter().min().unwrap()
        + values[result.0..result.1].iter().max().unwrap();
    println!("encryption weakness {}", weakness);
}
