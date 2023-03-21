use crate::Float;
use std::{
    fmt::{self, Debug, Display},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

/// Radians marker type.
#[derive(Debug, Clone, Copy)]
pub struct Rad;

/// Degrees marker type.
#[derive(Debug, Clone, Copy)]
pub struct Deg;

mod private {
    pub trait Sealed {}
    impl Sealed for super::Rad {}
    impl Sealed for super::Deg {}
}

/// Single precession angle.
pub type Anglef<M> = Angle<f32, M>;
/// Double precession angle.
pub type Angled<M> = Angle<f64, M>;

/// Angle with marker to indicate its mesure.
#[repr(transparent)]
pub struct Angle<F: Float, M: Measure>(pub F, PhantomData<M>);

impl<F: Float, M: Measure> Debug for Angle<F, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<F: Float, M: Measure> Display for Angle<F, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<F: Float, M: Measure> PartialEq for Angle<F, M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<F: Float, M: Measure> PartialOrd for Angle<F, M> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<F: Float, M: Measure> From<F> for Angle<F, M> {
    #[inline]
    fn from(value: F) -> Self {
        Self(value, PhantomData)
    }
}

impl<F: Float, M: Measure> Deref for Angle<F, M> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F: Float, M: Measure> DerefMut for Angle<F, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Helper trait to distinguish between radians and degrees.
pub trait Measure: private::Sealed {}

impl Measure for Rad {}
impl Measure for Deg {}
