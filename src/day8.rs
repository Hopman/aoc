use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn day8(inputpath: &str) -> usize {
    let text = read_to_string(inputpath).unwrap();

    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let mut interference_points: HashSet<Point> = HashSet::new();
    let height = text.lines().count();
    let width = text.lines().next().unwrap().chars().count();
    for (current_y, line) in text.lines().enumerate() {
        for (current_x, c) in line.chars().enumerate() {
            let current_x_i64 = current_x as i64;
            if c != '.' {
                if let Some(antenna_points) = antennas.get(&c) {
                    for antenna_point in antenna_points {
                        // 
                        let diff_y = current_y - antenna_point.y;
                        let diff_x = current_x_i64 - antenna_point.x as i64;
                        if diff_y <= antenna_point.y {
                            let up_y = antenna_point.y - diff_y;
                            let up_x = antenna_point.x as i64 - diff_x;
                            if up_x >= 0 && up_x < width as i64 {
                                interference_points.insert(Point {y: up_y, x: up_x as usize});
                            }
                        }
                        let down_y = current_y + diff_y;
                        let down_x = current_x_i64 + diff_x;
                        if down_y < height && down_x >= 0 && down_x < width as i64 {
                            interference_points.insert(Point { y: down_y, x: down_x as usize});
                        }

                    }
                }
                antennas
                    .entry(c)
                    .and_modify(|points| points.push(Point { y: current_y, x: current_x }))
                    .or_insert(vec![Point { y: current_y, x: current_x }]);
            }
        }
    }

    return interference_points.len();
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    y: usize,
    x: usize,
}
