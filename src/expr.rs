//! Expression structs.

#[macro_use]
mod macros;
mod arith;
mod bit;
mod cmp;
mod other;
mod range;
mod typenum;

pub use arith::*;
pub use bit::*;
pub use cmp::*;
pub use other::*;
pub use range::*;

/// Represents a expression for values.
pub trait ValueExpr<A> {
    type Output;
    fn call(arg: A) -> Self::Output;
}
/// Get a `ValueExpr`'s output type.
pub type ValType<O, A> = <O as ValueExpr<A>>::Output;

/// Represents a expression for types.
pub trait TypeExpr<A> {
    type Output;
}
/// Call `TypeExpr` then returning the `Output`.
pub type Call<O, A> = <O as TypeExpr<A>>::Output;

/// Represents an argument.
#[derive(Debug, Default, Clone, Copy)]
pub struct Arg;
impl<A> ValueExpr<A> for Arg {
    type Output = A;
    fn call(arg: A) -> Self::Output {
        arg
    }
}
impl<A> TypeExpr<A> for Arg {
    type Output = A;
}

/// Can use as bound.
pub trait AsBound<T>: ValueExpr<T, Output = bool> {
    fn contains(value: T) -> bool {
        Self::call(value)
    }
}
impl<O, T> AsBound<T> for O where O: ValueExpr<T, Output = bool> {}

/// Call a type expression as bound.
pub trait Contains<A> {
    type Output: ::typenum::Bit;
}

impl<A, T> Contains<A> for T
where
    T: TypeExpr<A>,
    <T as TypeExpr<A>>::Output: ::typenum::Bit,
{
    type Output = T::Output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_is_not_bound() {
        use impls::impls;
        assert!(impls!(Add<Arg, Arg>: !AsBound<i8>));
        assert!(impls!(Add<Arg, Arg>: !AsBound<i16>));
        assert!(impls!(Add<Arg, Arg>: !AsBound<i32>));
        assert!(impls!(Add<Arg, Arg>: !AsBound<i64>));
        assert!(impls!(Add<Arg, Arg>: !AsBound<isize>));
        assert!(impls!(Add<Arg, Arg>: !AsBound<u8>));
        assert!(impls!(Add<Arg, Arg>: !AsBound<u16>));
        assert!(impls!(Add<Arg, Arg>: !AsBound<u32>));
        assert!(impls!(Add<Arg, Arg>: !AsBound<u64>));
        assert!(impls!(Add<Arg, Arg>: !AsBound<usize>));
    }

    #[test]
    fn add_valop() {
        assert_eq!(3 + 2, Add::<Arg, ::typenum::U2>::call(3u32));
    }
}
