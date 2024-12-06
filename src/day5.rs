use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn day5(order_inputpath: &str, pages_inputpath: &str) -> u64 {
    let mut result = 0;
    let order_string = read_to_string(order_inputpath).unwrap();
    let pages_string = read_to_string(pages_inputpath).unwrap();

    let order_map = create_map(&order_string);

    for line in pages_string
        .lines()
        .map(|l| l.split(',').rev().collect::<Vec<&str>>())
    {
        result += check_line(line, &order_map);
    }

    return result;
}

fn check_line(mut line: Vec<&str>, order_map: &HashMap<&str, HashSet<&str>>) -> u64 {
    let mut left: Vec<&str> = Vec::new();
    while !line.is_empty() {
        let page = line.pop().unwrap();
        let page_map = order_map.get(&page);
        match page_map {
            Some(pages) => {
                for p in pages {
                    if left.contains(&p) {
                        return 0;
                    }
                }
            }
            None => (),
        }

        left.push(page);
    }
    return left[left.len() / 2].parse::<u64>().unwrap();
}

fn create_map(order_string: &str) -> HashMap<&str, HashSet<&str>> {
    let mut order_map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for (o, p) in order_string.lines().map(|l| {
        let combos: Vec<_> = l.split('|').collect();
        if combos.len() != 2 {
            panic!("NO!");
        }
        (combos[0], combos[1])
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
