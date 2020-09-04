//! https://github.com/rust-lang/rust/blob/36b0d7e25769e88fec85e1d073196065a7f2d7c4/library/core/src/cmp.rs#L1036

use crate::{expr::AsBound, Bounded};
use core::cmp::Ordering;

impl<T, B> Eq for Bounded<T, B>
where
    Self: PartialEq<Self>,
    B: AsBound<T>,
{
}

/// Bounded to Bounded PartialEq
impl<OT, OB, ST, SB> PartialEq<Bounded<OT, OB>> for Bounded<ST, SB>
where
    OB: AsBound<OT>,
    SB: AsBound<ST>,
    ST: PartialEq<OT>,
{
    #[inline]
    fn eq(&self, other: &Bounded<OT, OB>) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl<T, B> Ord for Bounded<T, B>
where
    T: Ord,
    B: AsBound<T>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

/// Bounded to Bounded PartialOrd
impl<OT, OB, ST, SB> PartialOrd<Bounded<OT, OB>> for Bounded<ST, SB>
where
    OB: AsBound<OT>,
    SB: AsBound<ST>,
    ST: PartialOrd<OT>,
{
    #[inline]
    fn partial_cmp(&self, other: &Bounded<OT, OB>) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

macro_rules! impl_trait_with_primitive {
    ($Primitive:ty, $Trait:ident, $func_name:ident, $Output:ty) => {
        impl<BO> $Trait<Bounded<$Primitive, BO>> for $Primitive
        where
            BO: AsBound<$Primitive>,
        {
            #[inline]
            fn $func_name(&self, other: &Bounded<$Primitive, BO>) -> $Output {
                self.$func_name(other.as_ref())
            }
        }

        impl<BS> $Trait<$Primitive> for Bounded<$Primitive, BS>
        where
            BS: AsBound<$Primitive>,
        {
            #[inline]
            fn $func_name(&self, other: &$Primitive) -> $Output {
                self.as_ref().$func_name(other)
            }
        }
    };
    ($($Primitive:ty,)+: $Trait:ident::$func_name:ident -> $Output:ty) => {
        $(impl_trait_with_primitive! { $Primitive, $Trait, $func_name, $Output })+
    }
}

impl_trait_with_primitive! {
    i8,
    i16,
    i32,
    i64,
    isize,
    u8,
    u16,
    u32,
    u64,
    usize,
    : PartialEq::eq -> bool
}
impl_trait_with_primitive! {
    i8,
    i16,
    i32,
    i64,
    isize,
    u8,
    u16,
    u32,
    u64,
    usize,
    : PartialOrd::partial_cmp -> Option<Ordering>
}
