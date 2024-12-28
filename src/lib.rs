use std::num::ParseIntError;

pub fn add(string_of_numbers: &str) -> Result<i32, AddError> {
    if string_of_numbers.is_empty() {
        return Ok(0);
    }

    let sum = string_of_numbers
        .split(",")
        .collect::<Vec<_>>()
        .into_iter()
        .map(|n| n.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?
        .iter()
        .sum();

    Ok(sum)
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
        assert_eq!(Ok(0), add(empty));
    }

    #[test]
    fn string_with_just_a_number_is_that_number() {
        let just_a_number = "1";
        assert_eq!(Ok(1), add(just_a_number));
    }

    #[test]
    fn sum_two_numbers_separated_by_comma() {
        let two_numbers = "1,2";
        assert_eq!(Ok(3), add(two_numbers));
    }

    #[test]
    fn cannot_add_string_containing_unparsable_numbers() {
        let not_a_number = "ABC";
        assert_eq!(
            Err(AddError::CannotParseNumber(
                "invalid digit found in string".to_string()
            )),
            add(not_a_number)
        );
    }
}
