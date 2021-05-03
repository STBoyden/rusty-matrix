use std::{
    fmt::{Debug, Display},
    ops::*,
};

pub trait Numeric = Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + PartialEq
    + Sized
    + Copy
    + Debug
    + Display
    + Default;
