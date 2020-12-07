use std::fs;
#[derive(Debug, Clone, Copy)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn run_program(program: &Vec<Op>, override_index: i32, override_op: Op) -> Result<i32, i32> {
    let len = program.len();
    let mut hit = vec![false; len];
    let mut accumulator: i32 = 0;
    let mut pc: i32 = 0;

    loop {
        if pc as usize == len {
            return Ok(accumulator);
        }
        if hit[pc as usize] == true {
            return Err(accumulator);
        }

        hit[pc as usize] = true;
        let op = if pc == override_index {
            override_op
        } else {
            program[pc as usize]
        };
        pc += match op {
            Op::Acc(acc) => {
                accumulator += acc;
                1
            }
            Op::Jmp(jmp) => jmp,
            Op::Nop(_) => 1,
        }
    }
}

fn main() {
    let program: Vec<Op> = fs::read_to_string("inputs/input_d08.txt")
        .expect("File not found")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| match &line[..3] {
            "acc" => Op::Acc(line[4..].parse::<i32>().unwrap()),
            "jmp" => Op::Jmp(line[4..].parse::<i32>().unwrap()),
            _ => Op::Nop(line[4..].parse::<i32>().unwrap()),
        })
        .collect();

    let first_op = program[0].clone();

    match run_program(&program, 0, first_op) {
        Ok(_) => println!("not actually corrupted"),
        Err(acc) => println!("default loop accumulator {}", acc),
    }

    for (i, &op) in program.iter().enumerate() {
        let index = i as i32;
        let res = match op {
            Op::Jmp(jmp) => run_program(&program, index, Op::Nop(jmp)),
            Op::Nop(jmp) => run_program(&program, index, Op::Jmp(jmp)),
            _ => Err(0),
        };
        if let Ok(acc) = res {
            println!("fixed accumulator {} at changed index {} of {}", acc, index, program.len());
        }
    }
}
