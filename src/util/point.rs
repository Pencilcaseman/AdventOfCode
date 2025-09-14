use num_traits::{Signed, WrappingNeg};
pub use num_traits::{WrappingAdd, WrappingSub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point2D<T> {
    pub row: T,
    pub col: T,
}

impl<T> Point2D<T> {
    pub fn new(row: T, col: T) -> Self {
        Self { row, col }
    }

    pub fn tuple(self) -> (T, T) {
        (self.row, self.col)
    }
}

impl<T> Point2D<T>
where
    T: num_traits::int::PrimInt + WrappingAdd + WrappingSub,
{
    pub fn orthogonal() -> [Self; 4] {
        let zero = T::zero();
        let one = T::one();
        let minus_one = zero.wrapping_sub(&one);

        [
            Self::new(minus_one, zero),
            Self::new(one, zero),
            Self::new(zero, minus_one),
            Self::new(zero, one),
        ]
    }

    pub fn wrapping_add_dir(&self, v: &Direction) -> Self {
        <Self as WrappingAdd>::wrapping_add(self, &Self::from(*v))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    pub const fn clockwise(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub const fn counter_clockwise(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    pub fn orthogonal() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }
}

impl<T> From<Direction> for (T, T)
where
    T: Signed + From<i8> + Copy,
{
    fn from(dir: Direction) -> Self {
        let zero = T::from(0i8);
        let one = T::from(1i8);
        let minus_one = -one;

        match dir {
            Direction::Up => (minus_one, zero),
            Direction::Down => (one, zero),
            Direction::Left => (zero, minus_one),
            Direction::Right => (zero, one),
        }
    }
}

impl<T> From<(T, T)> for Direction
where
    T: Signed + From<i8> + Copy,
{
    fn from(dir: (T, T)) -> Self {
        let zero = T::from(0i8);
        let one = T::from(1i8);
        let minus_one = -one;

        match dir {
            (a, b) if a == minus_one && b == zero => Self::Up,
            (a, b) if a == one && b == zero => Self::Down,
            (a, b) if a == zero && b == minus_one => Self::Left,
            (a, b) if a == zero && b == one => Self::Right,
            _ => panic!("Invalid Direction"),
        }
    }
}

impl<T> From<Direction> for Point2D<T>
where
    T: num_traits::int::PrimInt + WrappingAdd + WrappingSub,
{
    fn from(dir: Direction) -> Self {
        Point2D::<T>::orthogonal()[dir as usize]
    }
}

impl<T> From<(T, T)> for Point2D<T> {
    fn from(dir: (T, T)) -> Self {
        Self::new(dir.0, dir.1)
    }
}

impl<T> std::ops::Add<Direction> for Point2D<T>
where
    T: std::ops::Add<Output = T>,
    T: std::ops::Sub<Output = T>,
    T: From<u8>,
{
    type Output = Self;

    fn add(self, dir: Direction) -> Self::Output {
        let one = T::from(1u8);

        match dir {
            Direction::Up => Self::new(self.row - one, self.col),
            Direction::Down => Self::new(self.row + one, self.col),
            Direction::Left => Self::new(self.row, self.col - one),
            Direction::Right => Self::new(self.row, self.col + one),
        }
    }
}

impl<T> std::ops::Neg for Point2D<T>
where
    T: std::ops::Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { row: -self.row, col: -self.col }
    }
}

impl<T> WrappingNeg for Point2D<T>
where
    T: WrappingNeg,
{
    fn wrapping_neg(&self) -> Self {
        Self { row: self.row.wrapping_neg(), col: self.col.wrapping_neg() }
    }
}

impl<T> std::ops::AddAssign<Direction> for Point2D<T>
where
    T: std::ops::Add<Output = T>,
    T: std::ops::Sub<Output = T>,
    T: From<u8>,
    T: Copy,
{
    fn add_assign(&mut self, dir: Direction) {
        *self = *self + dir;
    }
}

impl<T> std::ops::Add for Point2D<T>
where
    T: Copy + std::ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, v: Self) -> Self {
        Self::new(self.row + v.row, self.col + v.col)
    }
}

impl<T> WrappingAdd for Point2D<T>
where
    T: Copy + WrappingAdd,
{
    fn wrapping_add(&self, v: &Self) -> Self {
        Self::new(self.row.wrapping_add(&v.row), self.col.wrapping_add(&v.col))
    }
}

impl<T> std::ops::Sub<Direction> for Point2D<T>
where
    T: std::ops::Add<Output = T>,
    T: std::ops::Sub<Output = T>,
    T: From<u8>,
{
    type Output = Self;

    fn sub(self, dir: Direction) -> Self::Output {
        let one = T::from(1u8);

        match dir {
            Direction::Up => Self::new(self.row + one, self.col),
            Direction::Down => Self::new(self.row - one, self.col),
            Direction::Left => Self::new(self.row, self.col + one),
            Direction::Right => Self::new(self.row, self.col - one),
        }
    }
}

impl<T> std::ops::SubAssign<Direction> for Point2D<T>
where
    T: std::ops::Add<Output = T>,
    T: std::ops::Sub<Output = T>,
    T: From<u8>,
    T: Copy,
{
    fn sub_assign(&mut self, dir: Direction) {
        *self = *self - dir;
    }
}
