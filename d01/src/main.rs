use std::fs;

fn main() {
    let numbers: Vec<i64> = fs::read_to_string("inputs/input_d01.txt")
        .expect("File not found")
        .lines()
        .map(|x| x.parse::<i64>().expect("Not a i64 number"))
        .collect();
    
    'outer_loop: for (pos, number) in numbers.iter().enumerate() {
        for other_number in &numbers[pos..] {
            if number + other_number == 2020 {
                println!("Result {}", number * other_number);
                break 'outer_loop;
            }
        }
    }

    for (pos1, number1) in numbers.iter().enumerate() {
        for (pos2, number2) in numbers[pos1..].iter().enumerate() {
            let two_sum = number1 + number2;
            if two_sum < 2020 {
                for number3 in &numbers[pos2..] {
                    if two_sum + number3 == 2020 {
                        println!("Result2 {}", number1 * number2 * number3);
                        return;
                    }
                }
            }
        }
    }
}
