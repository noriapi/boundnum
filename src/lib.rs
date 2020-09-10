//! This crate provides a type boundary and a numeric type bounded by it.
//!
//! # Example
//! ```
//! use boundnum::{expr::*, typenum::consts::*, Bounded, Boundable, CONST};
//!
//! fn main() {
//!     const less4: Bounded::<u8, Le<Arg, U4>> = CONST!(U3);
//!
//!     let mul_of_two: Bounded<u8, Eq<U0, Rem<Arg, U2>>> =
//!         (*less4 + 3).bound().unwrap_or(Bounded::new::<U0>());
//! }
//! ```

#![no_std]

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

mod bounded_impls;
pub mod expr;
pub mod value;

pub use typenum;

use core::{iter::FusedIterator, marker::PhantomData};
use expr::{AsBound, Contains, Range, RangeFrom, RangeInclusive};
use shrinkwraprs::Shrinkwrap;
use value::ToValue;

/// A wrapper struct representing bounded numeric type.
#[derive(Shrinkwrap, Copy, Clone, Debug)]
pub struct Bounded<T, B: AsBound<T>> {
    #[shrinkwrap(main_field)]
    value: T,
    bound: PhantomData<B>,
}

impl<T, B: AsBound<T>> Bounded<T, B> {
    pub fn new<A>() -> Self
    where
        A: ToValue<T>,
        B: Contains<A, Output = typenum::True>,
    {
        Bounded {
            value: A::VALUE,
            bound: PhantomData,
        }
    }

    pub fn value(self) -> T {
        self.value
    }
}

impl<T, Start, End> Bounded<T, Range<Start, End>>
where
    Range<Start, End>: AsBound<T> + ToValue<core::ops::Range<T>>,
    core::ops::Range<T>: Iterator<Item = T> + DoubleEndedIterator + FusedIterator,
{
    pub fn iter() -> impl Iterator<Item = Self> + DoubleEndedIterator + FusedIterator {
        Range::<Start, End>::VALUE.map(|value| Bounded {
            value,
            bound: PhantomData,
        })
    }
}

impl<T, Start, End> Bounded<T, RangeInclusive<Start, End>>
where
    RangeInclusive<Start, End>: AsBound<T> + ToValue<core::ops::RangeInclusive<T>>,
    core::ops::RangeInclusive<T>: Iterator<Item = T> + DoubleEndedIterator + FusedIterator,
{
    pub fn iter() -> impl Iterator<Item = Self> + DoubleEndedIterator + FusedIterator {
        RangeInclusive::<Start, End>::VALUE.map(|value| Bounded {
            value,
            bound: PhantomData,
        })
    }
}

impl<T, Start> Bounded<T, RangeFrom<Start>>
where
    RangeFrom<Start>: AsBound<T> + ToValue<core::ops::RangeFrom<T>>,
    core::ops::RangeFrom<T>: Iterator<Item = T> + FusedIterator,
{
    pub fn iter() -> impl Iterator<Item = Self> + FusedIterator {
        RangeFrom::<Start>::VALUE.map(|value| Bounded {
            value,
            bound: PhantomData,
        })
    }
}

/// An easier way to define a const `Bounded` value.
#[macro_export]
macro_rules! CONST {
    ($Arg:path) => {
        <boundnum::Bounded<_, _> as boundnum::Const<$Arg>>::BOUNDED
    };
}

/// Provides a const value for `Bounded`.
///
/// The type parameter `T` is the value on the `typenum` to be converted to the raw value of `Bounded`.
pub trait Const<T> {
    const BOUNDED: Self;
}

impl<A, T, B> Const<A> for Bounded<T, B>
where
    B: AsBound<T> + Contains<A, Output = typenum::True>,
    A: ToValue<T>,
{
    const BOUNDED: Self = Bounded {
        value: A::VALUE,
        bound: PhantomData,
    };
}

/// A trait of the type being converted to `Bounded`.
pub trait Boundable<B> {
    type Raw;
    type Bound: AsBound<Self::Raw>;
    fn bound(self) -> Option<Bounded<Self::Raw, Self::Bound>>;
}

impl<T, B> Boundable<B> for T
where
    T: Copy,
    B: AsBound<T>,
{
    type Raw = T;
    type Bound = B;

    /// Try to bound a value.
    fn bound(self) -> Option<Bounded<Self::Raw, Self::Bound>> {
        if <Self::Bound as AsBound<T>>::contains(self) {
            Some(Bounded {
                value: self,
                bound: PhantomData,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_iter() {
        use typenum::consts::*;
        assert!(Bounded::<i8, Range<N3, P3>>::iter().eq(-3..3));
    }

    #[test]
    fn range_inclusive_iter() {
        use typenum::consts::*;
        assert!(Bounded::<i8, RangeInclusive<N3, P3>>::iter().eq(-3..=3));
    }

    #[test]
    fn range_from_iter() {
        use typenum::consts::*;
        let mut bounded = Bounded::<i8, RangeFrom<N3>>::iter();
        let mut prim = -3..;

        // Check manually to avoid panic due to overflow.
        for _ in -3..127 {
            assert!(|| -> bool {
                let b = match bounded.next() {
                    None => return prim.next().is_none(),
                    Some(val) => val,
                };

                let p = match prim.next() {
                    None => return false,
                    Some(val) => val,
                };

                b == p
            }());
        }
    }
}
