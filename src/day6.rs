use core::fmt;
use std::{collections::HashSet, fs::read_to_string};

use crate::day4;

pub fn day6(inputpath: &str) -> usize {
    let mut result = 0;
    let mut grid = day4::parse_input(read_to_string(inputpath).unwrap());
    let mut map = Map::from_grid(grid);
    solve_map(&mut map);
    for guard_pos in &map.guard_steps.iter().map(|s| (s.0, s.1)).collect::<HashSet<(usize, usize)>>() {
        let mut obstruction_map = map.clone();
        obstruction_map.grid[guard_pos.0][guard_pos.1] = 'O';
        obstruction_map.guard_on_map = true;
        obstruction_map.obstruction = true;
        obstruction_map.guard_postion = map.start_position;
        obstruction_map.guard_direction = map.start_direction;
        obstruction_map.guard_steps.clear();
        solve_map(&mut obstruction_map);
        if obstruction_map.guard_loops {
            result += 1;
        }
    }
    return result;
}

fn solve_map(map: &mut Map) {
    while map.guard_on_map && !map.guard_loops {
        map.step();
    }
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<char>>,
    grid_size: (usize, usize),
    guard_postion: (usize, usize),
    guard_direction: Direction,
    guard_on_map: bool,
    guard_steps: HashSet<(usize, usize, Direction)>,
    guard_loops: bool,
    start_position: (usize, usize),
    start_direction: Direction,
    obstruction: bool,
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
            start_direction: Direction::Up,
            guard_postion: (guard_y, guard_x),
            start_position: (guard_y, guard_x),
            guard_on_map: true,
            guard_loops: false,
            guard_steps: HashSet::new(),
            obstruction: false,
        }
    }
    fn print(&self) {
        println!("{}", self);
    }

    fn step(&mut self) {
        if self.obstruction {
            self.grid[self.guard_postion.0][self.guard_postion.1] = '=';
        } else {
            self.grid[self.guard_postion.0][self.guard_postion.1] = 'X';
        }
        let new_position = match self.guard_direction {
            Direction::Up => {
                if self.guard_postion.0 == 0 {
                    self.guard_on_map = false;
                    return;
                } else {
                    (self.guard_postion.0 - 1, self.guard_postion.1)
                }
            }
            Direction::Down => (self.guard_postion.0 + 1, self.guard_postion.1),
            Direction::Left => {
                if self.guard_postion.1 == 0 {
                    self.guard_on_map = false;
                    return;
                } else {
                    (self.guard_postion.0, self.guard_postion.1 - 1)
                }
            }
            Direction::Right => (self.guard_postion.0, self.guard_postion.1 + 1),
        };
        if new_position.0 >= self.grid_size.0 || new_position.1 >= self.grid_size.1 {
            self.guard_on_map = false;
            return;
        }
        if self
            .guard_steps
            .get(&(new_position.0, new_position.1, self.guard_direction))
            .is_some()
        {
            self.guard_loops = true;
            return;
        } else if self.grid[new_position.0][new_position.1] == '#'
            || self.grid[new_position.0][new_position.1] == 'O'
        {
            self.rotate();
            self.step();
        } else {
            self.guard_postion = new_position;
            self.guard_steps
                .insert((new_position.0, new_position.1, self.guard_direction));
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
        self.grid
            .iter()
            .map(|line| line.iter().filter(|c| **c == 'X').count())
            .sum()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
