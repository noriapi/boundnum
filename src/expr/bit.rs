use super::{Call, TypeExpr, ValType, ValueExpr};

define_unary_operator! {
    not,
    /// Represents `core::ops::Not`
    Not,
    core::ops::Not,
    core::ops::Not::not,
    core::ops::Not,
}

define_binary_operator! {
    bitand,
    /// Represents `core::ops::BitAnd`
    BitAnd,
    core::ops::BitAnd,
    core::ops::BitAnd::bitand,
    core::ops::BitAnd,
}

define_binary_operator! {
    bitor,
    /// Represents `core::ops::BitOr`
    BitOr,
    core::ops::BitOr,
    core::ops::BitOr::bitor,
    core::ops::BitOr,
}

define_binary_operator! {
    bitxor,
    /// Represents `core::ops::BitXor`
    BitXor,
    core::ops::BitXor,
    core::ops::BitXor::bitxor,
    core::ops::BitXor,
}

define_binary_operator! {
    shl,
    /// Represents `core::ops::Shl`
    Shl,
    core::ops::Shl,
    core::ops::Shl::shl,
    core::ops::Shl,
}

define_binary_operator! {
    shr,
    /// Represents `core::ops::Shr`
    Shr,
    core::ops::Shr,
    core::ops::Shr::shr,
    core::ops::Shr,
}
