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

pub struct ParseSigned<'a, T: util::integer::Signed> {
    bytes: std::str::Bytes<'a>,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, T: util::integer::Unsigned> ParseUnsigned<'a, T> {
    pub fn new(bytes: std::str::Bytes<'a>) -> Self {
        Self { bytes, phantom: std::marker::PhantomData }
    }
}

impl<'a, T: util::integer::Signed> ParseSigned<'a, T> {
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

impl<T: util::integer::Signed> Iterator for ParseSigned<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        try_signed::<T>(&mut self.bytes)
    }
}

pub fn try_unsigned_immediate<T: util::integer::Unsigned>(
    bytes: &mut std::str::Bytes,
) -> Option<T> {
    let byte = bytes.next()?;
    let digit = byte.wrapping_sub(b'0');

    if digit >= 10 {
        return None;
    }

    let mut n: T = T::from(digit);

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

pub fn try_unsigned_immediate_with_final_byte<T: util::integer::Unsigned>(
    bytes: &mut std::str::Bytes,
) -> (Option<T>, Option<u8>) {
    let Some(byte) = bytes.next() else { return (None, None) };
    let digit = byte.wrapping_sub(b'0');

    if digit >= 10 {
        return (None, Some(byte));
    }

    let mut n: T = T::from(digit);

    loop {
        let Some(byte) = bytes.next() else { break (Some(n), None) };
        let digit = byte.wrapping_sub(b'0');

        if digit < 10 {
            n = T::TEN * n + T::from(digit);
        } else {
            break (Some(n), Some(byte));
        }
    }
}

pub fn try_unsigned<T: util::integer::Unsigned>(
    bytes: &mut std::str::Bytes,
) -> Option<T> {
    let mut n = loop {
        let byte = bytes.next()?;
        let digit = byte.wrapping_sub(b'0');
        if digit < 10 {
            break T::from(digit);
        }
    };

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

pub fn try_signed_immediate<T: util::integer::Signed>(
    bytes: &mut std::str::Bytes,
) -> Option<T> {
    let mut byte = bytes.next()?;
    let mut sign = false;

    if byte == b'-' {
        sign = true;
        byte = bytes.next()?;
    } else if byte == b'+' {
        byte = bytes.next()?;
    }

    let digit = byte.wrapping_sub(b'0');

    if digit >= 10 {
        return None;
    }

    let mut n: T = T::from(digit);

    let res = loop {
        let Some(byte) = bytes.next() else { break n };
        let digit = byte.wrapping_sub(b'0');

        if digit < 10 {
            n = T::TEN * n + T::from(digit);
        } else {
            break n;
        }
    };

    Some(if sign { -res } else { res })
}

pub fn try_signed<T: util::integer::Signed>(
    bytes: &mut std::str::Bytes,
) -> Option<T> {
    // Take while not digits
    let (mut n, sign) = loop {
        let byte = bytes.next()?;

        if byte == b'-' {
            break (T::ZERO, true);
        } else if byte == b'+' {
            break (T::ZERO, false);
        }

        let digit = byte.wrapping_sub(b'0');

        if digit < 10 {
            break (T::from(digit), false);
        }
    };

    // Take while digits
    let res = loop {
        let Some(byte) = bytes.next() else {
            break n;
        };
        let digit = byte.wrapping_sub(b'0');

        if digit < 10 {
            n = T::TEN * n + T::from(digit);
        } else {
            break n;
        }
    };

    Some(if sign { -res } else { res })
}
