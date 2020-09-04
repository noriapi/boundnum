use crate::{AsBound, Bounded};
use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

impl_bounded_for_bounded! { BitAnd::bitand }

impl_ops_reflective! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :BitAnd::bitand
}

impl_assign! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :BitAndAssign::bitand_assign
}

impl_bounded_for_bounded! { BitOr::bitor }

impl_ops_reflective! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :BitOr::bitor
}

impl_assign! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :BitOrAssign::bitor_assign
}

impl_bounded_for_bounded! { BitXor::bitxor }

impl_ops_reflective! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :BitXor::bitxor
}

impl_assign! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :BitXorAssign::bitxor_assign
}

impl_bounded_for_bounded! { Shl::shl }

impl_ops_reflective! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :Shl::shl
}

impl_assign! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :ShlAssign::shl_assign
}

impl_bounded_for_bounded! { Shr::shr }

impl_ops_reflective! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :Shr::shr
}

impl_assign! {
    { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
    :ShrAssign::shr_assign
}

impl<T, B> Not for Bounded<T, B>
where
    B: AsBound<T>,
    T: Not,
{
    type Output = <T as Not>::Output;

    #[inline]
    fn not(self) -> Self::Output {
        self.value().not()
    }
}
