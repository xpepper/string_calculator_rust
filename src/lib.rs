use std::num::ParseIntError;

pub fn add(string_of_numbers: &str) -> Result<i32, AddError> {
    const SEPARATORS: [char; 2] = [',', '\n'];

    if string_of_numbers.is_empty() {
        return Ok(0);
    }

    if string_of_numbers.starts_with("//") {
        if let Some((custom_delimiter, string_of_numbers)) = find_custom_delimiter(string_of_numbers) {
            return string_of_numbers
                .split(custom_delimiter)
                .map(|n| n.trim().parse::<i32>().map_err(AddError::from))
                .sum();
        } else {
            return Err(AddError::CannotFindCustomDelimiter);
        }
    }

    string_of_numbers
        .split(&SEPARATORS)
        .map(|n| n.trim().parse::<i32>().map_err(AddError::from))
        .sum()
}

fn find_custom_delimiter(string_of_numbers: &str) -> Option<(&str,&str)> {
    match string_of_numbers.find('\n') {
        None => None,
        Some(newline_index) => {
            let custom_delimiter = &string_of_numbers[2..newline_index];
            Some((custom_delimiter, &string_of_numbers[newline_index..]))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AddError {
    CannotParseNumber(String),
    CannotFindCustomDelimiter,
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
        assert_eq!(add("//;\n1;2"), Ok(3))
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
