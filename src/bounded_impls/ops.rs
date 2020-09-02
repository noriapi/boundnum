macro_rules! impl_bounded_for_bounded {
    ($Trait:ident::$func:ident) => {
        impl<TL, BL, TR, BR> $Trait<Bounded<TR, BR>> for Bounded<TL, BL>
        where
            BL: AsBound<TL>,
            BR: AsBound<TR>,
            TL: $Trait<TR>,
        {
            type Output = <TL as $Trait<TR>>::Output;

            #[inline]
            fn $func(self, rhs: Bounded<TR, BR>) -> Self::Output {
                self.value().$func(rhs.value())
            }
        }
    };
}

macro_rules! impl_ops_reflective {
    ($Type:ty, $Trait:ident, $func:ident) => {
        impl<TL, BL> $Trait<$Type> for Bounded<TL, BL>
        where
            BL: AsBound<TL>,
            TL: $Trait<$Type>,
        {
            type Output = <TL as $Trait<$Type>>::Output;

            #[inline]
            fn $func(self, rhs: $Type) -> Self::Output {
                self.value().$func(rhs)
            }
        }

        impl<TR, BR> $Trait<Bounded<TR, BR>> for $Type
        where
            BR: AsBound<TR>,
            $Type: $Trait<TR>,
        {
            type Output = <$Type as $Trait<TR>>::Output;

            #[inline]
            fn $func(self, rhs: Bounded<TR, BR>) -> Self::Output {
                self.$func(rhs.value())
            }
        }
    };
    ({ $($Type:ty),+ }: $Trait:ident::$func:ident) => {
        $(impl_ops_reflective! { $Type, $Trait, $func })+
    }
}

macro_rules! impl_assign {
    ($Type:ty, $Trait:ident, $func:ident) => {
        impl<TR, BR> $Trait<Bounded<TR, BR>> for $Type
        where
            BR: AsBound<TR>,
            $Type: $Trait<TR>,
        {
            #[inline]
            fn $func(&mut self, rhs: Bounded<TR, BR>) {
                self.$func(rhs.value())
            }
        }
    };
    ({$($Type:ty),+}: $Trait:ident::$func:ident) => {
        $(impl_assign! { $Type, $Trait, $func })+
    }
}

mod arith;
mod bit;
