use std::cmp::min;
use std::{collections::HashMap, thread};

pub fn frequency(input: &[&'static str], worker_count: usize) -> HashMap<char, usize> {
    let real_worker_count = min(input.len(), worker_count);
    let worker_length = (input.len() / real_worker_count).max(1);
    let fixed_worker_length = if real_worker_count * worker_length < input.len() {
        worker_length + 1
    } else {
        worker_length
    };

    let mut thread_pool = Vec::with_capacity(real_worker_count);

    for input in input.chunks(fixed_worker_length) {
        let owned_input = input.to_owned();
        let my_thread = thread::spawn(move || count_letters(owned_input));
        thread_pool.push(my_thread);
    }

    let mut map = HashMap::<char, usize>::new();

    for my_thread in thread_pool {
        let result = my_thread.join().unwrap();
        for (key, val) in result {
            *map.entry(key).or_insert(0) += val;
        }
    }

    map
}

fn count_letters(input: Vec<&str>) -> HashMap<char, usize> {
    let mut map = HashMap::<char, usize>::new();

    for string in input {
        for ch in string.chars() {
            *map.entry(ch).or_default() += 1;
        }
    }
    map
}
