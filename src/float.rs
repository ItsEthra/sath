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
    ($f32:ident, $f64:ident, $(fn $method:ident($($aname:ident: $aty:ty),*) $(-> $ret:ty)?);*$(;)?) => {
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
            const PI: Self;
            const EPSILON: Self;
            const TWO: Self;
            const ONE: Self;
            const ZERO: Self;

            $(
                fn $method(&self, $($aname: $aty),*) $(-> $ret)?;
            )*
        }

        impl Float for $f32 {
            const PI: Self = core::f32::consts::PI;
            const EPSILON: Self = f32::EPSILON;
            const TWO: Self = 2.0;
            const ONE: Self = 1.0;
            const ZERO: Self = 0.0;

            $(
                #[inline(always)]
                fn $method(&self, $($aname: $aty),*) $(-> $ret)? {
                    (*self as f32).$method($($aname),*)
                }
            )*
        }

        impl Float for $f64 {
            const PI: Self = core::f64::consts::PI;
            const EPSILON: Self = f64::EPSILON;
            const TWO: Self = 2.0;
            const ONE: Self = 1.0;
            const ZERO: Self = 0.0;

            $(
                #[inline(always)]
                fn $method(&self, $($aname: $aty),*) $(-> $ret)? {
                    (*self as f64).$method($($aname),*)
                }
            )*
        }
    };
}

forward_float_impl! { f32, f64,
    fn sin() -> Self;
    fn cos() -> Self;
    fn atan2(x: Self) -> Self;
    fn exp() -> Self;
    fn to_radians() -> Self;
    fn to_degrees() -> Self;
    fn sqrt() -> Self;
    fn signum() -> Self;
    fn abs() -> Self;
    fn acos() -> Self;
    fn ln() -> Self;
    fn asin() -> Self;
    fn clamp(from: Self, to: Self) -> Self;
    fn max(other: Self) -> Self;
    fn min(other: Self) -> Self;
}
