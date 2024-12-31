use regex::Regex;
use std::num::ParseIntError;

const DEFAULT_SEPARATORS: [&str; 2] = [",", "\n"];
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
        split_with_custom_delimiters(string_of_numbers)
    } else {
        split_with_delimiters(string_of_numbers, DEFAULT_SEPARATORS.to_vec())
    }
}

fn split_with_custom_delimiters(string_of_numbers: &str) -> Result<Vec<&str>, AddError> {
    if let Some((custom_delimiters, string_of_numbers)) = find_custom_delimiters(string_of_numbers)
    {
        split_with_delimiters(string_of_numbers, custom_delimiters)
    } else {
        Err(AddError::CannotFindCustomDelimiter)
    }
}

fn split_with_delimiters<'a>(
    string_of_numbers: &'a str,
    custom_delimiters: Vec<&'a str>,
) -> Result<Vec<&'a str>, AddError> {
    let delimiters_regexp = Regex::new(
        custom_delimiters
            .into_iter()
            .map(regex::escape)
            .collect::<Vec<String>>()
            .join("|")
            .as_str(),
    )
    .unwrap();
    Ok(delimiters_regexp
        .split(string_of_numbers)
        .collect::<Vec<&str>>())
}

fn has_custom_delimiter(string_of_numbers: &str) -> bool {
    string_of_numbers.starts_with("//")
}

fn find_custom_delimiters(string_of_numbers: &str) -> Option<(Vec<&str>, &str)> {
    if string_of_numbers.starts_with("//\n") {
        return None;
    }

    string_of_numbers.strip_prefix("//").and_then(|rest| {
        let newline_index = rest.find("\n")?;
        let delimiters = &rest[..newline_index];
        let numbers = &rest[newline_index + 1..];
        Some((extract_delimiters_from(delimiters), numbers))
    })
}

fn extract_delimiters_from(string_of_delimiters: &str) -> Vec<&str> {
    Regex::new(r"\[(.*?)]")
        .unwrap()
        .captures_iter(string_of_delimiters)
        .map(|captures| captures.get(1).unwrap().as_str())
        .collect()
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
        assert_eq!(add("//[*]\n1*2"), Ok(3));
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
        assert_eq!(add("//[***]\n1***2***3"), Ok(1 + 2 + 3));
    }

    #[test]
    fn allow_multiple_delimiters() {
        //Allow multiple delimiters like this: “//[delim1][delim2]\n” for example “//[*][%]\n1*2%3”
        assert_eq!(add("//[;][,]\n1;2,3"), Ok(1 + 2 + 3));
        assert_eq!(add("//[*][%]\n1*2%3"), Ok(1 + 2 + 3));
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
