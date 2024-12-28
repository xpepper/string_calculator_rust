pub fn add(_numbers: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn empty_string_is_0() {
        let empty = "";
        assert_eq!(0, add(empty));
    }
}
