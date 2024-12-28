pub fn add(numbers: &str) -> i32 {
    if numbers.is_empty() {
        return 0;
    }
    1
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn empty_string_is_0() {
        let empty = "";
        assert_eq!(0, add(empty));
    }

    #[test]
    fn string_with_just_a_number_is_that_number() {
        let just_a_number = "1";
        assert_eq!(1, add(just_a_number));
    }
}
