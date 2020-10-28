pub fn extract_digits(s: &str) -> Result<(&str, &str), String> {
    take_while_1(|c| c.is_ascii_digit(), s, "expected digits".to_string())
}

pub fn extract_ident(s: &str) -> (&str, &str) {
    let input_starts_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);

    if input_starts_with_alphabetic {
        take_while(|c| c.is_alphanumeric(), s)
    } else {
        ("", s)
    }
}

pub fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| c == ' ', s)
}

pub fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> Result<&'b str, String> {
    match s.strip_prefix(starting_text) {
        Some(s) => Ok(s),
        None => Err(format!("Expected {}", starting_text)),
    }
}

fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];

    (extracted, remainder)
}

fn take_while_1(
    accept: impl Fn(char) -> bool,
    s: &str,
    error_msg: String,
) -> Result<(&str, &str), String> {
    let (extracted, remainder) = take_while(accept, s);

    if extracted.is_empty() {
        Err(error_msg)
    } else {
        Ok((extracted, remainder))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("1", "+2")));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10-20"), Ok(("10", "-20")));
    }

    #[test]
    fn do_not_extract_digits_when_input_is_invalid() {
        assert_eq!(extract_digits("abcd"), Err("expected digits".to_string()));
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), Ok(("100", "")));
    }

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("    1"), ("    ", "1"));
    }

    #[test]
    fn extract_alphabetic_ident() {
        assert_eq!(extract_ident("abcdEFG stop"), ("abcdEFG", " stop"));
    }

    #[test]
    fn extract_alphanumeric_ident() {
        assert_eq!(extract_ident("foobar1()"), ("foobar1", "()"));
    }

    #[test]
    fn cannot_extract_ident_beginning_with_number() {
        assert_eq!(extract_ident("123abc"), ("", "123abc"));
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), Ok(" a"));
    }
}
