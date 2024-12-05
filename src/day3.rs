use std::fs::read_to_string;
use regex::Regex;

pub fn day3() -> u64 {
    let mut result = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let input_string = read_to_string("input/day3.txt").unwrap();

    for i in re.captures_iter(&input_string).map(|nums| {
        let (_, [one, two]) = nums.extract();
        one.parse::<u64>().unwrap() * two.parse::<u64>().unwrap()
    }) {
        result += i;
    }

    return result;
}

