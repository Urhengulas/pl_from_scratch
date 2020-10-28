pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    let digits_end = s
        .char_indices()
        .find_map(|(idx, c)| if c.is_ascii_digit() { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let digits = &s[..digits_end];
    let remainder = &s[digits_end..];

    (digits, remainder)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    let op = &s[0..1];
    match op {
        "+" | "-" | "*" | "/" => (op, &s[1..]),
        _ => panic!("Couldn't extract operator: {}", op),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), ("1", "+2"));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10-20"), ("10", "-20"));
    }

    #[test]
    fn do_not_extract_anything_from_empty_input() {
        assert_eq!(extract_digits(""), ("", ""));
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), ("100", ""));
    }

    #[test]
    fn extract_plus() {
        assert_eq!(extract_op("+2"), ("+", "2"));
    }

    #[test]
    fn extract_minus() {
        assert_eq!(extract_op("-10"), ("-", "10"));
    }

    #[test]
    fn extract_star() {
        assert_eq!(extract_op("*3"), ("*", "3"));
    }

    #[test]
    fn extract_slash() {
        assert_eq!(extract_op("/4"), ("/", "4"));
    }
}
