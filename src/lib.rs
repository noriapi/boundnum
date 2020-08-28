//! This crate provides a type boundary and a numeric type bounded by it.
//!
//! # Example
//! ```
//! use boundnum::{expr::*, typenum::consts::*, Bounded, Boundable};
//!
//! fn main() {
//!     let less4 = Bounded::<u8, Le<Arg, U4>>::new::<U1>();
//!
//!     let mul_of_two: Bounded<u8, Eq<U0, Rem<Arg, U2>>> =
//!         (*less4 + 3).bound().unwrap_or(Bounded::new::<U0>());
//! }
//! ```

mod default;
pub mod expr;
pub mod value;

pub use typenum;

use default::DefaultValueType;
use expr::{AsBound, Contains};
use shrinkwraprs::Shrinkwrap;
use value::ToValue;

/// A wrapper struct representing bounded numeric type.
#[derive(Shrinkwrap, Copy, Clone, Debug)]
pub struct Bounded<T, B: AsBound<T>> {
    #[shrinkwrap(main_field)]
    value: T,
    bound: B,
}

impl<T, B: AsBound<T> + Default> Bounded<T, B> {
    pub fn new<A>() -> Self
    where
        A: ToValue<T>,
        B: Contains<A, Output = typenum::True>,
    {
        Bounded {
            value: A::value(),
            bound: Default::default(),
        }
    }
}

/// If the bound contains zero, the `Bounded` implements `Default` trait.
///
/// If `B: AsBound` contains `T`'s default value, `Bounded<T, B>` implements `Default` trait.
impl<T, B> Default for Bounded<T, B>
where
    T: DefaultValueType,
    B: AsBound<T> + Default + Contains<<T as DefaultValueType>::Output, Output = typenum::True>,
{
    fn default() -> Self {
        Bounded {
            value: T::default(),
            bound: Default::default(),
        }
    }
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
    B: AsBound<T> + std::default::Default,
{
    type Raw = T;
    type Bound = B;

    /// Try to bound a value.
    fn bound(self) -> Option<Bounded<Self::Raw, Self::Bound>> {
        if <Self::Bound as AsBound<T>>::contains(self) {
            Some(Bounded {
                value: self,
                bound: Default::default(),
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
    fn bounded_is_copyable() {
        use expr::*;
        use impls::impls;
        use typenum::consts::*;
        assert!(impls!(Bounded<u8, Range<U3, U4>>: Copy));
    }

    #[test]
    fn impl_default() {
        use expr::*;
        use impls::impls;
        use typenum::consts::*;
        assert!(impls!(Bounded<i8, Range<N3, P4>>: Default))
    }

    #[test]
    fn not_impl_default() {
        use expr::*;
        use impls::impls;
        use typenum::consts::*;
        assert!(impls!(Bounded<u8, Range<U3, U4>>: !Default))
    }
}
