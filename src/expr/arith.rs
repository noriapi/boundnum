use super::{Call, TypeExpr, ValType, ValueExpr};

define_binary_operator! {
    add,
    /// Represents `core::ops::Add`
    Add,
    core::ops::Add,
    core::ops::Add::add,
    core::ops::Add,
}

define_binary_operator! {
    sub,
    /// Represents `core::ops::Sub`
    Sub,
    core::ops::Sub,
    core::ops::Sub::sub,
    core::ops::Sub,
}

define_binary_operator! {
    mul,
    /// Represents `core::ops::Mul`
    Mul,
    core::ops::Mul,
    core::ops::Mul::mul,
    core::ops::Mul,
}

define_binary_operator! {
    div,
    /// Represents `core::ops::Div`
    Div,
    core::ops::Div,
    core::ops::Div::div,
    core::ops::Div,
}

define_binary_operator! {
    rem,
    /// Represents `core::ops::Rem`
    Rem,
    core::ops::Rem,
    core::ops::Rem::rem,
    core::ops::Rem,
}

define_unary_operator! {
    neg,
    /// Represents `core::ops::Neg`
    Neg,
    core::ops::Neg,
    core::ops::Neg::neg,
    core::ops::Neg,
}
