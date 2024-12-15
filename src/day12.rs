use std::{fmt::Display, fs::read_to_string};

pub fn day12(inputpath: &str) -> usize {
    let mut grid = Grid::from_file(inputpath);

    for y in 0..grid.height {
        for x in 0..grid.width {
            let region = grid.follow(y, x, grid.grid[y][x].plant, None);
            grid.regions.push(region);
        }
    }

    let price = grid.calc_region_prices();

    return price;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
struct Point {
    plant: char,
    y: usize,
    x: usize,
    in_region: bool,
    sides: Sides,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.y == other.y {
            return Some(self.x.cmp(&other.x));
        }
        return Some(self.y.cmp(&other.y));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Sides {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

struct Region {
    points: Vec<Point>,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Point>>,
    regions: Vec<Vec<Point>>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Grid {
    fn from_file(inputpath: &str) -> Self {
        let grid = read_to_string(inputpath)
            .unwrap()
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, p)| {
                        let sides = Sides {
                            top: false,
                            bottom: false,
                            left: false,
                            right: false,
                        };

                        Point {
                            plant: p,
                            y,
                            x,
                            in_region: false,
                            sides,
                        }
                    })
                    .collect::<Vec<Point>>()
            })
            .collect::<Vec<Vec<Point>>>();
        let height = grid.len();
        let width = grid[0].len();
        Grid {
            grid,
            regions: Vec::new(),
            height,
            width,
        }
    }

    fn follow(
        &mut self,
        y: usize,
        x: usize,
        plant_name: char,
        dir: Option<Direction>,
    ) -> Vec<Point> {
        let mut points = Vec::new();

        if self.grid[y][x].plant == plant_name {
            if !self.grid[y][x].in_region {
                self.grid[y][x].in_region = true;
                points.push(self.grid[y][x]);
                if y > 0 {
                    points.extend(self.follow(y - 1, x, plant_name, Some(Direction::Up)));
                } else {
                    self.grid[y][x].sides.top = true;
                }
                if y + 1 < self.height {
                    points.extend(self.follow(y + 1, x, plant_name, Some(Direction::Down)));
                } else {
                    self.grid[y][x].sides.bottom = true;
                }
                if x > 0 {
                    points.extend(self.follow(y, x - 1, plant_name, Some(Direction::Left)));
                } else {
                    self.grid[y][x].sides.left = true;
                }
                if x + 1 < self.width {
                    points.extend(self.follow(y, x + 1, plant_name, Some(Direction::Right)));
                } else {
                    self.grid[y][x].sides.right = true;
                }
            }
        } else {
            if let Some(d) = dir {
                match d {
                    Direction::Up => self.grid[y][x].sides.bottom = true,
                    Direction::Down => self.grid[y][x].sides.top = true,
                    Direction::Left => self.grid[y][x].sides.right = true,
                    Direction::Right => self.grid[y][x].sides.left = true,
                }
            }
        }
        return points;
    }

    fn calc_region_prices(&mut self) -> usize {
        let mut price = 0;
        for region in self.regions.iter_mut() {
            region.sort();
        }
        for region in &self.regions {
            let mut sides = 0;
            for point in region {
                if self.grid[point.y][point.x].sides.top {
                    sides += 1;
                    let mut step = 0;
                    while let Some(p) = region
                        .iter()
                        .find(|p| p.y == point.y && p.x == point.x + step)
                    {
                        if self.grid[p.y][p.x].sides.top {
                            self.grid[p.y][p.x].sides.top = false;
                            step += 1;
                        } else {
                            break;
                        }
                    }
                }
                if self.grid[point.y][point.x].sides.bottom {
                    sides += 1;
                    let mut step = 0;
                    while let Some(p) = region
                        .iter()
                        .find(|p| p.y == point.y && p.x == point.x + step)
                    {
                        if self.grid[p.y][p.x].sides.bottom {
                            self.grid[p.y][p.x].sides.bottom = false;
                            step += 1;
                        } else {
                            break;
                        }
                    }
                }
                if self.grid[point.y][point.x].sides.left {
                    sides += 1;
                    let mut step = 0;
                    while let Some(p) = region
                        .iter()
                        .find(|p| p.y == point.y + step && p.x == point.x)
                    {
                        if self.grid[p.y][p.x].sides.left {
                            self.grid[p.y][p.x].sides.left = false;
                            step += 1;
                        } else {
                            break;
                        }
                    }
                }
                if self.grid[point.y][point.x].sides.right {
                    sides += 1;
                    let mut step = 0;
                    while let Some(p) = region
                        .iter()
                        .find(|p| p.y == point.y + step && p.x == point.x)
                    {
                        if self.grid[p.y][p.x].sides.right {
                            self.grid[p.y][p.x].sides.right = false;
                            step += 1;
                        } else {
                            break;
                        }
                    }
                }
                dbg!(self.grid[point.y][point.x]);
            }
            price += sides * region.len();
        }

        return price;
    }
    fn print(&self) {
        println!("{}", self);
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for line in &self.grid {
            for point in line {
                write!(f, "{}", point.plant)?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
