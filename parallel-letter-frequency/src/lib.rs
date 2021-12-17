use std::{collections::HashMap, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let division = if worker_count < input.len() {
        (input.len() - 1) / worker_count
    } else {
        input.len()
    };

    let mut handles = vec![];

    for input in input.chunks(division) {
        let input: Vec<String> = input.iter().map(|s| s.to_string()).collect();
        let handle = thread::spawn(move || {
            input.iter().fold(HashMap::new(), |map, word| {
                word.chars()
                    .filter(|letter| letter.is_alphabetic())
                    .fold(map, |mut map, letter| {
                        *map.entry(letter.to_ascii_lowercase()).or_insert(0) += 1;
                        map
                    })
            })
        });

        handles.push(handle);
    }

    let mut map = HashMap::new();

    for handle in handles {
        let result = handle.join().unwrap();
        for (key, val) in result {
            *map.entry(key).or_insert(0) += val;
        }
    }

    map
}
