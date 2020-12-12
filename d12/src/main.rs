use std::fs;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

fn split_first(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

fn change_orientation(current: i32, update: i32) -> i32 {
    (current + update + 360) % 360
}

fn rotate(rot: (i32, i32), times: i32) -> (i32, i32) {
    let mut rot = rot;
    for _ in 0..times.abs() {
        rot = (times.signum() * rot.1, times.signum() * -rot.0)
    }
    rot
}

fn main() {
    let instructions: Vec<Instruction> = fs::read_to_string("inputs/input_d12.txt")
        .expect("File not found")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|x| {
            let (c, value_str) = split_first(x);
            let value = value_str.parse::<i32>().unwrap();
            match c {
                "N" => Instruction::North(value),
                "S" => Instruction::South(-value),
                "E" => Instruction::East(value),
                "W" => Instruction::West(-value),
                "L" => Instruction::Left(-value),
                "R" => Instruction::Right(value),
                "F" => Instruction::Forward(value),
                _ => panic!("Unexpected input"),
            }
        })
        .collect();

    let mut orientation = 90;
    let mut x = 0;
    let mut y = 0;

    for i in &instructions {
        let instruction = match i {
            Instruction::Forward(value) => match orientation {
                0 => Instruction::North(*value),
                90 => Instruction::East(*value),
                180 => Instruction::South(-value),
                270 => Instruction::West(-value),
                _ => panic!["Unexpected orientation"],
            },
            _ => *i,
        };
        match instruction {
            Instruction::North(value) | Instruction::South(value) => y += value,
            Instruction::East(value) | Instruction::West(value) => x += value,
            Instruction::Left(value) | Instruction::Right(value) => {
                orientation = change_orientation(orientation, value)
            }
            _ => panic!["Unexpected instruction"],
        }
    }

    println!("result {}", x.abs() + y.abs());

    let mut loc = (0, 0);
    let mut w_loc = (10, 1);

    for i in instructions {
        match i {
            Instruction::North(value) | Instruction::South(value) => {
                w_loc = (w_loc.0, w_loc.1 + value)
            }
            Instruction::East(value) | Instruction::West(value) => {
                w_loc = (w_loc.0 + value, w_loc.1)
            }
            Instruction::Right(value) | Instruction::Left(value) => {
                w_loc = rotate(w_loc, value / 90)
            }
            Instruction::Forward(value) => loc = (loc.0 + value * w_loc.0, loc.1 + value * w_loc.1),
        }
    }

    println!("result {}", loc.0.abs() + loc.1.abs());
}
