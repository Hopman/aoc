mod day1;
mod day2;
mod day3;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_test() {
        let result = day1::day1();
        assert_eq!(result, 21024792);
    }

    #[test]
    fn day2_test() {
        let result = day2::day2();
        assert_eq!(result, 271);
    }

    #[test]
    fn day3_test() {
        let result = day3::day3();
        assert_eq!(result, 42);
    }
}


