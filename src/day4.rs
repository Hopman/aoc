use std::fs::read_to_string;

pub fn day4(inputpath: &str) -> usize {
    let mut result = 0;
    let text = read_to_string(inputpath).unwrap();
    let grid = parse_input(text);
    let y_len = grid.len();
    let x_len = grid[0].len();
    for (y, line) in grid.iter().enumerate() {
        if y == 0 || y == y_len - 1 {
            continue;
        }
        for (x, c) in line.iter().enumerate() {
            if x == 0 || x == x_len - 1 {
                continue;
            }
            if *c != 'A' {
                continue;
            }

            let mut horizontal = false;
            let mut vertical = false;
            let mut diagonal_l = false;
            let mut diagonal_r = false;

            if grid[y][x - 1] == 'M' && grid[y][x + 1] == 'S' {
                horizontal = true;
            }
            if grid[y][x - 1] == 'S' && grid[y][x + 1] == 'M' {
                horizontal = true;
            }
            if grid[y - 1][x] == 'M' && grid[y + 1][x] == 'S' {
                vertical = true;
            }
            if grid[y - 1][x] == 'S' && grid[y + 1][x] == 'M' {
                vertical = true;
            }
            if grid[y - 1][x - 1] == 'M' && grid[y + 1][x + 1] == 'S' {
                diagonal_l = true;
            }
            if grid[y - 1][x - 1] == 'S' && grid[y + 1][x + 1] == 'M' {
                diagonal_l = true;
            }
            if grid[y - 1][x + 1] == 'M' && grid[y + 1][x - 1] == 'S' {
                diagonal_r = true;
            }
            if grid[y - 1][x + 1] == 'S' && grid[y + 1][x - 1] == 'M' {
                diagonal_r = true;
            }
            if horizontal && vertical {
                result += 1;
            } else if diagonal_l && diagonal_r {
                println!("DIA:");
                println!("{} {} {}", grid[y - 1][x - 1], grid[y][x], grid[y + 1][x + 1]);
                println!("{} {} {}", grid[y - 1][x + 1], grid[y][x], grid[y + 1][x - 1]);
                result += 1;
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
