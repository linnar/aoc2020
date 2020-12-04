use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let passports: Vec<HashMap<String, String>> = fs::read_to_string("inputs/input_d04.txt")
        .expect("File not found")
        .split("\n\n")
        .map(|x| {
            let mut passport = HashMap::new();
            x.trim().split(|c| c == ' ' || c == '\n').for_each(|pair| {
                let mut iter = pair.split(':');
                let key = iter.next().unwrap().to_string();
                let value = iter.next().unwrap().to_string();
                passport.insert(key, value);
            });
            passport
        })
        .collect();

    let required: Vec<String> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|x| x.to_string())
        .collect();
    let valid: Vec<HashMap<String, String>> = passports
        .into_iter()
        .filter(|p| required.iter().all(|k| p.contains_key(k)))
        .collect();
    println!("count {}", valid.len());

    let eye_colors: HashSet<&'static str> = [ "amb", "blu", "brn", "gry", "grn", "hzl", "oth" ].iter().cloned().collect();
    let count = valid.into_iter().filter(|p| {
        let byr = p.get("byr").unwrap().parse::<i32>().unwrap();
        if byr < 1920 || byr > 2002 {
            return false;
        }
        let iyr = p.get("iyr").unwrap().parse::<i32>().unwrap();
        if iyr < 2010 || iyr > 2020 {
            return false;
        }
        let eyr = p.get("eyr").unwrap().parse::<i32>().unwrap();
        if eyr < 2020 || eyr > 2030 {
            return false;
        }
        let hgt = p.get("hgt").unwrap();
        if hgt.ends_with("cm") {
            let cm = hgt[..(hgt.len() - 2)].parse::<i32>().unwrap_or(0);
            if cm < 150 || cm > 193 {
                return false;
            }
        } else if hgt.ends_with("in") {
            let inch = hgt[..(hgt.len() - 2)].parse::<i32>().unwrap_or(0);
            if inch < 59 || inch > 76 {
                return false;
            }
        } else {
            return false;
        }

        let mut hcl = p.get("hcl").unwrap().chars();
        if hcl.next().unwrap() != '#' || hcl.any(|c| !c.is_numeric() && (c < 'a' || c > 'f')) {
            return false;
        }
        
        if !eye_colors.contains(p.get("ecl").unwrap().as_str()) {
            return false;
        }

        let pid = p.get("pid").unwrap();
        if pid.len() != 9 || !pid.chars().all(char::is_numeric) {
            return false;
        }

        return true;
    }).count();
    println!("count {}", count);
}
