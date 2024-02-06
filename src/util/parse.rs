//! Helper functions to parse strings

/// Given a string of the form "Lorem ipsum dolor1234", returns the number 1234
pub fn parse_number<T: std::str::FromStr>(s: &str) -> Option<T> {
    s.chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
}
