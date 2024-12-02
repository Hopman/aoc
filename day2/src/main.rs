use std::fs::read_to_string;

fn main() {
    let mut result = 0;

    let report_string = read_to_string("input.txt").unwrap();

    let reports: Vec<Vec<i64>> = report_string
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    for r in reports {
        let safe = check_report(r);
        if safe {
            result += 1;
        }
    }

    println!("Result: {result}");
}

fn check_report(r: Vec<i64>) -> bool {
    let mut up = true;
    for i in 0..(r.len() - 1) {
        let d = r[i + 1] - r[i];
        if i == 0 {
            up = d > 0;
        }
        if d == 0 {
            return false;
        }
        if d > 3 || d < -3 {
            return false;
        }
        if d < 0 && up {
            return false;
        }
        if d > 0 && !up {
            return false;
        }
    }
    return true;
}

// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.
