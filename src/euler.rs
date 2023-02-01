use crate::FloatType as F;
use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// Euler degree angles.
pub type EulerD = Euler<Deg>;
/// Euler radian angles.
pub type EulerR = Euler<Rad>;

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

/// Helper trait to distinguish between radians and degrees.
pub trait AngleMeasure: private::Sealed {}

impl AngleMeasure for Rad {}
impl AngleMeasure for Deg {}

/// Euler angles
#[derive(Clone, Copy)]
pub struct Euler<A: AngleMeasure> {
    /// Rotation around Z axis.
    pub yaw: F,
    /// Rotation around X axis.
    pub pitch: F,
    /// Rotation around Y axis.
    pub roll: F,

    _pd: PhantomData<A>,
}

impl<A: AngleMeasure> Debug for Euler<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Euler")
            .field("yaw", &self.yaw)
            .field("pitch", &self.pitch)
            .field("roll", &self.roll)
            .finish()
    }
}

impl Euler<Rad> {
    /// Creates new euler angles. `yaw`, `pitch`, `roll` must be in radians!
    #[inline]
    pub fn new(yaw: F, pitch: F, roll: F) -> Self {
        Self {
            yaw,
            pitch,
            roll,
            _pd: PhantomData,
        }
    }

    /// Converts radians to degrees.
    pub fn to_degrees(self) -> Euler<Deg> {
        Euler {
            yaw: self.yaw.to_degrees(),
            pitch: self.pitch.to_degrees(),
            roll: self.roll.to_degrees(),
            _pd: PhantomData,
        }
    }
}

impl Euler<Deg> {
    /// Creates new euler angles. `yaw`, `pitch`, `roll` must be in degrees!
    pub fn new(yaw: F, pitch: F, roll: F) -> Self {
        Self {
            yaw,
            pitch,
            roll,
            _pd: PhantomData,
        }
    }

    /// Converts degrees to radians.
    pub fn to_radians(self) -> Euler<Rad> {
        Euler {
            yaw: self.yaw.to_radians(),
            pitch: self.pitch.to_radians(),
            roll: self.roll.to_radians(),
            _pd: PhantomData,
        }
    }
}
impl<A: AngleMeasure> Add for Euler<A> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            yaw: self.yaw + rhs.yaw,
            pitch: self.pitch + rhs.pitch,
            roll: self.roll + rhs.roll,
            _pd: PhantomData,
        }
    }
}

impl<A: AngleMeasure> AddAssign for Euler<A> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.yaw += rhs.yaw;
        self.pitch += rhs.pitch;
        self.roll += rhs.roll;
    }
}

impl<A: AngleMeasure> Sub for Euler<A> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            yaw: self.yaw - rhs.yaw,
            pitch: self.pitch - rhs.pitch,
            roll: self.roll - rhs.roll,
            _pd: PhantomData,
        }
    }
}

impl<A: AngleMeasure> SubAssign for Euler<A> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.yaw -= rhs.yaw;
        self.pitch -= rhs.pitch;
        self.roll -= rhs.roll;
    }
}

impl<A: AngleMeasure> Mul<F> for Euler<A> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: F) -> Self {
        Self {
            yaw: self.yaw * rhs,
            pitch: self.pitch * rhs,
            roll: self.roll * rhs,
            _pd: PhantomData,
        }
    }
}

impl<A: AngleMeasure> MulAssign<F> for Euler<A> {
    #[inline]
    fn mul_assign(&mut self, rhs: F) {
        self.yaw *= rhs;
        self.pitch *= rhs;
        self.roll *= rhs;
    }
}

impl<A: AngleMeasure> Div<F> for Euler<A> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: F) -> Self {
        Self {
            yaw: self.yaw / rhs,
            pitch: self.pitch / rhs,
            roll: self.roll / rhs,
            _pd: PhantomData,
        }
    }
}

impl<A: AngleMeasure> DivAssign<F> for Euler<A> {
    #[inline]
    fn div_assign(&mut self, rhs: F) {
        self.yaw /= rhs;
        self.pitch /= rhs;
        self.roll /= rhs;
    }
}

impl<A: AngleMeasure> Neg for Euler<A> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            yaw: -self.yaw,
            pitch: -self.pitch,
            roll: -self.roll,
            _pd: PhantomData,
        }
    }
}
