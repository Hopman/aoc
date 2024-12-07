use std::fs::read_to_string;

pub fn day7(inputpath: &str) -> usize {
    let mut result = 0;
    let text = read_to_string(inputpath).unwrap();
    for line in text.lines() {
        result += check_line(&line);
    }
    return result;
}

fn check_line(line: &str) -> usize {
    let mut result = 0;

    let split: Vec<&str> = line.split_whitespace().collect();
    let sum = split[0]
        .strip_suffix(':')
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut numbers: Vec<usize> = split[1..]
        .iter()
        .map(|n| n.parse::<usize>().unwrap())
        .rev()
        .collect();
    println!("sum: {} numbers: {:?}", sum, numbers);
    if resolve(sum, &mut numbers) {
        result += sum;
    }

    return result;
}

fn resolve(sum: usize, numbers: &mut Vec<usize>) -> bool {
    if numbers.len() > 1 {
        let a = numbers.pop().unwrap();
        let b = numbers.pop().unwrap();
        let mut add_numbers = numbers.clone();
        add_numbers.push(a + b);
        let mut mul_numbers = numbers.clone();
        mul_numbers.push(a * b);
        return resolve(sum, &mut add_numbers) || resolve(sum, &mut mul_numbers);
    } else if numbers[0] == sum {
        return true;
    } else {
        return false;
    }
}
