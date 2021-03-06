use crate::{
    expr::{AsBound, Contains},
    value::ToValue,
    Bounded,
};
use core::{hash::Hash, marker::PhantomData};
use typenum::{U0, Z0};

pub trait DefaultValueType: core::marker::Sized {
    type Output: ToValue<Self>;

    #[inline]
    fn default() -> Self {
        Self::Output::VALUE
    }
}

impl DefaultValueType for i8 {
    type Output = Z0;
}

impl DefaultValueType for i16 {
    type Output = Z0;
}

impl DefaultValueType for i32 {
    type Output = Z0;
}

impl DefaultValueType for i64 {
    type Output = Z0;
}

impl DefaultValueType for isize {
    type Output = Z0;
}

impl DefaultValueType for u8 {
    type Output = U0;
}

impl DefaultValueType for u16 {
    type Output = U0;
}

impl DefaultValueType for u32 {
    type Output = U0;
}

impl DefaultValueType for u64 {
    type Output = U0;
}

impl DefaultValueType for usize {
    type Output = U0;
}

/// If the bound contains zero, the `Bounded` implements `Default` trait.
///
/// If `B: AsBound` contains `T`'s default value, `Bounded<T, B>` implements `Default` trait.
impl<T, B> Default for Bounded<T, B>
where
    T: DefaultValueType,
    B: AsBound<T> + Contains<<T as DefaultValueType>::Output, Output = typenum::True>,
{
    #[inline]
    fn default() -> Self {
        Bounded {
            value: T::default(),
            bound: PhantomData,
        }
    }
}

impl<T, B> Hash for Bounded<T, B>
where
    B: AsBound<T>,
    T: Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}
