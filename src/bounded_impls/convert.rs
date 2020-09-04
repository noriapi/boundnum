//! refers [core::convert](https://doc.rust-lang.org/src/core/convert/num.rs.html)
use crate::{AsBound, Bounded};
use core::convert::{TryFrom, TryInto};

macro_rules! impl_from_bounded_for_internal_value {
    ($Internal: ty) => {
        /// Simply returns the internal value.
        ///
        /// Because of Rust's orphan rules for implementing traits,
        /// it is not possible to implement them using generics.
        impl<B> From<Bounded<$Internal, B>> for $Internal
        where
            B: AsBound<$Internal>,
        {
            #[inline]
            fn from(bounded: Bounded<$Internal, B>) -> Self {
                bounded.value()
            }
        }
    };
}

impl_from_bounded_for_internal_value! { i8 }
impl_from_bounded_for_internal_value! { i16 }
impl_from_bounded_for_internal_value! { i32 }
impl_from_bounded_for_internal_value! { i64 }
impl_from_bounded_for_internal_value! { isize }
impl_from_bounded_for_internal_value! { u8 }
impl_from_bounded_for_internal_value! { u16 }
impl_from_bounded_for_internal_value! { u32 }
impl_from_bounded_for_internal_value! { u64 }
impl_from_bounded_for_internal_value! { usize }

macro_rules! impl_from_bounded_for_bounded {
    ($Small: ty, $Large: ty, $doc: expr) => {
        #[doc = $doc]
        impl<B> From<Bounded<$Small, B>> for Bounded<$Large, B>
        where
            B: AsBound<$Small> + AsBound<$Large>,
        {
            #[inline]
            fn from(small: Bounded<$Small, B>) -> Self {
                Bounded {
                    value: small.value() as $Large,
                    bound: Default::default(),
                }
            }
        }
    };
    ($Small: ty, $Large: ty) => {
        impl_from_bounded_for_bounded!($Small,
                   $Large,
                   concat!("Converts `Bounded<",
                           stringify!($Small),
                           ", B>` to `Bounded<",
                           stringify!($Large),
                           ", B>` losslessly."));
    }

}

macro_rules! impl_from_bounded_for_primitive {
    ($Small: ty, $Large: ty, $doc: expr) => {
        #[doc = $doc]
        impl<B> From<Bounded<$Small, B>> for $Large
        where
            B: AsBound<$Small>,
        {
            #[inline]
            fn from(small: Bounded<$Small, B>) -> Self {
                small.value() as Self
            }
        }
    };
    ($Small: ty, $Large: ty) => {
        impl_from_bounded_for_primitive!($Small,
                   $Large,
                   concat!("Converts `Bounded<",
                           stringify!($Small),
                           ", B>` to `",
                           stringify!($Large),
                           "` losslessly."));
    }
}

macro_rules! impl_from_bounded {
    ($Small: ty, $Large: ty) => {
        impl_from_bounded_for_bounded! { $Small, $Large }
        impl_from_bounded_for_primitive! { $Small, $Large }
    };
}

// Unsigned -> Unsigned
impl_from_bounded! { u8, u16 }
impl_from_bounded! { u8, u32 }
impl_from_bounded! { u8, u64 }
impl_from_bounded! { u8, u128 }
impl_from_bounded! { u8, usize }
impl_from_bounded! { u16, u32 }
impl_from_bounded! { u16, u64 }
impl_from_bounded! { u16, u128 }
impl_from_bounded! { u32, u64 }
impl_from_bounded! { u32, u128 }
impl_from_bounded! { u64, u128 }

// Signed -> Signed
impl_from_bounded! { i8, i16 }
impl_from_bounded! { i8, i32 }
impl_from_bounded! { i8, i64 }
impl_from_bounded! { i8, i128 }
impl_from_bounded! { i8, isize }
impl_from_bounded! { i16, i32 }
impl_from_bounded! { i16, i64 }
impl_from_bounded! { i16, i128 }
impl_from_bounded! { i32, i64 }
impl_from_bounded! { i32, i128 }
impl_from_bounded! { i64, i128 }

// Unsigned -> Signed
impl_from_bounded! { u8, i16 }
impl_from_bounded! { u8, i32 }
impl_from_bounded! { u8, i64 }
impl_from_bounded! { u8, i128 }
impl_from_bounded! { u16, i32 }
impl_from_bounded! { u16, i64 }
impl_from_bounded! { u16, i128 }
impl_from_bounded! { u32, i64 }
impl_from_bounded! { u32, i128 }
impl_from_bounded! { u64, i128 }

// The C99 standard defines bounds on INTPTR_MIN, INTPTR_MAX, and UINTPTR_MAX
// which imply that pointer-sized integers must be at least 16 bits:
// https://port70.net/~nsz/c/c99/n1256.html#7.18.2.4
impl_from_bounded! { u16, usize }
impl_from_bounded! { u8, isize }
impl_from_bounded! { i16, isize }

// Signed -> Float
impl_from_bounded! { i8, f32 }
impl_from_bounded! { i8, f64 }
impl_from_bounded! { i16, f32 }
impl_from_bounded! { i16, f64 }
impl_from_bounded! { i32, f64 }

