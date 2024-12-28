pub fn add(_numbers: String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn empty_string_is_0() {
        let numbers = String::from("");
        assert_eq!(0, add(numbers));
    }
}
