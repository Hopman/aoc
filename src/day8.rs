use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn day8(inputpath: &str) -> usize {
    let text = read_to_string(inputpath).unwrap();

    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let mut interference_points: HashSet<Point> = HashSet::new();

    let height = text.lines().count() as i64;
    let width = text.lines().next().unwrap().chars().count() as i64;

    for (current_y, line) in text.lines().enumerate() {
        for (current_x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let current_x_i64 = current_x as i64;
            let current_y_i64 = current_y as i64;

            if let Some(antenna_points) = antennas.get(&c) {
                for antenna_point in antenna_points {
                    let diff_y = current_y_i64 - antenna_point.y;
                    let diff_x = current_x_i64 - antenna_point.x;

                    let mut up_y = antenna_point.y;
                    let mut up_x = antenna_point.x;
                    let mut up_step = 0;
                    while up_x >= 0 && up_x < width && up_y >= 0 && up_y < height {
                        interference_points.insert(Point { y: up_y, x: up_x });
                        up_step += 1;
                        up_y = antenna_point.y - (diff_y * up_step);
                        up_x = antenna_point.x - (diff_x * up_step);
                    }

                    let mut down_y = current_y_i64;
                    let mut down_x = current_x_i64;
                    let mut down_step = 0;
                    while down_y >= 0 && down_y < height && down_x >= 0 && down_x < width {
                        interference_points.insert(Point {
                            y: down_y,
                            x: down_x,
                        });
                        down_step += 1;
                        down_y = antenna_point.y + (diff_y * down_step);
                        down_x = antenna_point.x + (diff_x * down_step);
                    }
                }
            }
            antennas
                .entry(c)
                .and_modify(|points| {
                    points.push(Point {
                        y: current_y_i64,
                        x: current_x_i64,
                    })
                })
                .or_insert(vec![Point {
                    y: current_y_i64,
                    x: current_x_i64,
                }]);
        }
    }
    for y in 0..height {
        for x in 0..width {
            if let Some(a) = antennas
                .iter()
                .find(|(_k, v)| v.iter().find(|p| p.x == x && p.y == y).is_some())
            {
                if let Some(_b) = a.1.iter().find(|p| p.x == x && p.y == y) {
                    print!("A");
                    continue;
                }
            }
            if let Some(_i) = interference_points.iter().find(|p| p.x == x && p.y == y) {
                print!("#");
                continue;
            }
            print!(".");
        }
        print!("\n");
    }

    return interference_points.len();
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    y: i64,
    x: i64,
}
