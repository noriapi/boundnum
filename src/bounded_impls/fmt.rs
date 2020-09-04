//! refers [rust/num.rs](https://github.com/rust-lang/rust/blob/master/library/core/src/fmt/num.rs)
use crate::{expr::AsBound, Bounded};
use core::fmt;

macro_rules! impl_trait {
    ($Trait:path) => {
        impl<T, B> $Trait for Bounded<T, B>
        where
            T: $Trait,
            B: AsBound<T>,
        {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.as_ref().fmt(f)
            }
        }
    };
}

impl_trait! { fmt::Display }
impl_trait! { fmt::Binary }
impl_trait! { fmt::Octal }
impl_trait! { fmt::LowerExp }
impl_trait! { fmt::LowerHex }
impl_trait! { fmt::UpperExp }
impl_trait! { fmt::UpperHex }
