fn first_char(str: String) -> Option<char> {
    return str.chars().nth(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_char() {
        assert_eq!(Some('E'), first_char(String::from("Emily")));
    }
}
