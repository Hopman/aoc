use std::fs::read_to_string;

pub fn day13(inputpath: &str) -> usize {
    let mut tokens = 0;

    let text = read_to_string(inputpath).unwrap();

    let lines: Vec<&str> = text.lines().collect();

    for l in lines.chunks(4) {
        let mut machine = Machine::from_input(l);
        tokens += machine.solve();
        //dbg!(machine);
    }

    return tokens;
}

#[derive(Debug)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Point,
    max: usize,
}

impl Machine {
    fn solve(&mut self) -> usize {
        let mut solutions = Vec::new();
        for n in 0..=100 {
            for m in 0..=100 {
                if n * self.button_a.plus_x + m * self.button_b.plus_x == self.prize.x
                    && n * self.button_a.plus_y + m * self.button_b.plus_y == self.prize.y
                {
                    solutions.push(n * self.button_a.cost + m * self.button_b.cost);
                }
            }
        }

        return solutions.into_iter().min().unwrap_or(0);
    }
    fn from_input(input: &[&str]) -> Self {
        let mut a_split = input[0].split_whitespace();
        dbg!(&a_split);
        let ax = a_split.next().unwrap().parse::<usize>().unwrap();
        let ay = a_split.next().unwrap().parse::<usize>().unwrap();

        let mut b_split = input[1].split_whitespace();
        let bx = b_split.next().unwrap().parse::<usize>().unwrap();
        let by = b_split.next().unwrap().parse::<usize>().unwrap();

        let mut prize_split = input[2].split_whitespace();
        let prizex = prize_split.next().unwrap().parse::<usize>().unwrap();
        let prizey = prize_split.next().unwrap().parse::<usize>().unwrap();

        Machine {
            button_a: Button {
                plus_y: ay,
                plus_x: ax,
                cost: 3,
            },
            button_b: Button {
                plus_y: by,
                plus_x: bx,
                cost: 1,
            },
            prize: Point {
                y: prizey,
                x: prizex,
            },
            max: 100,
        }
    }
}

#[derive(Debug)]
struct Button {
    plus_y: usize,
    plus_x: usize,
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
