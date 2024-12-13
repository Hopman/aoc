use std::fs::read_to_string;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread, usize,
};

pub fn day11(inputpath: &str) -> usize {
    let mut stones: Vec<usize> = read_to_string(inputpath)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let counter: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let dict: Arc<Mutex<HashMap<(usize, usize), usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = Vec::new();

    for stone in stones {
        let counter = Arc::clone(&counter);
        let dict = Arc::clone(&dict);
        let handle = thread::spawn(move || {
            let transformed_stone = transform_stone(stone, 0, dict);
            let mut counter_lock = counter.lock().unwrap();
            *counter_lock += transformed_stone;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()
    }

    return *counter.lock().unwrap();
}

fn transform_stone(
    stone: usize,
    step: usize,
    dict: Arc<Mutex<HashMap<(usize, usize), usize>>>,
) -> usize {
    if let Some(val) = dict.lock().unwrap().get(&(stone, step)) {
        return *val;
    }
    if step == 175 {
        return 1;
    }
    let mut result = 0;
    if stone == 0 {
        result += transform_stone(1, step + 1, Arc::clone(&dict));
    } else if stone.ilog10() % 2 == 1 {
        let split = 10_usize.pow((stone.ilog10() + 1) / 2);
        result += transform_stone(stone % split, step + 1, Arc::clone(&dict));
        result += transform_stone(stone / split, step + 1, Arc::clone(&dict));
    } else {
        result += transform_stone(stone * 2024, step + 1, Arc::clone(&dict));
    }
    let mut dll = dict.lock().unwrap();
    dll.insert((stone, step), result);
    return result;
}
