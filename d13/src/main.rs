use std::fs;

#[derive(Debug, Clone, Copy)]
enum Bus {
    Id(i64),
    Unknown,
}

fn particular(large: i64, small: i64, diff: i64) -> (i64, i64) {
    let mut a_0 = 1i64;
    let mut a_1 = 0i64;
    let mut b_0 = 0i64;
    let mut b_1 = 1i64;
    let mut a = large;
    let mut b = small;
    while b > 0 {
        let q = a / b;
        let temp = a - q * b;
        let temp_a = a_0 - q * a_1;
        let temp_b = b_0 - q * b_1;
        a = b;
        b = temp;
        a_0 = a_1;
        a_1 = temp_a;
        b_0 = b_1;
        b_1 = temp_b;
    }
    (diff * a_0, diff * b_0)
}

fn main() {
    let lines: Vec<String> = fs::read_to_string("inputs/test_d13.txt")
        .expect("File not found")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();
    let time = lines[0].parse::<i64>().unwrap();
    let buses: Vec<Bus> = lines[1]
        .split(",")
        .map(|bus| match bus {
            "x" => Bus::Unknown,
            _ => Bus::Id(bus.parse().unwrap()),
        })
        .collect();

    let min = buses
        .iter()
        .filter_map(|x| match x {
            Bus::Id(id) => Some(*id),
            _ => None,
        })
        .map(|id| (id, (((time / id) + 1) * id) - time))
        .min_by(|(_, x), (_, y)| x.cmp(y))
        .and_then(|(id, time)| Some(id * time))
        .unwrap();
    println!("part 1 {}", min);

    let number: i64 = 100000000000000;
    println!("number {}", number);

    let offsets: Vec<(usize, i64)> = buses
        .iter()
        .enumerate()
        .filter_map(|(i, bus)| match bus {
            Bus::Id(id) => Some((i, *id)),
            _ => None,
        })
        .collect();

    println!("offsets {:?}", offsets);

    let min_match = offsets.iter().fold(1i64, |a, (_, b)| a * *b as i64);
    println!("min match {}", min_match);

    println!("m {}", 6 % 1);

    let a = offsets[0].1;

    for (diff, b) in offsets.iter().skip(1) {
        let (a_diff, b_diff) = particular(*b, a, *diff as i64);
        println!("row {}({}k + {})", b, a, a_diff);
        println!("{}", -a * (-b + b_diff));
    }

    let res = particular(321, 221, -104+456);
    println!("{}", -221 * (-321 + res.1));
}
