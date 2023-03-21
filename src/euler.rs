use crate::{Deg, Float, Measure, Rad};
use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// Euler degree-float angles.
pub type EulerDegf = Euler<Deg, f32>;
/// Euler degree-float angles.
pub type EulerDegd = Euler<Deg, f64>;

/// Euler radian angles.
pub type EulerRadf = Euler<Rad, f32>;
/// Euler radian angles.
pub type EulerRadd = Euler<Rad, f64>;

/// Euler angles
#[derive(Clone, Copy)]
pub struct Euler<A: Measure, F: Float> {
    /// Rotation around Z axis.
    pub yaw: F,
    /// Rotation around X axis.
    pub pitch: F,
    /// Rotation around Y axis.
    pub roll: F,

    _pd: PhantomData<A>,
}

impl<A: Measure, F: Float> Debug for Euler<A, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Euler")
            .field("yaw", &self.yaw)
            .field("pitch", &self.pitch)
            .field("roll", &self.roll)
            .finish()
    }
}

impl<A: Measure, F: Float> Euler<A, F> {
    /// Creates new euler angles from `yaw`, `pitch`, `roll`.
    pub fn new(yaw: F, pitch: F, roll: F) -> Self {
        Self {
            yaw,
            pitch,
            roll,
            _pd: PhantomData,
        }
    }
}

impl<F: Float> Euler<Rad, F> {
    /// Converts radians to degrees.
    pub fn to_degrees(self) -> Euler<Deg, F> {
        Euler {
            yaw: self.yaw.to_degrees(),
            pitch: self.pitch.to_degrees(),
            roll: self.roll.to_degrees(),
            _pd: PhantomData,
        }
    }
}

impl<F: Float> Euler<Deg, F> {
    /// Converts degrees to radians.
    pub fn to_radians(self) -> Euler<Rad, F> {
        Euler {
            yaw: self.yaw.to_radians(),
            pitch: self.pitch.to_radians(),
            roll: self.roll.to_radians(),
            _pd: PhantomData,
        }
    }
}

impl<A: Measure, F: Float> Add for Euler<A, F> {
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

impl<A: Measure, F: Float> AddAssign for Euler<A, F> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.yaw += rhs.yaw;
        self.pitch += rhs.pitch;
        self.roll += rhs.roll;
    }
}

impl<A: Measure, F: Float> Sub for Euler<A, F> {
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

impl<A: Measure, F: Float> SubAssign for Euler<A, F> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.yaw -= rhs.yaw;
        self.pitch -= rhs.pitch;
        self.roll -= rhs.roll;
    }
}

impl<A: Measure, F: Float> Mul<F> for Euler<A, F> {
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

impl<A: Measure, F: Float> MulAssign<F> for Euler<A, F> {
    #[inline]
    fn mul_assign(&mut self, rhs: F) {
        self.yaw *= rhs;
        self.pitch *= rhs;
        self.roll *= rhs;
    }
}

impl<A: Measure, F: Float> Div<F> for Euler<A, F> {
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

impl<A: Measure, F: Float> DivAssign<F> for Euler<A, F> {
    #[inline]
    fn div_assign(&mut self, rhs: F) {
        self.yaw /= rhs;
        self.pitch /= rhs;
        self.roll /= rhs;
    }
}

impl<A: Measure, F: Float> Neg for Euler<A, F> {
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
