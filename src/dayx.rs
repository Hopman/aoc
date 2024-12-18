use std::fs::read_to_string;

pub fn DAYX(inputpath: &str) -> usize {
    let mut result = 0;
    let text = read_to_string(inputpath).unwrap();

    return result;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn DAYX_example_test() {
        let result_example = DAYX::DAYX("input/DAYX_example.txt");
        assert_eq!(result_example, 42);
    }
    #[test]
    fn DAYX_test() {
        let result = DAYX::DAYX("input/DAYX.txt");
        assert_eq!(result, 42);
    }
}
