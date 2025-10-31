//! Helper functions to parse strings

use std::borrow::Borrow;

use crate::util;

pub fn grid_to_ndarray<I>(grid: I) -> ndarray::Array2<u8>
where
    I: Iterator,
    I::Item: Borrow<u8>,
{
    let mut data = Vec::new();
    let mut rows = 1;
    let mut cols = None;

    let mut current_col = 0;
    let mut idx = 0;

    for byte in grid {
        if *byte.borrow() == b'\n' {
            if current_col == 0 {
                rows -= 1;
                break;
            }

            if cols.is_none() {
                cols = Some(idx);
            }

            rows += 1;
            current_col = 0;

            continue;
        }

        data.push(*byte.borrow());
        idx += 1;
        current_col += 1;
    }

    unsafe {
        ndarray::Array2::from_shape_vec_unchecked(
            (rows, cols.unwrap_or(idx)),
            data,
        )
    }
}

/// Given a string of the form "Lorem ipsum dolor1234", returns the number 1234
pub fn parse_number<T: std::str::FromStr>(s: &str) -> Option<T> {
    s.chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
}

pub struct ParseUnsigned<'a, T, I = std::str::Bytes<'a>>
where
    T: util::integer::Unsigned,
    I: Iterator,
    I::Item: Borrow<u8>,
{
    bytes: I,
    phantom: std::marker::PhantomData<&'a T>,
}

pub struct ParseSigned<'a, T, I = std::str::Bytes<'a>>
where
    T: util::integer::Signed,
    I: Iterator,
    I::Item: Borrow<u8>,
{
    bytes: I,
    phantom: std::marker::PhantomData<&'a T>,
}

impl<T, I> ParseUnsigned<'_, T, I>
where
    T: util::integer::Unsigned,
    I: Iterator,
    I::Item: Borrow<u8>,
{
    pub fn new(bytes: I) -> Self {
        Self { bytes, phantom: std::marker::PhantomData }
    }
}

impl<T, I> ParseSigned<'_, T, I>
where
    T: util::integer::Signed,
    I: Iterator,
    I::Item: Borrow<u8>,
{
    pub fn new(bytes: I) -> Self {
        Self { bytes, phantom: std::marker::PhantomData }
    }
}

impl<T, I> Iterator for ParseUnsigned<'_, T, I>
where
    T: util::integer::Unsigned,
    I: Iterator,
    I::Item: Borrow<u8>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        try_unsigned::<T, I>(&mut self.bytes)
    }
}

impl<T, I> Iterator for ParseSigned<'_, T, I>
where
    T: util::integer::Signed,
    I: Iterator,
    I::Item: Borrow<u8>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        try_signed::<T, I>(&mut self.bytes)
    }
}

pub fn try_unsigned_immediate<T: util::integer::Unsigned, I>(
    bytes: &mut I,
) -> Option<T>
where
    I: Iterator,
    I::Item: Borrow<u8>,
{
    let byte = bytes.next()?;
    let digit = byte.borrow().wrapping_sub(b'0');

    if digit >= 10 {
        return None;
    }

    let mut n: T = T::from(digit);

    loop {
        let Some(byte) = bytes.next() else { break Some(n) };
        let digit = byte.borrow().wrapping_sub(b'0');

        if digit < 10 {
            n = T::TEN * n + T::from(digit);
        } else {
            break Some(n);
        }
    }
}

pub fn try_unsigned_immediate_with_final_byte<T, I>(
    bytes: &mut I,
) -> (Option<T>, Option<u8>)
where
    T: util::integer::Unsigned,
    I: Iterator,
    I::Item: Borrow<u8>,
{
    let Some(byte) = bytes.next() else { return (None, None) };
    let digit = byte.borrow().wrapping_sub(b'0');

    if digit >= 10 {
        return (None, Some(*byte.borrow()));
    }

    let mut n: T = T::from(digit);

    loop {
        let Some(byte) = bytes.next() else { break (Some(n), None) };
        let digit = byte.borrow().wrapping_sub(b'0');

        if digit < 10 {
            n = T::TEN * n + T::from(digit);
        } else {
            break (Some(n), Some(*byte.borrow()));
        }
    }
}

pub fn try_unsigned<T: util::integer::Unsigned, I>(bytes: &mut I) -> Option<T>
where
    I: Iterator,
    I::Item: Borrow<u8>,
{
    let mut n = loop {
        let byte = bytes.next()?;
        let digit = byte.borrow().wrapping_sub(b'0');
        if digit < 10 {
            break T::from(digit);
        }
    };

    loop {
        let Some(byte) = bytes.next() else { break Some(n) };
        let digit = byte.borrow().wrapping_sub(b'0');

        if digit < 10 {
            n = T::TEN * n + T::from(digit);
        } else {
            break Some(n);
        }
    }
}

pub fn try_signed_immediate<T: util::integer::Signed, I>(
    bytes: &mut I,
) -> Option<T>
where
    I: Iterator,
    I::Item: Borrow<u8>,
{
    let mut byte = *(bytes.next()?.borrow());
    let mut sign = false;

    if byte == b'-' {
        sign = true;
        byte = *(bytes.next()?.borrow());
    } else if byte == b'+' {
        byte = *(bytes.next()?.borrow());
    }

    let digit = byte.wrapping_sub(b'0');

    if digit >= 10 {
        return None;
    }

    let mut n: T = T::from(digit);

    let res = loop {
        let Some(byte) = bytes.next() else { break n };
        let digit = byte.borrow().wrapping_sub(b'0');

        if digit < 10 {
            n = T::TEN * n + T::from(digit);
        } else {
            break n;
        }
    };

    Some(if sign { -res } else { res })
}

pub fn try_signed<T: util::integer::Signed, I>(bytes: &mut I) -> Option<T>
where
    I: Iterator,
    I::Item: Borrow<u8>,
{
    // Take while not digits
    let (mut n, sign) = loop {
        let byte = *(bytes.next()?.borrow());

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
        let digit = byte.borrow().wrapping_sub(b'0');

        if digit < 10 {
            n = T::TEN * n + T::from(digit);
        } else {
            break n;
        }
    };

    Some(if sign { -res } else { res })
}

pub trait ParseOps {
    fn iter_unsigned<T: util::integer::Unsigned>(&self)
    -> ParseUnsigned<'_, T>;
    fn iter_signed<T: util::integer::Signed>(&self) -> ParseSigned<'_, T>;
}

impl ParseOps for &str {
    fn iter_unsigned<T: util::integer::Unsigned>(
        &self,
    ) -> ParseUnsigned<'_, T> {
        ParseUnsigned::new(self.bytes())
    }

    fn iter_signed<T: util::integer::Signed>(&self) -> ParseSigned<'_, T> {
        ParseSigned::new(self.bytes())
    }
}
