use std::fs::read_to_string;

pub fn day4(inputpath: &str) -> usize {
    let mut result = 0;
    let text = read_to_string(inputpath).unwrap();
    let grid = parse_input(text);
    let y_len = grid.len();
    let x_len = grid[0].len();
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != 'X' {
                continue;
            }
            if x + 3 < x_len {
                let d = &grid[y][x + 1..x + 4];
                if d == ['M', 'A', 'S'] {
                    result += 1;
                }
            }
            if x > 2 {
                if &grid[y][x - 3..x] == ['S', 'A', 'M'] {
                    result += 1;
                }
            }
            if y + 3 < y_len {
                if grid[y + 1][x] == 'M' && grid[y + 2][x] == 'A' && grid[y + 3][x] == 'S' {
                    result += 1;
                }
            }
            if y > 2 {
                if grid[y - 1][x] == 'M' && grid[y - 2][x] == 'A' && grid[y - 3][x] == 'S' {
                    result += 1;
                }
            }
            if y > 2 && x > 2 {
                if grid[y - 1][x - 1] == 'M'
                    && grid[y - 2][x - 2] == 'A'
                    && grid[y - 3][x - 3] == 'S'
                {
                    result += 1;
                }
            }
            if y > 2 && x + 3 < x_len {
                if grid[y - 1][x + 1] == 'M'
                    && grid[y - 2][x + 2] == 'A'
                    && grid[y - 3][x + 3] == 'S'
                {
                    result += 1;
                }
            }
            if y + 3 < y_len && x + 3 < x_len {
                if grid[y + 1][x + 1] == 'M'
                    && grid[y + 2][x + 2] == 'A'
                    && grid[y + 3][x + 3] == 'S'
                {
                    result += 1;
                }
            }
            if y + 3 < y_len && x > 2 {
                if grid[y + 1][x - 1] == 'M'
                    && grid[y + 2][x - 2] == 'A'
                    && grid[y + 3][x - 3] == 'S'
                {
                    result += 1;
                }
            }
        }
    }

    return result;
}

fn parse_input(text: String) -> Vec<Vec<char>> {
    let grid = text
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    return grid;
}
