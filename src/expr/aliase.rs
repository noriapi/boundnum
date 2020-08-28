use super::*;

/// (Start..End)
pub type Range<Start, End> = BitAnd<Ge<Arg, Start>, Lt<Arg, End>>;

/// (Start..=End)
pub type RangeInclusive<Start, End> = BitAnd<Ge<Arg, Start>, Le<Arg, End>>;
