use std::num::ParseIntError;

pub fn add(string_of_numbers: &str) -> Result<i32, AddError> {
    if string_of_numbers.is_empty() {
        return Ok(0);
    }

    string_of_numbers
        .split(",")
        .map(|n| n.parse::<i32>().map_err(AddError::from))
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
pub enum AddError {
    CannotParseNumber(String),
}

impl From<ParseIntError> for AddError {
    fn from(value: ParseIntError) -> Self {
        AddError::CannotParseNumber(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_is_0() {
        let empty = "";
        assert_eq!(add(empty), Ok(0));
    }

    #[test]
    fn string_with_just_a_number_is_that_number() {
        let just_a_number = "1";
        assert_eq!(add(just_a_number), Ok(1));
    }

    #[test]
    fn sum_two_numbers_separated_by_comma() {
        let two_numbers = "1,2";
        assert_eq!(add(two_numbers), Ok(3));
    }

    #[test]
    fn cannot_add_string_containing_unparsable_numbers() {
        let not_a_number = "ABC";
        assert_eq!(
            add(not_a_number),
            Err(AddError::CannotParseNumber(
                "invalid digit found in string".to_string()
            ))
        );
    }
}
