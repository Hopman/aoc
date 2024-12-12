use std::fs::read_to_string;

pub fn day9(inputpath: &str) -> usize {
    let text = read_to_string(inputpath).unwrap();

    let mut disk: Vec<Option<usize>> = text
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                vec![Some(i / 2); c]
            } else {
                vec![None; c]
            }
        })
        .flatten()
        .collect();

    loop {
        while disk.last().is_none() {
            disk.pop();
        }
        if let Some(pos) = disk.iter().position(|e| *e == None) {
            let last_element = disk.pop().unwrap();
            disk[pos] = last_element;
        } else {
            break
        }
    }

    return disk.iter().enumerate().map(|(i, n)| i * n.unwrap()).sum();

}
