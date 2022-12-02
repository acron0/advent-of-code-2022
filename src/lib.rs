#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
mod day1;
mod day2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_1() {
        assert_eq!(day1::run1(), day1::EXPECTED_RESULT1);
    }
    #[test]
    fn day1_2() {
        assert_eq!(day1::run2(), day1::EXPECTED_RESULT2);
    }

    #[test]
    fn day2() {
        let result = day2::run();
        assert_eq!(result, day2::EXPECTED_RESULT);
    }
}
