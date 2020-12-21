use std::collections::HashSet;
use std::fs;
#[derive(Debug)]
struct Food {
    alergens: HashSet<String>,
    ingredients: HashSet<String>,
}

fn main() {
    let food: Vec<Food> = fs::read_to_string("inputs/input_d21.txt")
        .expect("File not found")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| {
            let mut iter = line.chars();
            let ingredients_str: String = iter.by_ref().take_while(|&c| c != '(').collect();
            let ingredients = ingredients_str
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| s.to_owned())
                .collect();
            let alergens = iter
                .skip(9)
                .take_while(|&c| c != ')')
                .collect::<String>()
                .split(", ")
                .map(|s| s.to_owned())
                .collect();
            Food {
                alergens,
                ingredients,
            }
        })
        .collect();
    let alergens: HashSet<String> = food
        .iter()
        .flat_map(|f| f.alergens.iter().cloned())
        .collect();
    let ingredients: HashSet<String> = food
        .iter()
        .flat_map(|f| f.ingredients.iter().cloned())
        .collect();

    let mut dirty: HashSet<String> = HashSet::new();
    let mut dirty_sets: Vec<(String, HashSet<String>)> = Vec::new();

    for alergen in alergens.iter() {
        let mut intersection = HashSet::new();
        for food in food.iter().filter(|food| food.alergens.contains(alergen)) {
            if intersection.is_empty() {
                intersection.extend(food.ingredients.iter().cloned());
            } else {
                intersection = intersection
                    .intersection(&food.ingredients)
                    .cloned()
                    .collect();
            }
        }
        dirty.extend(intersection.iter().cloned());
        dirty_sets.push((alergen.to_owned(), intersection));
    }
    let clean: HashSet<String> = ingredients.difference(&dirty).cloned().collect();

    let count: usize = food
        .iter()
        .map(|f| f.ingredients.iter().filter(|&i| clean.contains(i)).count())
        .sum();
    println!("count {}", count);

    let mut results: Vec<(String, String)> = Vec::new();

    while !dirty_sets.is_empty() {
        dirty_sets.sort_by_key(|(_, set)| -1 * set.len() as i32);

        if let Some((alergen, ingredients)) = dirty_sets.pop() {
            if ingredients.len() != 1 {
                panic!("advanced");
            }
            let ingredient = ingredients.iter().next().unwrap();
            results.push((alergen, ingredient.to_owned()));

            for (_, other_ingredients) in dirty_sets.iter_mut() {
                other_ingredients.remove(ingredient);
            }
        }
    }

    results.sort_by_cached_key(|(alergen, _)| alergen.to_owned());
    println!(
        "result {:?}",
        results
            .iter()
            .map(|(_, ingredient)| ingredient.to_owned())
            .collect::<Vec<String>>()
            .join(",")
    );
}
