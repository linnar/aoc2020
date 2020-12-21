use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
enum Rule {
    Link(i32),
    Terminal(char),
}

fn eval(rules: &HashMap<i32, Vec<Vec<Rule>>>, rule: i32) -> Vec<String> {
    match rules.get(&rule) {
        None => panic!("unexpected"),
        Some(vec) => {
            if let Some(Rule::Terminal(c)) = vec.get(0).and_then(|v| v.get(0)) {
                vec![c.to_string()]
            } else {
                vec.iter().flat_map(|r| {
                    r.iter().map(|n| {
                        let rule_a = match n {
                            Rule::Link(n) => n.clone(),
                            _ => panic!("unexpected")
                        };
                        eval(rules, rule_a)
                    }).fold(vec!["".to_owned()], |a, b| {
                        let mut result = Vec::new();
                        for first in &a {
                            for second in &b {
                                result.push(first.to_owned() + second);
                            }
                        }
                        result
                    })
                }).collect()
            }
        }
    }
}

fn main() {
    let lines: Vec<String> = fs::read_to_string("inputs/input_d19.txt")
        .expect("File not found")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut iter = lines.iter();
    let rules: HashMap<i32, Vec<Vec<Rule>>> = iter.by_ref()
        .take_while(|l| !l.is_empty())
        .map(|s| {
            let mut kvi = s.split(":");
            let key: i32 = kvi.next().unwrap().parse().unwrap();
            let vals: Vec<Vec<Rule>> = kvi
                .next()
                .unwrap()
                .split("|")
                .map(|g| {
                    g.split_whitespace()
                        .map(|v| {
                            if v.starts_with("\"") {
                                Rule::Terminal(v.chars().nth(1).unwrap())
                            } else {
                                Rule::Link(v.parse().unwrap())
                            }
                        })
                        .collect()
                })
                .collect();
            (key, vals)
        })
        .collect();

    let tests: Vec<String> = iter.map(|l| l.to_string()).collect();

    let correct: HashSet<String> = eval(&rules, 0).into_iter().collect();

    let count = tests.iter().filter(|&t| correct.contains(t)).count();

    println!("count {}", count);


}
