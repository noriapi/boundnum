use crate::value::ToValue;
use typenum::{U0, Z0};

pub trait DefaultValueType: std::marker::Sized {
    type Output: ToValue<Self>;
    fn default() -> Self {
        Self::Output::value()
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
