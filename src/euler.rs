use crate::{Angle, Deg, FloatType as F, Rad};
use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// Euler degree angles.
pub type EulerD = Euler<Deg>;
/// Euler radian angles.
pub type EulerR = Euler<Rad>;

/// Euler angles
#[derive(Clone, Copy)]
pub struct Euler<A: Angle> {
    /// Rotation around Z axis.
    pub yaw: F,
    /// Rotation around X axis.
    pub pitch: F,
    /// Rotation around Y axis.
    pub roll: F,

    _pd: PhantomData<A>,
}

impl<A: Angle> Debug for Euler<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Euler")
            .field("yaw", &self.yaw)
            .field("pitch", &self.pitch)
            .field("roll", &self.roll)
            .finish()
    }
}

impl<A: Angle> Euler<A> {
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

impl Euler<Rad> {
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

impl<A: Angle> Add for Euler<A> {
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

impl<A: Angle> AddAssign for Euler<A> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.yaw += rhs.yaw;
        self.pitch += rhs.pitch;
        self.roll += rhs.roll;
    }
}

impl<A: Angle> Sub for Euler<A> {
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

impl<A: Angle> SubAssign for Euler<A> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.yaw -= rhs.yaw;
        self.pitch -= rhs.pitch;
        self.roll -= rhs.roll;
    }
}

impl<A: Angle> Mul<F> for Euler<A> {
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

impl<A: Angle> MulAssign<F> for Euler<A> {
    #[inline]
    fn mul_assign(&mut self, rhs: F) {
        self.yaw *= rhs;
        self.pitch *= rhs;
        self.roll *= rhs;
    }
}

impl<A: Angle> Div<F> for Euler<A> {
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

impl<A: Angle> DivAssign<F> for Euler<A> {
    #[inline]
    fn div_assign(&mut self, rhs: F) {
        self.yaw /= rhs;
        self.pitch /= rhs;
        self.roll /= rhs;
    }
}

impl<A: Angle> Neg for Euler<A> {
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
