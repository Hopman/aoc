use core::fmt;
use std::fs::read_to_string;

use crate::day4;

pub fn day6(inputpath: &str) -> usize {
    let mut grid = day4::parse_input(read_to_string(inputpath).unwrap());
    let mut map = Map::from_grid(grid);
    while map.guard_on_map {
        map.step();
    }
    return map.count();
}

struct Map {
    grid: Vec<Vec<char>>,
    grid_size: (usize, usize),
    guard_postion: (usize, usize),
    guard_direction: Direction,
    guard_on_map: bool,
}

impl Map {
    fn from_grid(grid: Vec<Vec<char>>) -> Self {
        let guard_y = grid.iter().position(|line| line.contains(&'^')).unwrap();
        let guard_x = grid[guard_y].iter().position(|c| *c == '^').unwrap();
        let grid_height = grid.len();
        let grid_width = grid[0].len();
        Self {
            grid,
            grid_size: (grid_height, grid_width),
            guard_direction: Direction::Up,
            guard_postion: (guard_y, guard_x),
            guard_on_map: true,
        }
    }
    fn print(&self) {
        println!("{}", self);
    }

    fn step(&mut self) {
        self.grid[self.guard_postion.0][self.guard_postion.1] = 'X';
        let new_position = match self.guard_direction {
            Direction::Up => {
                if self.guard_postion.0 == 0 {
                    self.guard_on_map = false;
                    return;
                } else {
                    (self.guard_postion.0 - 1, self.guard_postion.1)
                }
            },
            Direction::Down => (self.guard_postion.0 + 1, self.guard_postion.1),
            Direction::Left => { 
                if self.guard_postion.1 == 0 {
                    self.guard_on_map = false;
                    return;
                } else {
                    (self.guard_postion.0, self.guard_postion.1 - 1)
                }
            },
            Direction::Right =>  (self.guard_postion.0, self.guard_postion.1 + 1),
        };
        if new_position.0 >= self.grid_size.0 || new_position.1 >= self.grid_size.1 {
            self.guard_on_map = false;
            return
        }
        if self.grid[new_position.0][new_position.1] == '#' {
            self.rotate();
            self.step();
        } else {
            self.guard_postion = new_position;
        }
    }

    fn rotate(&mut self) {
        match self.guard_direction {
            Direction::Up => self.guard_direction = Direction::Right,
            Direction::Down => self.guard_direction = Direction::Left,
            Direction::Left => self.guard_direction = Direction::Up,
            Direction::Right => self.guard_direction = Direction::Down,
        }
    }

    fn count(&self) -> usize {
        self.grid.iter().map(|line| line.iter().filter(|c| **c == 'X').count()).sum()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.grid {
            write!(f, "{}\n", line.iter().collect::<String>());
        }
        Ok(())
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
