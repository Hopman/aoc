use std::fs::read_to_string;

pub fn day2() -> u64 {
    let mut result = 0;

    let report_string = read_to_string("input/day2.txt").unwrap();

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
    return result;

}

fn check_report(r: Vec<i64>) -> bool {
    let mut up = true;
    let mut dampened = false;

    for i in 0..(r.len() - 1) {
        let d = r[i + 1] - r[i];
        if i == 0 {
            up = d > 0;
        }
        if d == 0 {
            if dampened {
                return false;
            } else {
                dampened = true;
            }
        }
        if d > 3 || d < -3 {
            if dampened {
                return false;
            } else {
                dampened = true;
            }
        }
        if d < 0 && up {
            if dampened {
                return false;
            } else {
                dampened = true;
            }
        }
        if d > 0 && !up {
            if dampened {
                return false;
            } else {
                dampened = true;
            }
        }
    }

    return true;
}

// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.
