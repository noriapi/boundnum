use super::*;
use crate::typenum::consts::True;
use crate::value::ToValue;

/// (Start..End)
pub type Range<Start, End> = BitAnd<Ge<Arg, Start>, Lt<Arg, End>>;

impl<T, Start, End> ToValue<core::ops::Range<T>> for Range<Start, End>
where
    Start: ToValue<T>,
    End: ToValue<T>,
{
    const VALUE: core::ops::Range<T> = core::ops::Range {
        start: Start::VALUE,
        end: End::VALUE,
    };
}

/// (Start..)
pub type RangeFrom<Start> = Ge<Arg, Start>;

impl<T, Start> ToValue<core::ops::RangeFrom<T>> for RangeFrom<Start>
where
    Start: ToValue<T>,
{
    const VALUE: core::ops::RangeFrom<T> = core::ops::RangeFrom {
        start: Start::VALUE,
    };
}

/// (..)
pub type RangeFull = True;

impl ToValue<core::ops::RangeFull> for RangeFull {
    const VALUE: core::ops::RangeFull = core::ops::RangeFull;
}

/// (Start..=End)
pub type RangeInclusive<Start, End> = BitAnd<Ge<Arg, Start>, Le<Arg, End>>;

impl<T, Start, End> ToValue<core::ops::RangeInclusive<T>> for RangeInclusive<Start, End>
where
    Start: ToValue<T>,
    End: ToValue<T>,
{
    const VALUE: core::ops::RangeInclusive<T> =
        core::ops::RangeInclusive::new(Start::VALUE, End::VALUE);
}

/// (..End)
pub type RangeTo<End> = Lt<Arg, End>;

impl<T, End> ToValue<core::ops::RangeTo<T>> for RangeTo<End>
where
    End: ToValue<T>,
{
    const VALUE: core::ops::RangeTo<T> = core::ops::RangeTo { end: End::VALUE };
}

/// (..=End)
pub type RangeToInclusive<End> = Le<Arg, End>;

impl<T, End> ToValue<core::ops::RangeToInclusive<T>> for RangeToInclusive<End>
where
    End: ToValue<T>,
{
    const VALUE: core::ops::RangeToInclusive<T> = core::ops::RangeToInclusive { end: End::VALUE };
}
