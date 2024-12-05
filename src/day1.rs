use std::fs::read_to_string;

pub fn day1() -> u64 {
    let mut result = 0;

    let left_list_string = read_to_string("input/day1_left.txt").unwrap();
    let right_list_string = read_to_string("input/day1_right.txt").unwrap();

    let left_list_int: Vec<u64> = left_list_string
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();
    let right_list_int: Vec<u64> = right_list_string
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    for left in left_list_int.iter() {
        let right_count = right_list_int.iter().filter(|r| *r == left).count() as u64;
        result += left * right_count;
    }
    return result;
}
