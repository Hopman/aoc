use std::fs::read_to_string;

pub fn day13(inputpath: &str) -> usize {
    let mut tokens = 0;

    let text = read_to_string(inputpath).unwrap();

    let lines: Vec<&str> = text.lines().collect();

    for l in lines.chunks(4) {
        let mut machine = Machine::from_input(l);
        tokens += machine.solve();
    }

    return tokens;
}

#[derive(Debug)]
struct Machine {
    a: Button,
    b: Button,
    p: Point,
}

impl Machine {
    fn solve(&mut self) -> usize {
        let a = ((self.p.x * self.b.y) as f64 - (self.b.x * self.p.y) as f64)
            / ((self.a.x * self.b.y) as f64 - (self.b.x * self.a.y) as f64);
        let b = ((self.a.x * self.p.y) as f64 - (self.p.x * self.a.y) as f64)
            / ((self.a.x * self.b.y) as f64 - (self.b.x * self.a.y) as f64);

        if a.fract() != 0.0 || b.fract() != 0.0 {
            return 0;
        }

        return a as usize * 3 + b as usize;
    }

    fn from_input(input: &[&str]) -> Self {
        let mut a_split = input[0].split_whitespace();
        let ax = a_split.next().unwrap().parse::<usize>().unwrap();
        let ay = a_split.next().unwrap().parse::<usize>().unwrap();

        let mut b_split = input[1].split_whitespace();
        let bx = b_split.next().unwrap().parse::<usize>().unwrap();
        let by = b_split.next().unwrap().parse::<usize>().unwrap();

        let mut prize_split = input[2].split_whitespace();
        let prizex = 10_000_000_000_000 + prize_split.next().unwrap().parse::<usize>().unwrap();
        let prizey = 10_000_000_000_000 + prize_split.next().unwrap().parse::<usize>().unwrap();
        //let prizex = prize_split.next().unwrap().parse::<usize>().unwrap();
        //let prizey = prize_split.next().unwrap().parse::<usize>().unwrap();

        Machine {
            a: Button {
                y: ay,
                x: ax,
            },
            b: Button {
                y: by,
                x: bx,
            },
            p: Point {
                y: prizey,
                x: prizex,
            },
        }
    }
}

#[derive(Debug)]
struct Button {
    y: usize,
    x: usize,
}

#[derive(Debug)]
struct Point {
    y: usize,
    x: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day13_example_test() {
        let result_example = day13("input/day13_example.txt");
        assert_eq!(result_example, 480);
    }
    #[test]
    fn day13_test() {
        let result = day13("input/day13.txt");
        assert_eq!(result, 40069);
    }
}
