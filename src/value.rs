//! Type to value conversions.

use std::ops::Add;
use typenum::*;

pub trait ToValue<I> {
    fn value() -> I;
}

pub trait TypeBound {
    type Min;
    type Max;
}

// 符号はそのままに絶対値を 1 だけ大きくする
pub trait Outer {
    type Output;
}

type OuterOf<T> = <T as Outer>::Output;

impl<U: Unsigned + NonZero> Outer for PInt<U>
where
    PInt<U>: Add<P1>,
{
    type Output = <PInt<U> as Add<P1>>::Output;
}

impl<U: Unsigned + NonZero> Outer for NInt<U>
where
    NInt<U>: Add<N1>,
{
    type Output = <NInt<U> as Add<N1>>::Output;
}

impl<U: Unsigned, B: Bit> Outer for UInt<U, B>
where
    UInt<U, B>: Add<B1>,
{
    type Output = Add1<UInt<U, B>>;
}

impl TypeBound for i8 {
    type Min = NInt<Exp<U2, U7>>;
    type Max = PInt<Sub1<Exp<U2, U7>>>;
}

impl TypeBound for i16 {
    type Min = NInt<Exp<U2, U15>>;
    type Max = PInt<Sub1<Exp<U2, U15>>>;
}

impl TypeBound for i32 {
    type Min = NInt<Exp<U2, U31>>;
    type Max = PInt<Sub1<Exp<U2, U31>>>;
}

impl TypeBound for i64 {
    type Min = NInt<Exp<U2, U63>>;
    type Max = PInt<Sub1<Exp<U2, U63>>>;
}

#[cfg(target_pointer_width = "16")]
impl TypeBound for isize {
    type Min = <i16 as TypeBound>::Min;
    type Max = <i16 as TypeBound>::Max;
}

#[cfg(target_pointer_width = "32")]
impl TypeBound for isize {
    type Min = <i32 as TypeBound>::Min;
    type Max = <i32 as TypeBound>::Max;
}

#[cfg(target_pointer_width = "64")]
impl TypeBound for isize {
    type Min = <i64 as TypeBound>::Min;
    type Max = <i64 as TypeBound>::Max;
}

macro_rules! impl_tovalue_for_integer {
    ( $ToType:ty, $Const:tt ) => {
        impl ToValue<$ToType> for Z0 {
            #[inline]
            fn value() -> $ToType {
                Self::$Const
            }
        }

        impl<U> ToValue<$ToType> for NInt<U>
        where
            U: Unsigned + NonZero,
            NInt<U>: Cmp<OuterOf<<$ToType as TypeBound>::Min>, Output = Greater>
                + Cmp<OuterOf<<$ToType as TypeBound>::Max>, Output = Less>,
        {
            #[inline]
            fn value() -> $ToType {
                Self::$Const
            }
        }

        impl<U> ToValue<$ToType> for PInt<U>
        where
            U: Unsigned + NonZero,
            PInt<U>: Cmp<OuterOf<<$ToType as TypeBound>::Min>, Output = Greater>
                + Cmp<OuterOf<<$ToType as TypeBound>::Max>, Output = Less>,
        {
            #[inline]
            fn value() -> $ToType {
                Self::$Const
            }
        }
    };
}

impl_tovalue_for_integer!(i8, I8);
impl_tovalue_for_integer!(i16, I16);
impl_tovalue_for_integer!(i32, I32);
impl_tovalue_for_integer!(i64, I64);
impl_tovalue_for_integer!(isize, ISIZE);

impl TypeBound for u8 {
    type Min = U0;
    type Max = Sub1<Exp<U2, U8>>;
}

impl TypeBound for u16 {
    type Min = U0;
    type Max = Sub1<Exp<U2, U16>>;
}

impl TypeBound for u32 {
    type Min = U0;
    type Max = Sub1<Exp<U2, U32>>;
}

impl TypeBound for u64 {
    type Min = U0;
    type Max = Sub1<Exp<U2, U64>>;
}

#[cfg(target_pointer_width = "16")]
impl TypeBound for usize {
    type Min = U0;
    type Max = <u16 as TypeBound>::Max;
}

#[cfg(target_pointer_width = "32")]
impl TypeBound for usize {
    type Min = U0;
    type Max = <u32 as TypeBound>::Max;
}

#[cfg(target_pointer_width = "64")]
impl TypeBound for usize {
    type Min = U0;
    type Max = <u64 as TypeBound>::Max;
}

macro_rules! impl_tovalue_for_unsigned {
    ( $ToType:ty, $Const:tt ) => {
        impl ToValue<$ToType> for UTerm {
            #[inline]
            fn value() -> $ToType {
                Self::$Const
            }
        }

        impl<U, B> ToValue<$ToType> for UInt<U, B>
        where
            U: Unsigned,
            B: Bit,
            UInt<U, B>: Cmp<OuterOf<<$ToType as TypeBound>::Max>, Output = Less>,
        {
            #[inline]
            fn value() -> $ToType {
                Self::$Const
            }
        }
    };
}

impl_tovalue_for_unsigned!(u8, U8);
impl_tovalue_for_unsigned!(u16, U16);
impl_tovalue_for_unsigned!(u32, U32);
impl_tovalue_for_unsigned!(u64, U64);
impl_tovalue_for_unsigned!(usize, USIZE);

#[cfg(test)]
mod tests {
    macro_rules! min_bound_tests {
    ( $pt:ty ) => {
		    #[test]
            fn inner_min() {
                assert_eq!(<$pt>::MIN, <$pt as TypeBound>::Min::value());
            }

		    #[test]
            fn outer_min() {
                use impls::impls;
                assert!(impls!(OuterOf<<$pt as TypeBound>::Min>: !ToValue<$pt>));
            }
	    };
    }

    macro_rules! max_bound_tests {
	    ( $pt:ty ) => {
		    #[test]
            fn inner_max() {
                assert_eq!(<$pt>::MAX, <$pt as TypeBound>::Max::value());
            }

		    #[test]
            fn outer_max() {
                use impls::impls;
                assert!(impls!(OuterOf<<$pt as TypeBound>::Max>: !ToValue<$pt>));
            }
	    };
    }

    macro_rules! zero_test {
        ( $pt:ty ) => {
            #[test]
            fn zero() {
                assert_eq!((0 as $pt), Z0::value());
            }
        };
    }

    mod i8 {
        use super::super::*;
        min_bound_tests!(i8);
        max_bound_tests!(i8);
        zero_test!(i8);
    }

    mod i16 {
        use super::super::*;
        min_bound_tests!(i16);
        max_bound_tests!(i16);
        zero_test!(i16);
    }

    mod i32 {
        use super::super::*;
        min_bound_tests!(i32);
        max_bound_tests!(i32);
        zero_test!(i32);
    }

    mod i64 {
        use super::super::*;
        min_bound_tests!(i64);
        max_bound_tests!(i64);
        zero_test!(i64);
    }

    mod isize {
        use super::super::*;
        min_bound_tests!(isize);
        max_bound_tests!(isize);
        zero_test!(isize);
    }

    mod u8 {
        use super::super::*;
        max_bound_tests!(u8);
    }

    mod u16 {
        use super::super::*;
        max_bound_tests!(u16);
    }

    mod u32 {
        use super::super::*;
        max_bound_tests!(u32);
    }

    mod u64 {
        use super::super::*;
        max_bound_tests!(u64);
    }

    mod usize {
        use super::super::*;
        max_bound_tests!(usize);
    }
}
