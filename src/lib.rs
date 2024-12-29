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
        assert_eq!(add(""), Ok(0));
    }

    #[test]
    fn string_with_just_a_number_is_that_number() {
        assert_eq!(add("1"), Ok(1));
    }

    #[test]
    fn sum_multiple_numbers_separated_by_comma() {
        assert_eq!(add("1,2,3,4"), Ok(1 + 2 + 3 + 4));
    }

    #[test]
    fn cannot_add_string_containing_unparsable_numbers() {
        assert_eq!(
            add("ABC,2"),
            Err(AddError::CannotParseNumber(
                "invalid digit found in string".to_string()
            ))
        );
    }
}
