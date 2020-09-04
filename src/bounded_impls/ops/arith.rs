use crate::{expr::AsBound, Bounded};
use core::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
    time::Duration,
};

impl_bounded_for_bounded! { Add::add }

impl_ops_reflective! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :Add::add
}

impl_assign! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :AddAssign::add_assign
}

impl_bounded_for_bounded! { Sub::sub }

impl_ops_reflective! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :Sub::sub
}

impl_assign! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :SubAssign::sub_assign
}

impl_bounded_for_bounded! { Mul::mul }

impl_ops_reflective! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :Mul::mul
}

impl_assign! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :MulAssign::mul_assign
}

impl_bounded_for_bounded! { Div::div }

impl_ops_reflective! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :Div::div
}

impl_assign! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :DivAssign::div_assign
}

impl_bounded_for_bounded! { Rem::rem }

impl_ops_reflective! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :Rem::rem
}

impl_assign! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :RemAssign::rem_assign
}

impl<TL, BL> Mul<Duration> for Bounded<TL, BL>
where
    BL: AsBound<TL>,
    TL: Mul<Duration>,
{
    type Output = <TL as Mul<Duration>>::Output;

    #[inline]
    fn mul(self, rhs: Duration) -> Self::Output {
        self.value().mul(rhs)
    }
}

impl<TR, BR> Mul<Bounded<TR, BR>> for Duration
where
    BR: AsBound<TR>,
    Duration: Mul<TR>,
{
    type Output = <Duration as Mul<TR>>::Output;

    #[inline]
    fn mul(self, rhs: Bounded<TR, BR>) -> Self::Output {
        self.mul(rhs.value())
    }
}

impl<T, B> Neg for Bounded<T, B>
where
    B: AsBound<T>,
    T: Neg,
{
    type Output = <T as Neg>::Output;

    #[inline]
    fn neg(self) -> Self::Output {
        self.value().neg()
    }
}
