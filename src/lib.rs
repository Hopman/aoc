mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;


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
        let result_test = day4::day4("input/day4_test.txt");
        assert_eq!(result_test, 0);
        let result_example = day4::day4("input/day4_example.txt");
        assert_eq!(result_example, 9);
        let result_one = day4::day4("input/day4.txt");
        assert_eq!(result_one, 1974);
    }

    #[test]
    fn day5_test() {
        let result_test = day5::day5("input/day5_test_order.txt", "input/day5_test_pages.txt");
        assert_eq!(result_test, 5);
        let result_example = day5::day5("input/day5_example_order.txt", "input/day5_example_pages.txt");
        assert_eq!(result_example, 123);
        let result = day5::day5("input/day5_order.txt", "input/day5_pages.txt");
        assert_eq!(result, 4713);
    }

    #[test]
    fn day6_test() {
        let result_example = day6::day6("input/day6_example.txt");
        assert_eq!(result_example, 6);
        let result = day6::day6("input/day6.txt");
        assert_eq!(result, 1753);
    }

    #[test]
    fn day7_example_test() {
        let result_example = day7::day7("input/day7_example.txt");
        assert_eq!(result_example, 11387);
    }

    #[test]
    fn day7_test() {
        let result = day7::day7("input/day7.txt");
        assert_eq!(result, 96779702119491);
    }

    #[test]
    fn day8_example_test() {
        let result_example = day8::day8("input/day8_example.txt");
        assert_eq!(result_example, 34);
    }
    #[test]
    fn day8_test() {
        let result = day8::day8("input/day8.txt");
        assert_eq!(result, 1005);
    }

    #[test]
    fn day9_example_test() {
        let result_example = day9::day9("input/day9_example.txt");
        assert_eq!(result_example, 2858);
    }
    #[test]
    fn day9_test() {
        let result = day9::day9("input/day9.txt");
        assert_eq!(result, 6389911791746);
    }

    #[test]
    fn day10_example_test() {
        let result_example = day10::day10("input/day10_example.txt");
        assert_eq!(result_example, 36);
    }
    #[test]
    fn day10_test() {
        let result = day10::day10("input/day10.txt");
        assert_eq!(result, 737);
    }

    #[test]
    fn day11_example_test() {
        let result_example = day11::day11("input/day11_example.txt");
        assert_eq!(result_example, 55312);
    }
    #[test]
    fn day11_test() {
        let result = day11::day11("input/day11.txt");
        assert_eq!(result, 276661131175807);
    }

    #[test]
    fn day12_example_one_test() {
        let result_example = day12::day12("input/day12_example_one.txt");
        assert_eq!(result_example, 80);
    }
    #[test]
    fn day12_example_two_test() {
        let result_example = day12::day12("input/day12_example_two.txt");
        assert_eq!(result_example, 1206);
    }
    #[test]
    fn day12_test() {
        let result = day12::day12("input/day12.txt");
        assert_eq!(result, 870202);
    }

}

