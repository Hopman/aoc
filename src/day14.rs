use std::fs::read_to_string;

use regex::Regex;

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

    for robot in robots.iter_mut() {
        robot.step_n(100, H, W);
    }
    print_bots(&robots, H, W);

    let topleft = robots.iter().filter(|r| r.px < W / 2 && r.py < H /2).count();
    let topright = robots.iter().filter(|r| r.px > W / 2 && r.py < H /2).count();
    let botleft = robots.iter().filter(|r| r.px < W / 2 && r.py > H /2).count();
    let botright = robots.iter().filter(|r| r.px > W / 2 && r.py > H /2).count();

    return topleft * topright * botleft * botright;
}

fn print_bots(robots: &Vec<Robot>, height: i64, width: i64) {
    for y in 0..height {
        println!();
        for x in 0..width {
            let r = robots.iter().filter(|r| r.px == x && r.py == y).count();
            if r > 0 {
                print!("{r}");
            } else {
                print!(".");
            }
        }
    }
    println!();
}

#[derive(Debug)]
struct Robot {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

impl Robot {
    fn step_n(&mut self, steps: i64, height: i64, width: i64) {
        self.px = (steps * self.vx + self.px).rem_euclid(width);
        self.py = (steps * self.vy + self.py).rem_euclid(height);
    }
    fn step(&mut self, height: usize, width: usize) {}
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
