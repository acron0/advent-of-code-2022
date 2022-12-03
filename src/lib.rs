#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
mod day1;
mod day2;
mod day3;

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
    fn day2_1() {
        assert_eq!(day2::run1(), day2::EXPECTED_RESULT1);
    }

    #[test]
    fn day2_2() {
        assert_eq!(day2::run2(), day2::EXPECTED_RESULT2);
    }

    #[test]
    fn day3_1() {
        assert_eq!(day3::run1(), day3::EXPECTED_RESULT1);
    }

    #[test]
    fn day3_2() {
        assert_eq!(day3::run2(), day3::EXPECTED_RESULT2);
    }
}
