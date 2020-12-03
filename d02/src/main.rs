use std::fs;

#[derive(Debug)]
struct Pattern {
    letter: char,
    min: u16,
    max: u16
}

fn main() {
    let pairs: Vec<(Pattern, String)> = fs::read_to_string("inputs/input_d02.txt")
        .expect("File not found")
        .lines()
        .map(|x| {
            let pattern_text: Vec<&str> = x.split(":").collect();
            let pattern_str: Vec<&str> = pattern_text[0].split(" ").collect();
            let counts: Vec<&str> = pattern_str[0].split("-").collect();
            let min: u16 = counts[0].parse::<u16>().expect("number");
            let max: u16 = counts[1].parse::<u16>().expect("number");
            let letter = pattern_str[1].chars().next().expect("character");
            let text = pattern_text[1].trim().to_string();
            (Pattern { letter, min, max }, text)
        })
        .collect();

    let mut total_count: u32 = 0;
    for (pattern, text) in pairs.iter() {
        let mut count: u16 = 0;
        for c in text.chars() {
            if c == pattern.letter {
                count += 1;
            }
        }
        if count >= pattern.min && count <= pattern.max {
            total_count += 1;
        } 
    }
    println!("total count {}", total_count);

    let mut total_count2: u32 = 0;
    for (pattern, text) in pairs.iter() {
        let a = text.chars().nth(pattern.min as usize - 1).unwrap() == pattern.letter;
        let b = text.chars().nth(pattern.max as usize - 1).unwrap() == pattern.letter;
        if a ^ b {
            total_count2 += 1;
        } 
    }
    println!("total count {}", total_count2);
}
