//! Helper functions to parse strings

use crate::util;

/// Given a string of the form "Lorem ipsum dolor1234", returns the number 1234
pub fn parse_number<T: std::str::FromStr>(s: &str) -> Option<T> {
    s.chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
}

pub struct ParseUnsigned<'a, T: util::integer::Unsigned> {
    bytes: std::str::Bytes<'a>,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, T: util::integer::Unsigned> ParseUnsigned<'a, T> {
    pub fn new(bytes: std::str::Bytes<'a>) -> Self {
        Self { bytes, phantom: std::marker::PhantomData }
    }
}

impl<T: util::integer::Unsigned> Iterator for ParseUnsigned<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        try_unsigned::<T>(&mut self.bytes)
    }
}

pub fn try_unsigned<T: util::integer::Unsigned>(
    bytes: &mut std::str::Bytes,
) -> Option<T> {
    // Take while not digits
    let mut n = loop {
        let byte = bytes.next()?;
        let digit = byte.wrapping_sub(b'0');
        if digit < 10 {
            break T::from(digit);
        }
    };

    // Take while digits
    loop {
        let Some(byte) = bytes.next() else { break Some(n) };
        let digit = byte.wrapping_sub(b'0');

        if digit < 10 {
            n = T::TEN * n + T::from(digit);
        } else {
            break Some(n);
        }
    }
}
