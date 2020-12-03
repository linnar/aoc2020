use std::fs;

fn main() {
    let matrix: Vec<Vec<char>> = fs::read_to_string("inputs/input_d03.txt")
        .expect("File not found")
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let height = matrix.len();
    let width = matrix[0].len();
    println!("height x width {}x{}", height, width);

    let mut x1 = 0;
    let mut count1: i64 = 0;
    let mut x2 = 0;
    let mut count2: i64 = 0;
    let mut x3 = 0;
    let mut count3: i64 = 0;
    let mut x4 = 0;
    let mut count4: i64 = 0;
    let mut x5 = 0;
    let mut count5: i64 = 0;
    for y in 1..height {
        x1 = (x1 + 1) % width;
        if matrix[y][x1] == '#' {
            count1 += 1;
        }
        x2 = (x2 + 3) % width;
        if matrix[y][x2] == '#' {
            count2 += 1;
        }
        x3 = (x3 + 5) % width;
        if matrix[y][x3] == '#' {
            count3 += 1;
        }
        x4 = (x4 + 7) % width;
        if matrix[y][x4] == '#' {
            count4 += 1;
        }

        if y % 2 == 0 {
            x5 = (x5 + 1) % width;
            if matrix[y][x5] == '#' {
                count5 += 1;
            }
        }
    }
    println!("count {}", count2);
    println!("count {}", count1 * count2 * count3 * count4 * count5);
}
