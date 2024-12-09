use std::{
    collections::HashSet,
    fmt::{self, Display},
    fs::read_to_string,
};

pub fn day8(inputpath: &str) -> usize {
    let mut result = 0;
    let text = read_to_string(inputpath).unwrap();
    let mut grid = Grid::from_text(text);
    println!("{}", grid);

    grid.interference();

    return result;
}

struct Grid {
    grid: Vec<Vec<Point>>,
    grid_size: (usize, usize),
    antenna_types: HashSet<char>,
    antennas: Vec<Point>,
}

impl Grid {
    fn from_text(text: String) -> Self {
        let mut antenna_types = HashSet::new();
        let mut antennas: Vec<Point> = Vec::new();
        let grid = text
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let p = Point {
                            y,
                            x,
                            antenna: if c != '.' { Some(c) } else { None },
                            antinode: false,
                        };
                        if let Some(c) = p.antenna {
                            antenna_types.insert(c);
                            antennas.push(p.clone());
                        }
                        p
                    })
                    .collect::<Vec<Point>>()
            })
            .collect::<Vec<Vec<Point>>>();
            let grid_size = (grid.len(), grid[0].len());
        Grid {
            grid,
            grid_size,
            antenna_types,
            antennas,
        }
    }

    fn interference(&mut self) {
        for antenna in &self.antenna_types {
            for p in self.antennas.iter().filter(|p| p.antenna == Some(*antenna) ) {
                println!("P: {:?}", p);
            }
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.grid {
            write!(
                f,
                "{}\n",
                line.iter().map(|p| format!("{}", p)).collect::<String>()
            )?
        }
        write!(f, "{:?}\n", self.antenna_types)?;
        for a in &self.antennas {
            write!(f, "{}: {}x{}\n", a, a.y, a.x)?;
        };
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Point {
    y: usize,
    x: usize,
    antenna: Option<char>,
    antinode: bool,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(a) = self.antenna {
            write!(f, "{} ", a)
        } else {
            write!(f, ". ")
        }
    }
}
