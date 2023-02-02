use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

mod sealed {
    pub trait Sealed {}
}

impl sealed::Sealed for f32 {}
impl sealed::Sealed for f64 {}

macro_rules! forward_float_impl {
    ($f32:ident, $f64:ident, [$($cnt:ident),*], $(fn $method:ident($($aname:ident: $aty:ty),*) $(-> $ret:ty)?);*$(;)?) => {
        pub trait Float:
            Add<Output = Self> +
            AddAssign +
            Div<Output = Self> +
            DivAssign +
            Mul<Output = Self> +
            MulAssign +
            Sub<Output = Self> +
            SubAssign +
            Neg<Output = Self> +
            Clone +
            Copy +
            Debug +
            Display +
            PartialEq +
            PartialOrd +
            sealed::Sealed +
            Sized +
            'static
        {
            const ONE: Self;
            const ZERO: Self;

            $(
                fn $method(&self, $($aname: $aty)*) $(-> $ret)?;
            )*
        }

        impl Float for $f32 {
            const ONE: Self = 1.0;
            const ZERO: Self = 0.0;

            $(
                #[inline(always)]
                fn $method(&self, $($aname: $aty)*) $(-> $ret)? {
                    self.$method($($aname),*)
                }
            )*
        }

        impl Float for $f64 {
            const ONE: Self = 1.0;
            const ZERO: Self = 0.0;

            $(
                #[inline(always)]
                fn $method(&self, $($aname: $aty)*) $(-> $ret)? {
                    self.$method($($aname),*)
                }
            )*
        }
    };
}

forward_float_impl! { f32, f64,
    [],

    fn sin() -> Self;
    fn cos() -> Self;
    fn atan2(x: Self) -> Self;
    fn exp() -> Self;
    fn to_radians() -> Self;
    fn to_degrees() -> Self;
    fn sqrt() -> Self;
    fn signum() -> Self;
}
