
extern crate num_complex;
extern crate num_traits;

use num_complex::Complex;
use num_traits::Float;
use std::ops::*;

/// Ring (math)
pub trait Ring:
    Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self>
    + AddAssign + SubAssign + MulAssign + Sized {}
impl <A> Ring for A
where A: Add<Output=A> + Sub<Output=A> + Mul<Output=A>
    + AddAssign + SubAssign + MulAssign + Sized {}

/// R-module
pub trait RMod<R: Ring>: Mul<R, Output = Self> + MulAssign<R> + Sized {}
impl<A, R: Ring> RMod<R> for A where A: Mul<R, Output = A> + MulAssign<R> + Sized {}

/// exponential function
pub trait Exponential: Clone + Copy + Sized {
    fn exp(self) -> Self;
}

impl Exponential for f32 {
    fn exp(self) -> Self {
        <Self>::exp(self)
    }
}

impl Exponential for f64 {
    fn exp(self) -> Self {
        <Self>::exp(self)
    }
}

impl<T> Exponential for Complex<T>
    where T: Clone + Float
{
    fn exp(self) -> Self {
        <Complex<T>>::exp(&self)
    }
}
