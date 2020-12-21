use std::fs;

#[derive(Debug,Clone)]
enum Op {
    Add,
    Mul,
    Val(i64),
    List(Vec<Op>),
}

fn run(path: &str) -> i64 {
    fs::read_to_string(path)
        .expect("File not found")
        .lines()
        .map(|s| eval(&read(s)))
        .sum()
}

fn read(code: &str) -> Vec<Op> {
    let mut stack: Vec<Vec<Op>> = Vec::new();
    stack.push(Vec::new());
    code.chars().for_each(|c| match c {
        ' ' => {}
        '0'..='9' => stack
            .last_mut()
            .unwrap()
            .push(Op::Val(c.to_digit(10).unwrap() as i64)),
        '+' => stack.last_mut().unwrap().push(Op::Add),
        '*' => stack.last_mut().unwrap().push(Op::Mul),
        '(' => {
            stack.push(Vec::new());
        }
        ')' => {
            let list = stack.pop().unwrap();
            stack.last_mut().unwrap().push(Op::List(list));
        }
        _ => panic!("unexpected input"),
    });
    stack.pop().unwrap()
}

fn eval(ops: &Vec<Op>) -> i64 {
    let mut acc: i64 = 0;
    let mut action: Option<Op> = None;

    for op in ops.iter().map(|op| {
        match op {
            Op::List(list) => Op::Val(eval(list)),
            _ => op.clone()
        }
    }) {
        match op {
            Op::Val(val) => {
                if acc == 0 {
                    acc = val;
                } else if let Some(Op::Mul) = action {
                    acc *= val;
                    action = None;
                } else if let Some(Op::Add) = action {
                    acc += val;
                    action = None;
                } else {
                    panic!("invalid")
                }
            },
            Op::Add | Op::Mul => {
                action = Some(op);
            },
            _ => panic!("unexpected")
        }
    }

    acc
}

fn main() {
    let result = run("inputs/input_d18.txt");
    println!("result {}", result);
}
