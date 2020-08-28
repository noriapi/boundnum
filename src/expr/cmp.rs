use super::{Call, TypeExpr, ValType, ValueExpr};

mod helper_traits {
    pub trait OrdExt<R>: std::cmp::Ord {
        type Output;
        fn cmp(self, rhs: R) -> Self::Output;
    }

    impl<L: std::cmp::Ord> OrdExt<L> for L {
        type Output = std::cmp::Ordering;
        fn cmp(self, rhs: L) -> Self::Output {
            std::cmp::Ord::cmp(&self, &rhs)
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

    pub trait MinMax<R>: std::cmp::Ord {
        type Output;
        fn max(self, rhs: R) -> Self::Output;
        fn min(self, rhs: R) -> Self::Output;
    }

    impl<L: std::cmp::Ord> MinMax<L> for L {
        type Output = L;
        fn max(self, rhs: L) -> Self::Output {
            std::cmp::Ord::max(self, rhs)
        }

        fn min(self, rhs: L) -> Self::Output {
            std::cmp::Ord::min(self, rhs)
        }
    }
}

use helper_traits::*;

define_binary_operator! {
    cmp,
    /// Represents `std::cmp::Ord::cmp`
    Cmp,
    OrdExt,
    OrdExt::cmp,
    typenum::Cmp,
}

define_binary_operator! {
    eq,
    /// Represents `std::cmp::PartialEq::eq`
    Eq,
    PartialEqExt,
    PartialEqExt::eq,
    typenum::IsEqual,
}

define_binary_operator! {
    ne,
    /// Represents `std::cmp::PartialEq::ne`
    Ne,
    PartialEqExt,
    PartialEqExt::ne,
    typenum::IsNotEqual,
}

define_binary_operator! {
    gt,
    /// Represents `std::cmp::PartialOrd::gt`
    Gt,
    PartialOrdExt,
    PartialOrdExt::gt,
    typenum::IsGreater,
}

define_binary_operator! {
    ge,
    /// Represents `std::cmp::PartialOrd::ge`
    Ge,
    PartialOrdExt,
    PartialOrdExt::ge,
    typenum::IsGreaterOrEqual,
}

define_binary_operator! {
    lt,
    /// Represents `std::cmp::PartialOrd::lt`
    Lt,
    PartialOrdExt,
    PartialOrdExt::lt,
    typenum::IsLess,
}

define_binary_operator! {
    le,
    /// Represents `std::cmp::PartialOrd::le`
    Le,
    PartialOrdExt,
    PartialOrdExt::le,
    typenum::IsLessOrEqual,
}

define_binary_operator! {
    max,
    /// Represents `std::cmp::Ord::max`
    Max,
    MinMax,
    MinMax::max,
    typenum::Max,
}

define_binary_operator! {
    min,
    /// Represents `std::cmp::Ord::min`
    Min,
    MinMax,
    MinMax::min,
    typenum::Min,
}
