mod day1;
mod day2;
mod day3;
mod day4;


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
        let result_one = day3::day3("input/day3.txt");
        assert_eq!(result_one, 166630675);

        // day3.2.txt = day3.txt with some execuded vim macros:
        // 1. find "don't()"
        // 2. go into visual mode
        // 3. find "do()"
        // 4. delete selected characters
        let result_two = day3::day3("input/day3.2.txt");
        assert_eq!(result_two, 93465710);
    }

    #[test]
    fn day4_test() {
        let result_example = day4::day4("input/day4_example.txt");
        assert_eq!(result_example, 3);
        let result_one = day4::day4("input/day4.txt");
        assert_eq!(result_one, 42);
    }
}




