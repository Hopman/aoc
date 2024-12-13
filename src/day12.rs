use std::{fmt::Display, fs::read_to_string};

pub fn day12(inputpath: &str) -> usize {
    let mut result = 0;
    let text = read_to_string(inputpath).unwrap();
    let mut grid = {
        let grid = text
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, p)| Point {
                        plant: p,
                        y,
                        x,
                        in_region: false,
                    })
                    .collect::<Vec<Point>>()
            })
            .collect::<Vec<Vec<Point>>>();
        let height = grid.len();
        let width = grid[0].len();
        Grid {
            grid,
            height,
            width,
        }
    };

    let mut price = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let area = grid.follow(y, x, grid.grid[y][x].plant);
            price += area.0 * area.1;
        }
    }

    return price;
}

#[derive(Debug)]
struct Point {
    plant: char,
    y: usize,
    x: usize,
    in_region: bool,
}

struct Region {
    plant: char,
    area: usize,
    perimeter: usize,
}

struct Grid {
    grid: Vec<Vec<Point>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn follow(&mut self, y: usize, x: usize, plant_name: char) -> (usize, usize) {
        let (mut area, mut perimiter) = (0, 0);

        if self.grid[y][x].plant == plant_name {
            if !self.grid[y][x].in_region {
                self.grid[y][x].in_region = true;
                area += 1;
                if y > 0 {
                    let ap = self.follow(y - 1, x, plant_name);
                    area += ap.0;
                    perimiter += ap.1;
                } else {
                    perimiter += 1;
                }

                if y + 1 < self.height {
                    let ap = self.follow(y + 1, x, plant_name);
                    area += ap.0;
                    perimiter += ap.1;
                } else {
                    perimiter += 1;
                }
                if x > 0 {
                    let ap = self.follow(y, x - 1, plant_name);
                    area += ap.0;
                    perimiter += ap.1;
                } else {
                    perimiter += 1;
                }
                if x + 1 < self.width {
                    let ap = self.follow(y, x + 1, plant_name);
                    area += ap.0;
                    perimiter += ap.1;
                } else {
                    perimiter += 1;
                }
            }
        } else {
            perimiter += 1;
        }

        return (area, perimiter);
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid:")?;
        for line in &self.grid {
            for point in line {
                write!(f, "{}", point.plant)?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
