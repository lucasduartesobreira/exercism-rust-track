use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let map: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    let division = if worker_count < input.len() {
        (input.len() - 1) / worker_count
    } else {
        input.len()
    };

    let mut handles = vec![];

    for sliced_input in input.chunks(division) {
        let map = Arc::clone(&map);
        let sla: Vec<String> = sliced_input.iter().map(|&s| s.to_string()).collect();
        //let sla = sliced_input.to_owned();
        let handle = thread::spawn(move || {
            sla.iter().for_each(|word| {
                word.chars().filter(|c| c.is_alphabetic()).for_each(|ch| {
                    *map.lock()
                        .unwrap()
                        .entry(ch.to_ascii_lowercase())
                        .or_insert(0) += 1;
                })
            })
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join();
    }

    let x = map
        .lock()
        .unwrap()
        .iter()
        .map(|(key, value)| (*key, *value))
        .collect();

    println!("{:?}", x);
    x
}
