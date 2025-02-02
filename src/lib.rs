pub fn add(string_of_numbers: &str) -> i32 {
    if string_of_numbers.is_empty() {
        return 0;
    }

    string_of_numbers
        .split(",")
        .map(|each_string| each_string.trim().parse::<i32>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_adds_to_zero() {
        assert_eq!(add(""), 0);
    }

    #[test]
    fn string_with_single_number_adds_to_just_that_number() {
        assert_eq!(add("1"), 1);
    }

    #[test]
    fn string_with_comma_separated_numbers_adds_those_numbers() {
        assert_eq!(add("1,2"), 1 + 2);
        assert_eq!(add("1,2,3"), 1 + 2 + 3);
    }
    #[test]
    fn ignore_trimming_spaces_between_numbers() {
        assert_eq!(add(" 1  , 2     ,3 "), 1 + 2 + 3);
    }
}
