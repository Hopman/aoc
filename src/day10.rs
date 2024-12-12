use std::{collections::HashSet, fs::read_to_string, usize};

pub fn day10(inputpath: &str) -> usize {
    let mut result = 0;
    let text = read_to_string(inputpath).unwrap();
    let grid = Grid::grid_from_text(text);

    let trailheads = grid.find_trailheads();

    for start in trailheads {
        result += grid.follow(start.0, start.1, 0);
    }

    return result;
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<usize>>,
    height: usize,
    width: usize,
    trail_length: usize,
}

impl Grid {
    fn grid_from_text(text: String) -> Self {
        let grid = text
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        let height = grid.len();
        let width = grid[0].len();
        return Grid {
            grid,
            height,
            width,
            trail_length: 9,
        };
    }

    fn find_trailheads(&self) -> Vec<(usize, usize)> {
        let mut trailheads = Vec::new();
        for (y, line) in self.grid.iter().enumerate() {
            for (x, pos) in line.iter().enumerate() {
                if *pos == 0 {
                    trailheads.push((y, x));
                }
            }
        }
        return trailheads;
    }

    fn follow(&self, y: usize, x: usize, step: usize) -> usize {
        let mut trail_ends = 0;

        if self.grid[y][x] == step {
            if step == self.trail_length {
                return 1
            }
            if y >= 1 {
                trail_ends += self.follow(y - 1, x, step + 1);
            }
            if y + 1 < self.height {
                trail_ends += self.follow(y + 1, x, step + 1);
            }
            if x >= 1 {
                trail_ends += self.follow(y, x - 1, step + 1);
            }
            if x + 1 < self.width {
                trail_ends += self.follow(y, x + 1, step + 1);
            }

        } else {
            return trail_ends;
        }

        return trail_ends
    }

    fn follow_old(&self, y: usize, x: usize, step: usize) -> HashSet<(usize, usize)>{
        let mut trail_ends: HashSet<(usize, usize)> = HashSet::new();

        if self.grid[y][x] == step {
            if step == self.trail_length {
                trail_ends.insert((y, x));
                return trail_ends
            }
            if y >= 1 {
                trail_ends.extend(self.follow_old(y - 1, x, step + 1));
            }
            if y + 1 < self.height {
                trail_ends.extend(self.follow_old(y + 1, x, step + 1));
            }
            if x >= 1 {
                trail_ends.extend(self.follow_old(y, x - 1, step + 1));
            }
            if x + 1 < self.width {
                trail_ends.extend(self.follow_old(y, x + 1, step + 1));
            }

        } else {
            return trail_ends;
        }

        return trail_ends
    }
}
