use std::num::ParseIntError;

const DEFAULT_SEPARATORS: [char; 2] = [',', '\n'];
pub fn add(string_of_numbers: &str) -> Result<i32, AddError> {
    if string_of_numbers.is_empty() {
        return Ok(0);
    }

    let numbers = numbers_from(string_of_numbers)?;
    let negatives = find_negatives_in(&numbers);
    if !negatives.is_empty() {
        return Err(AddError::NegativeNumbersNotAllowed(negatives));
    }

    Ok(numbers.into_iter().filter(|number| *number <= 1000).sum())
}

fn find_negatives_in(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .filter(|number| number.is_negative())
        .cloned()
        .collect::<Vec<i32>>()
}

fn numbers_from(string_of_numbers: &str) -> Result<Vec<i32>, AddError> {
    parse(string_of_numbers)?
        .into_iter()
        .map(|number| number.trim().parse::<i32>().map_err(AddError::from))
        .collect::<Result<Vec<i32>, AddError>>()
}

fn parse(string_of_numbers: &str) -> Result<Vec<&str>, AddError> {
    if has_custom_delimiter(string_of_numbers) {
        if let Some((custom_delimiter, string_of_numbers)) =
            find_custom_delimiter(string_of_numbers)
        {
            Ok(string_of_numbers
                .split(custom_delimiter)
                .collect::<Vec<&str>>())
        } else {
            Err(AddError::CannotFindCustomDelimiter)
        }
    } else {
        Ok(string_of_numbers
            .split(&DEFAULT_SEPARATORS)
            .collect::<Vec<&str>>())
    }
}

fn has_custom_delimiter(string_of_numbers: &str) -> bool {
    string_of_numbers.starts_with("//")
}

fn find_custom_delimiter(string_of_numbers: &str) -> Option<(&str, &str)> {
    if string_of_numbers.starts_with("//\n") {
        return None;
    }

    string_of_numbers.strip_prefix("//[").and_then(|rest| {
        let newline_index = rest.find("]\n")?;
        let delimiter = &rest[..newline_index];
        let numbers = &rest[newline_index + 1..];
        Some((delimiter, numbers))
    })
}

#[derive(Debug, PartialEq, Eq)]
pub enum AddError {
    CannotParseNumber(String),
    CannotFindCustomDelimiter,
    NegativeNumbersNotAllowed(Vec<i32>),
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
    fn new_line_is_an_alternative_separator() {
        assert_eq!(add("1\n2,3,4"), Ok(1 + 2 + 3 + 4));
    }

    #[test]
    fn remove_trailing_spaces_before_adding() {
        assert_eq!(add("1,  2"), Ok(1 + 2));
    }

    #[test]
    fn support_different_delimiters() {
        assert_eq!(add("//[;]\n1;2"), Ok(3));
        assert_eq!(add("//[|]\n1|2"), Ok(3));
        assert_eq!(add("//[==]\n1==2"), Ok(3));
        assert_eq!(add("//[ ]\n1 2"), Ok(3));
    }

    #[test]
    fn does_not_allow_negative_numbers() {
        assert_eq!(
            add("1,-2"),
            Err(AddError::NegativeNumbersNotAllowed(vec![-2]))
        );
        assert_eq!(
            add("//[;]\n-1;-2"),
            Err(AddError::NegativeNumbersNotAllowed(vec![-1, -2]))
        );
    }

    #[test]
    fn numbers_bigger_than_1000_are_ignored() {
        assert_eq!(add("1,1001,2"), Ok(1 + 2));
    }

    #[test]
    fn delimiters_can_be_of_any_length() {
        // Delimiters can be of any length with the following format:
        // “//[delimiter]\n” for example: “//[***]\n1***2***3” should return 6
        assert_eq!(add("//[***]\n1***2***3"), Ok(1 + 2 + 3));
    }

    #[test]
    fn cannot_find_customer_delimiter() {
        assert_eq!(add("//\n1;2"), Err(AddError::CannotFindCustomDelimiter));
        assert_eq!(add("//1;2"), Err(AddError::CannotFindCustomDelimiter));
        assert_eq!(add("//   1;2"), Err(AddError::CannotFindCustomDelimiter));
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
