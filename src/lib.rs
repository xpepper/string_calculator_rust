use std::num::ParseIntError;

pub fn add(numbers: &str) -> Result<i32, AddError> {
    if numbers.is_empty() {
        return Ok(0);
    }

    let result = numbers.parse()?;
    Ok(result)
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
