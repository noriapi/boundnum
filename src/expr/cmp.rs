use super::{Call, TypeExpr, ValType, ValueExpr};

mod helper_traits {
    pub trait OrdExt<R>: core::cmp::Ord {
        type Output;
        fn cmp(self, rhs: R) -> Self::Output;
    }

    impl<L: core::cmp::Ord> OrdExt<L> for L {
        type Output = core::cmp::Ordering;
        fn cmp(self, rhs: L) -> Self::Output {
            core::cmp::Ord::cmp(&self, &rhs)
        }
    }

    pub trait PartialEqExt<R>: PartialEq {
        type Output;
        fn eq(self, rhs: R) -> Self::Output;
        fn ne(self, rhs: R) -> Self::Output;
    }

    impl<L: PartialEq> PartialEqExt<L> for L {
        type Output = bool;
        fn eq(self, rhs: L) -> Self::Output {
            PartialEq::eq(&self, &rhs)
        }

        fn ne(self, rhs: L) -> Self::Output {
            PartialEq::ne(&self, &rhs)
        }
    }

    pub trait PartialOrdExt<R>: PartialOrd {
        type Output;
        fn gt(self, rhs: R) -> Self::Output;
        fn ge(self, rhs: R) -> Self::Output;
        fn lt(self, rhs: R) -> Self::Output;
        fn le(self, rhs: R) -> Self::Output;
    }

    impl<L: PartialOrd> PartialOrdExt<L> for L {
        type Output = bool;
        fn gt(self, rhs: L) -> Self::Output {
            PartialOrd::gt(&self, &rhs)
        }

        fn ge(self, rhs: L) -> Self::Output {
            PartialOrd::ge(&self, &rhs)
        }

        fn lt(self, rhs: L) -> Self::Output {
            PartialOrd::lt(&self, &rhs)
        }

        fn le(self, rhs: L) -> Self::Output {
            PartialOrd::le(&self, &rhs)
        }
    }

    pub trait MinMax<R>: core::cmp::Ord {
        type Output;
        fn max(self, rhs: R) -> Self::Output;
        fn min(self, rhs: R) -> Self::Output;
    }

    impl<L: core::cmp::Ord> MinMax<L> for L {
        type Output = L;
        fn max(self, rhs: L) -> Self::Output {
            core::cmp::Ord::max(self, rhs)
        }

        fn min(self, rhs: L) -> Self::Output {
            core::cmp::Ord::min(self, rhs)
        }
    }
}

use helper_traits::*;

define_binary_operator! {
    cmp,
    /// Represents `core::cmp::Ord::cmp`
    Cmp,
    OrdExt,
    OrdExt::cmp,
    typenum::Cmp,
}

define_binary_operator! {
    eq,
    /// Represents `core::cmp::PartialEq::eq`
    Eq,
    PartialEqExt,
    PartialEqExt::eq,
    typenum::IsEqual,
}

define_binary_operator! {
    ne,
    /// Represents `core::cmp::PartialEq::ne`
    Ne,
    PartialEqExt,
    PartialEqExt::ne,
    typenum::IsNotEqual,
}

define_binary_operator! {
    gt,
    /// Represents `core::cmp::PartialOrd::gt`
    Gt,
    PartialOrdExt,
    PartialOrdExt::gt,
    typenum::IsGreater,
}

define_binary_operator! {
    ge,
    /// Represents `core::cmp::PartialOrd::ge`
    Ge,
    PartialOrdExt,
    PartialOrdExt::ge,
    typenum::IsGreaterOrEqual,
}

define_binary_operator! {
    lt,
    /// Represents `core::cmp::PartialOrd::lt`
    Lt,
    PartialOrdExt,
    PartialOrdExt::lt,
    typenum::IsLess,
}

define_binary_operator! {
    le,
    /// Represents `core::cmp::PartialOrd::le`
    Le,
    PartialOrdExt,
    PartialOrdExt::le,
    typenum::IsLessOrEqual,
}

define_binary_operator! {
    max,
    /// Represents `core::cmp::Ord::max`
    Max,
    MinMax,
    MinMax::max,
    typenum::Max,
}

define_binary_operator! {
    min,
    /// Represents `core::cmp::Ord::min`
    Min,
    MinMax,
    MinMax::min,
    typenum::Min,
}
