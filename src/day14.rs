use std::{fs::read_to_string, thread::sleep, time::Duration};

use regex::Regex;

//fn main() {
//    //let result_example = day14("input/day14_example.txt");
//    let result = day14("input/day14.txt");
//}

pub fn day14(inputpath: &str) -> usize {
    let mut result = 0;
    let text = read_to_string(inputpath).unwrap();

    let re = Regex::new(r"=(-?\d+),(-?\d+).+=(-?\d+),(-?\d+)").unwrap();
    let mut robots = re
        .captures_iter(&text)
        .map(|matches| {
            let (_, [px, py, vx, vy]) = matches.extract();
            Robot {
                px: px.parse::<i64>().unwrap(),
                py: py.parse::<i64>().unwrap(),
                vx: vx.parse::<i64>().unwrap(),
                vy: vy.parse::<i64>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    const W: i64 = 101;
    const H: i64 = 103;
    let mut field = Field {
        robots,
        width: W,
        height: H,
    };

    field.step_n(10_000);

    let topleft = field
        .robots
        .iter()
        .filter(|r| r.px < W / 2 && r.py < H / 2)
        .count();
    let topright = field
        .robots
        .iter()
        .filter(|r| r.px > W / 2 && r.py < H / 2)
        .count();
    let botleft = field
        .robots
        .iter()
        .filter(|r| r.px < W / 2 && r.py > H / 2)
        .count();
    let botright = field
        .robots
        .iter()
        .filter(|r| r.px > W / 2 && r.py > H / 2)
        .count();

    return topleft * topright * botleft * botright;
}

#[derive(Debug)]
struct Robot {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

impl Robot {
    fn step(&mut self, height: i64, width: i64) {
        self.px = (self.vx + self.px).rem_euclid(width);
        self.py = (self.vy + self.py).rem_euclid(height);
    }
}

#[derive(Debug)]
struct Field {
    robots: Vec<Robot>,
    width: i64,
    height: i64,
}

impl Field {
    fn step_n(&mut self, steps: i64) {
        for i in 0..steps {
            let mut tree = false;
            for r in self.robots.iter_mut() {
                r.step(self.height, self.width);
            }
            for rr in &self.robots {
                tree = true;
                for j in 1..10 {
                    if !self
                        .robots
                        .iter()
                        .any(|or| or.px == rr.px + j && or.py == rr.py)
                    {
                        tree = false;
                        break;
                    }
                }
                if tree {
                    break;
                }
            }
            if tree {
                self.print_bots(self.height, self.width);
                println!("{i}");
            };
        }
    }
    fn print_bots(&self, height: i64, width: i64) {
        print!("{}[2J", 27 as char);
        for y in 0..height {
            println!();
            for x in 0..width {
                let r = self
                    .robots
                    .iter()
                    .filter(|r| r.px == x && r.py == y)
                    .count();
                if r > 0 {
                    print!("{r}");
                } else {
                    print!(".");
                }
            }
        }
        println!();
        sleep(Duration::from_millis(100));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_example_test() {
        let result_example = day14("input/day14_example.txt");
        assert_eq!(result_example, 12);
    }
    #[test]
    fn day14_test() {
        let result = day14("input/day14.txt");
        assert_eq!(result, 230461440);
    }
}
