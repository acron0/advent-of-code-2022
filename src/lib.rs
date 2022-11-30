#![allow(dead_code)]
#![allow(unused_variables)]
mod day1;
mod day2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        let result = day1::run();
        assert_eq!(result, day1::EXPECTED_RESULT);
    }

    #[test]
    fn day2() {
        let result = day2::run();
        assert_eq!(result, day2::EXPECTED_RESULT);
    }
}
