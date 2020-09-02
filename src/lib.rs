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

mod bounded_impls;
pub mod expr;
pub mod value;

pub use typenum;

use expr::{AsBound, Contains};
use shrinkwraprs::Shrinkwrap;
use std::marker::PhantomData;
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
