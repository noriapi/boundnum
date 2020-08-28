use super::{TypeExpr, ValueExpr};
use crate::typenum::*;
use crate::value::ToValue;

macro_rules! impl_op_for_typenum {
    ( $name:ident $(< $($p:ident),+ >)? $(
        where
            $($pb:ident: $first_bound:ident $(+ $rest_bound:ident)* ),+
    )?) => {
        impl<A $(, $($p),+)? > ValueExpr<A> for $name $(< $($p),+ >)?
            where
                Self: ToValue<A>, $(
                $($pb: $first_bound $(+ $rest_bound)* ),+
        )? {
            type Output = A;
            fn call(_: A) -> Self::Output {
                Self::value()
            }
        }

        impl<A $(, $($p),+)? > TypeExpr<A> for $name $(< $($p),+ >)? $(
            where
                $($pb: $first_bound $(+ $rest_bound)* ),+
        )? {
            type Output = Self;
        }
    }

}

impl_op_for_typenum! { PInt<U> where U: NonZero + Unsigned }
impl_op_for_typenum! { NInt<U> where U: NonZero + Unsigned }
impl_op_for_typenum! { Z0 }
impl_op_for_typenum! { B0 }
impl_op_for_typenum! { B1 }
impl_op_for_typenum! { UTerm }
impl_op_for_typenum! { UInt<U, B> where U: Unsigned, B: Bit }
