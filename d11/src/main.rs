use std::fmt;
use std::fs;

#[derive(Clone, Copy, PartialEq)]
enum Position {
    Empty,
    Occupied,
    Floor,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Position::Empty => write!(f, "L"),
            Position::Floor => write!(f, "."),
            Position::Occupied => write!(f, "#"),
        }
    }
}

struct Offset {
    y: i32,
    x: i32,
}

fn main() {
    let offsets: Vec<Offset> = vec![
        Offset { y: -1, x: -1 },
        Offset { y: -1, x: 0 },
        Offset { y: -1, x: 1 },
        Offset { y: 0, x: -1 },
        Offset { y: 0, x: 1 },
        Offset { y: 1, x: -1 },
        Offset { y: 1, x: 0 },
        Offset { y: 1, x: 1 },
    ];

    let matrix: Vec<Vec<Position>> = fs::read_to_string("inputs/input_d11.txt")
        .expect("File not found")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|x| {
            x.chars()
                .map(|c| match c {
                    'L' => Position::Empty,
                    '.' => Position::Floor,
                    '#' => Position::Occupied,
                    _ => panic!("Unexpected input"),
                })
                .collect()
        })
        .collect();

    let mut state = matrix.clone();
    loop {
        let mut updated = state.clone();

        for i in 0..state.len() {
            for j in 0..state[i as usize].len() {
                let count: i32 = offsets
                    .iter()
                    .map(|o| {
                        state
                            .get((i as i32 + o.y) as usize)
                            .and_then(|inner| inner.get((j as i32 + o.x) as usize))
                    })
                    .map(|r| match r {
                        Some(Position::Occupied) => 1,
                        _ => 0,
                    })
                    .sum();
                if state[i][j] == Position::Empty && count == 0 {
                    updated[i][j] = Position::Occupied;
                }

                if state[i][j] == Position::Occupied && count >= 4 {
                    updated[i][j] = Position::Empty;
                }
            }
        }

        if state.as_slice() == updated.as_slice() {
            break;
        }
        state = updated;
    }

    println!(
        "occupied near {}",
        state
            .iter()
            .map(|v| v.iter().fold(0, |a, b| a + match b {
                Position::Occupied => 1,
                _ => 0,
            }))
            .sum::<i32>()
    );

    let mut state = matrix.clone();
    loop {
        let mut updated = state.clone();

        for i in 0..state.len() {
            for j in 0..state[i as usize].len() {
                let count: i32 = offsets
                    .iter()
                    .map(|o| {
                        let mut o_i = i as i32 + o.y;
                        let mut o_j = j as i32 + o.x;
                        loop {
                            let place = state
                                .get(o_i as usize)
                                .and_then(|inner| inner.get(o_j as usize));
                            match place {
                                Some(Position::Occupied) => break Some(1),
                                Some(Position::Empty) => break Some(0),
                                None => break None,
                                _ => {
                                    o_i += o.y;
                                    o_j += o.x;
                                }
                            }
                        }
                    })
                    .map(|r| match r {
                        Some(val) => val,
                        _ => 0,
                    })
                    .sum();
                if state[i][j] == Position::Empty && count == 0 {
                    updated[i][j] = Position::Occupied;
                }

                if state[i][j] == Position::Occupied && count >= 5 {
                    updated[i][j] = Position::Empty;
                }
            }
        }

        if state.as_slice() == updated.as_slice() {
            break;
        }
        state = updated;
    }

    println!(
        "occupied sight {}",
        state
            .iter()
            .map(|v| v.iter().fold(0, |a, b| a + match b {
                Position::Occupied => 1,
                _ => 0,
            }))
            .sum::<i32>()
    );
}