// Unsigned -> Float
impl_from_bounded! { u8, f32 }
impl_from_bounded! { u8, f64 }
impl_from_bounded! { u16, f32 }
impl_from_bounded! { u16, f64 }
impl_from_bounded! { u32, f64 }

macro_rules! impl_try_from {
    (@inner $From:ty, $To:ty) => {
        impl<B> TryFrom<Bounded<$From, B>> for Bounded<$To, B>
        where
            B: AsBound<$From> + AsBound<$To>,
        {
            type Error = <$To as TryFrom<$From>>::Error;
            fn try_from(value: Bounded<$From, B>) -> Result<Self, Self::Error> {
                Ok(Bounded {
                    value: (*value).try_into()?,
                    bound: Default::default(),
                })
            }
        }
    };
    ($From:ty, $($To:ty),+) => {
        $(
            impl_try_from! { @inner $From, $To }
        )+
    }
}

macro_rules! rev {
    ($mac:ident, $source:ty, $($target:ty),*) => {$(
        $mac!($target, $source);
    )*}
}

// intra-sign conversions
impl_try_from!(u16, u8);
impl_try_from!(u32, u16, u8);
impl_try_from!(u64, u32, u16, u8);
impl_try_from!(u128, u64, u32, u16, u8);

impl_try_from!(i16, i8);
impl_try_from!(i32, i16, i8);
impl_try_from!(i64, i32, i16, i8);
impl_try_from!(i128, i64, i32, i16, i8);

// unsigned-to-signed
impl_try_from!(u8, i8);
impl_try_from!(u16, i8, i16);
impl_try_from!(u32, i8, i16, i32);
impl_try_from!(u64, i8, i16, i32, i64);
impl_try_from!(u128, i8, i16, i32, i64, i128);

// signed-to-unsigned
impl_try_from!(i8, u8, u16, u32, u64, u128);
impl_try_from!(i16, u16, u32, u64, u128);
impl_try_from!(i32, u32, u64, u128);
impl_try_from!(i64, u64, u128);
impl_try_from!(i128, u128);
impl_try_from!(i16, u8);
impl_try_from!(i32, u16, u8);
impl_try_from!(i64, u32, u16, u8);
impl_try_from!(i128, u64, u32, u16, u8);

// usize/isize
impl_try_from!(usize, isize);
impl_try_from!(isize, usize);

#[cfg(target_pointer_width = "16")]
mod ptr_try_from_impls {
    use super::*;

    impl_try_from!(usize, u8);
    impl_try_from!(usize, u16, u32, u64, u128);
    impl_try_from!(usize, i8, i16);
    impl_try_from!(usize, i32, i64, i128);

    impl_try_from!(isize, u8);
    impl_try_from!(isize, u16, u32, u64, u128);
    impl_try_from!(isize, i8);
    impl_try_from!(isize, i16, i32, i64, i128);

    rev!(impl_try_from, usize, u32, u64, u128);
    rev!(impl_try_from, usize, i8, i16);
    rev!(impl_try_from, usize, i32, i64, i128);

    rev!(impl_try_from, isize, u16, u32, u64, u128);
    rev!(impl_try_from, isize, i32, i64, i128);
}

#[cfg(target_pointer_width = "32")]
mod ptr_try_from_impls {
    use super::*;

    impl_try_from!(usize, u8, u16);
    impl_try_from!(usize, u32, u64, u128);
    impl_try_from!(usize, i8, i16, i32);
    impl_try_from!(usize, i64, i128);

    impl_try_from!(isize, u8, u16);
    impl_try_from!(isize, u32, u64, u128);
    impl_try_from!(isize, i8, i16);
    impl_try_from!(isize, i32, i64, i128);

    rev!(impl_try_from, usize, u32);
    rev!(impl_try_from, usize, u64, u128);
    rev!(impl_try_from, usize, i8, i16, i32);
    rev!(impl_try_from, usize, i64, i128);

    rev!(impl_try_from, isize, u16);
    rev!(impl_try_from, isize, u32, u64, u128);
    rev!(impl_try_from, isize, i32);
    rev!(impl_try_from, isize, i64, i128);
}

#[cfg(target_pointer_width = "64")]
mod ptr_try_from_impls {
    use super::*;

    impl_try_from!(usize, u8, u16, u32);
    impl_try_from!(usize, u64, u128);
    impl_try_from!(usize, i8, i16, i32, i64);
    impl_try_from!(usize, i128);

    impl_try_from!(isize, u8, u16, u32);
    impl_try_from!(isize, u64, u128);
    impl_try_from!(isize, i8, i16, i32);
    impl_try_from!(isize, i64, i128);

    rev!(impl_try_from, usize, u32, u64);
    rev!(impl_try_from, usize, u128);
    rev!(impl_try_from, usize, i8, i16, i32, i64);
    rev!(impl_try_from, usize, i128);

    rev!(impl_try_from, isize, u16, u32);
    rev!(impl_try_from, isize, u64, u128);
    rev!(impl_try_from, isize, i32, i64);
    rev!(impl_try_from, isize, i128);
}
