use std::ops::*;

pub trait Integer:
    Copy
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
    + From<u8>
    + std::fmt::Debug
    + std::fmt::Display
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const TEN: Self;
}

pub trait Signed: Integer + Neg<Output = Self> + From<i8> {}
pub trait Unsigned: Integer + From<u8> {}

macro_rules! impl_integer {
    ($($t:ty),*) => {
        $(
        impl Integer for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const TWO: Self = 2;
            const TEN: Self = 10;
        })*
    };
}

macro_rules! impl_trait {
    ($trait:ty, $($t:ty),*) => {
        $(impl $trait for $t {})*
    };
}

impl_integer!(u8, u16, i16, u32, i32, u64, i64, u128, i128);
impl_trait!(Signed, i16, i32, i64, i128);
impl_trait!(Unsigned, u8, u16, u32, u64, u128);
