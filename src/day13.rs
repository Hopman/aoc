use std::{fs::read_to_string, usize};

pub fn day13(inputpath: &str) -> usize {
    let mut tokens = 0;

    let text = read_to_string(inputpath).unwrap();

    let lines: Vec<&str> = text.lines().collect();

    for (i, l) in lines.chunks(4).enumerate() {
        dbg!(i);
        let mut machine = Machine::from_input(l);
        tokens += machine.solve();
        //dbg!(machine);
    }

    return tokens;
}

#[derive(Debug)]
struct Machine {
    a: Button,
    b: Button,
    p: Point,
    max: usize,
}

impl Machine {
    fn solve(&mut self) -> usize {
        let mut solutions = Vec::new();

        let button_matrix = nalgebra::Matrix2::new(
            self.a.x as f64,
            self.b.x as f64,
            self.a.y as f64,
            self.b.y as f64,
        );

        let prize_vector = nalgebra::Vector2::new(self.p.x as f64, self.p.y as f64);

        let button_matrix_lu = button_matrix.lu();

        let solution = button_matrix_lu.solve(&prize_vector);

        if let Some(val) = solution {
            let v: Vec<usize> = val.iter().filter(|&v| (v.round() - v).abs() < 1e-10).map(|&v| v as usize).collect() ;
            dbg!(&val, &v);
            if v.len() > 1 {
                solutions.push(v[0] * self.a.cost + v[1] * self.b.cost);
            }

        }

        //for n in 0..=100_000 {
        //    for m in 0..=100_000 {
        //        if n * self.a.x + m * self.b.x == self.p.x
        //            && n * self.a.y + m * self.b.y == self.p.y
        //        {
        //            solutions.push(n * self.a.cost + m * self.b.cost);
        //        }
        //    }
        //}

        return solutions.into_iter().min().unwrap_or(0);
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
                cost: 3,
            },
            b: Button {
                y: by,
                x: bx,
                cost: 1,
            },
            p: Point {
                y: prizey,
                x: prizex,
            },
            max: 100,
        }
    }
}

#[derive(Debug)]
struct Button {
    y: usize,
    x: usize,
    cost: usize,
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
    //#[test]
    //fn day13_example_two_test() {
    //    let result_example = day13::day13("input/day13_example_two.txt");
    //    assert_eq!(result_example, 1306);
    //}
    #[test]
    fn day13_test() {
        let result = day13("input/day13.txt");
        assert_eq!(result, 870202);
    }
}
