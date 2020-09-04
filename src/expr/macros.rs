macro_rules! define_unary_operator {
    (
        $mod_name:ident,
        $(#[$meta:meta])*
            $name:ident,
        $val_trait:path,
        $val_func:path,
        $type_operator:path,
    ) => {
        pub use $mod_name::*;
        mod $mod_name {
            use super::*;
            use $val_trait as ValTrait;
            use $type_operator as TypeOperator;
            use core::marker::PhantomData;

            $(#[$meta])*
            #[derive(Debug, Default, Clone, Copy)]
            pub struct $name<S>(PhantomData<S>);
            impl<A, S> ValueExpr<A> for $name<S>
            where
                S: ValueExpr<A>,
                ValType<S, A>: ValTrait,
            {
                type Output = <ValType<S, A> as ValTrait>::Output;

                fn call(arg: A) -> Self::Output {
                    $val_func(S::call(arg))
                }
            }
            impl<A, S> TypeExpr<A> for $name<S>
            where
                S: TypeExpr<A>,
                Call<S, A>: TypeOperator,
            {
                type Output = <Call<S, A> as TypeOperator>::Output;
            }
        }
    };
}

macro_rules! define_binary_operator {
    (
        $mod_name: ident,
        $(#[$meta:meta])*
            $name:ident,
        $val_trait:path,
        $val_func:path,
        $type_operator:path,
    ) => {
        pub use $mod_name::*;
        mod $mod_name {
            use super::*;
            use $type_operator as TypeOperator;
            use $val_trait as ValTrait;
            use core::marker::PhantomData;

            $(#[$meta])*
            #[derive(Debug, Default, Clone, Copy)]
            pub struct $name<L, R> {
                lhs: PhantomData<L>,
                rhs: PhantomData<R>,
            }

            impl<A, L, R> ValueExpr<A> for $name<L, R>
            where
                A: Copy,
                L: ValueExpr<A>,
                R: ValueExpr<A>,
                ValType<L, A>: ValTrait<ValType<R, A>>,
            {
                type Output = <ValType<L, A> as ValTrait<ValType<R, A>>>::Output;

                fn call(arg: A) -> Self::Output {
                    $val_func(L::call(arg), R::call(arg))
                }
            }

            impl<A, L, R> TypeExpr<A> for $name<L, R>
            where
                L: TypeExpr<A>,
                R: TypeExpr<A>,
                Call<L, A>: TypeOperator<Call<R, A>>,
            {
                type Output = <Call<L, A> as TypeOperator<Call<R, A>>>::Output;
            }
        }
    };
}
