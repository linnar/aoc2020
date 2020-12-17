use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Active,
    Inactive,
}

fn main() {
    let mut init: HashMap<(i32, i32, i32), State> = HashMap::new();

    fs::read_to_string("inputs/input_d17.txt")
        .expect("File not found")
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                init.insert(
                    (x as i32, y as i32, 0),
                    match c {
                        '#' => State::Active,
                        '.' => State::Inactive,
                        _ => panic!["unexpected input"],
                    },
                );
            })
        });

    let offsets: Vec<(i32, i32, i32)> = (-1..=1)
        .flat_map(|x| (-1..=1).flat_map(move |y| (-1..=1).map(move |z| (x, y, z))))
        .filter(|&t| t != (0, 0, 0))
        .collect();

    let mut current: HashMap<(i32, i32, i32), State> = init.clone();

    for _ in 0..6 {
        let mut future: HashMap<(i32, i32, i32), State> = HashMap::new();
        let mut created: Vec<(i32, i32, i32)> = Vec::new();

        current.iter().for_each(|((x, y, z), state)| {
            let count: i32 = offsets
                .iter()
                .map(|(d_x, d_y, d_z)| {
                    let key: (i32, i32, i32) = (x + d_x, y + d_y, z + d_z);
                    match current.get(&key) {
                        Some(State::Active) => 1,
                        Some(State::Inactive) => 0,
                        None => {
                            created.push(key);
                            0
                        }
                    }
                })
                .sum();
            future.insert(
                (*x, *y, *z),
                match state {
                    State::Active => {
                        if count == 2 || count == 3 {
                            State::Active
                        } else {
                            State::Inactive
                        }
                    }
                    State::Inactive => {
                        if count == 3 {
                            State::Active
                        } else {
                            State::Inactive
                        }
                    }
                },
            );
        });

        created.iter().for_each(|(x, y, z)| {
            let count: i32 = offsets
                .iter()
                .map(|(d_x, d_y, d_z)| {
                    let key: (i32, i32, i32) = (x + d_x, y + d_y, z + d_z);
                    match current.get(&key) {
                        Some(State::Active) => 1,
                        Some(State::Inactive) => 0,
                        None => 0,
                    }
                })
                .sum();
            future.insert(
                (*x, *y, *z),
                if count == 3 {
                    State::Active
                } else {
                    State::Inactive
                },
            );
        });

        current = future;
    }

    let count = current.values().filter(|s| **s == State::Active).count();
    println!("size {}", count);

    let offsets_d4: Vec<(i32, i32, i32, i32)> = (-1..=1)
        .flat_map(|x| {
            (-1..=1)
                .flat_map(move |y| (-1..=1).flat_map(move |z| (-1..=1).map(move |w| (x, y, z, w))))
        })
        .filter(|&t| t != (0, 0, 0, 0))
        .collect();

    let mut current_d4: HashMap<(i32, i32, i32, i32), State> = HashMap::new();
    init.iter().for_each(|((x, y, z), state)| {
        current_d4.insert((*x, *y, *z, 0), *state);
    });

    for _ in 0..6 {
        let mut future: HashMap<(i32, i32, i32, i32), State> = HashMap::new();
        let mut created: Vec<(i32, i32, i32, i32)> = Vec::new();

        current_d4.iter().for_each(|((x, y, z, w), state)| {
            let count: i32 = offsets_d4
                .iter()
                .map(|(d_x, d_y, d_z, d_w)| {
                    let key: (i32, i32, i32, i32) = (x + d_x, y + d_y, z + d_z, w + d_w);
                    match current_d4.get(&key) {
                        Some(State::Active) => 1,
                        Some(State::Inactive) => 0,
                        None => {
                            created.push(key);
                            0
                        }
                    }
                })
                .sum();
            future.insert(
                (*x, *y, *z, *w),
                match state {
                    State::Active => {
                        if count == 2 || count == 3 {
                            State::Active
                        } else {
                            State::Inactive
                        }
                    }
                    State::Inactive => {
                        if count == 3 {
                            State::Active
                        } else {
                            State::Inactive
                        }
                    }
                },
            );
        });

        created.iter().for_each(|(x, y, z, w)| {
            let count: i32 = offsets_d4
                .iter()
                .map(|(d_x, d_y, d_z, d_w)| {
                    let key: (i32, i32, i32, i32) = (x + d_x, y + d_y, z + d_z, w + d_w);
                    match current_d4.get(&key) {
                        Some(State::Active) => 1,
                        Some(State::Inactive) => 0,
                        None => 0,
                    }
                })
                .sum();
            future.insert(
                (*x, *y, *z, *w),
                if count == 3 {
                    State::Active
                } else {
                    State::Inactive
                },
            );
        });

        current_d4 = future;
    }

    let count = current_d4.values().filter(|s| **s == State::Active).count();
    println!("size {}", count);
}
