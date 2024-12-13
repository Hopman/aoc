use std::fs::read_to_string;

pub fn day11(inputpath: &str) -> usize {
    let mut stones: Vec<usize> = read_to_string(inputpath)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let steps = 25;

    for _ in 0..steps {
        let mut index = 0;
        for _ in 0..stones.len() {
            if stones[index] == 0 {
                stones[index] = 1;
            } else if (stones[index].ilog10() % 2) == 1 {
                let n = 10_usize.pow((stones[index].ilog10() + 1) / 2);
                stones.insert(index + 1, stones[index] % n);
                stones[index] = stones[index] / n;
                index += 1;
            } else {
                stones[index] *= 2024;
            }
            index += 1;
        }
    }


    return stones.len();
}
