use std::collections::HashMap;

fn main() {
    let starter = [17,1,3,16,19,0];

    let mut history: HashMap<u64, u64> = HashMap::new();

    let mut turn = 0;
    let mut spoken = 0;

    for (t, number) in starter.iter().enumerate() {
        turn = t as u64 + 1;
        spoken = *number;
        if turn < starter.len() as u64 {
            history.insert(*number, turn);
        }
    }

    loop {
        let updated = match history.get(&spoken) {
            Some(previous) => turn - previous,
            None => 0
        };
        history.insert(spoken, turn);

        spoken = updated;
        turn += 1;

        if turn == 2020 {
            println!("first spoken {}", spoken);
        }

        if turn == 30000000 {
            println!("second spoken {}", spoken);
            break;
        }
    }
}
