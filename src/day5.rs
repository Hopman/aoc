use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn day5(order_inputpath: &str, pages_inputpath: &str) -> u64 {
    let mut result = 0;
    let order_string = read_to_string(order_inputpath).unwrap();
    let pages_string = read_to_string(pages_inputpath).unwrap();

    let order_map = create_map(&order_string);

    for line in pages_string.lines().map(|l| {
        l.split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    }) {
        let r = check_line(line.clone(), &order_map);
        if r != line {
            println!("r vs line:\n{:?}\n{:?}", r, line);
            result += r[r.len() / 2];
        }
    }
    return result;
}

fn check_line(mut line: Vec<u64>, order_map: &HashMap<u64, HashSet<u64>>) -> Vec<u64> {
    let mut left: Vec<u64> = Vec::new();
    line = line.into_iter().rev().collect();
    while !line.is_empty() {
        let page_number = line.pop().unwrap();
        let page_mappings = order_map.get(&page_number);
        let mut insert_positions = Vec::new();
        match page_mappings {
            Some(pages) => {
                for p in pages {
                    if left.contains(&p) {
                        let insert_at = left.iter().position(|q| q == p).unwrap();
                        insert_positions.push(insert_at);
                    }
                }
            }
            None => (),
        }
        if insert_positions.len() == 0 {
            left.push(page_number);
        } else {
            left.insert(*insert_positions.iter().min().unwrap(), page_number);
        }
    }
    return left;
}

fn create_map(order_string: &str) -> HashMap<u64, HashSet<u64>> {
    let mut order_map: HashMap<u64, HashSet<u64>> = HashMap::new();

    for (o, p) in order_string.lines().map(|l| {
        let combos: Vec<_> = l.split('|').collect();
        if combos.len() != 2 {
            panic!("NO!");
        }
        (
            combos[0].parse::<u64>().unwrap(),
            combos[1].parse::<u64>().unwrap(),
        )
    }) {
        order_map
            .entry(o)
            .and_modify(|v| {
                v.insert(p);
            })
            .or_insert(HashSet::from([p]));
    }

    return order_map;
}
