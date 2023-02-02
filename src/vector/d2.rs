use crate::{Complex, Float, Vector3};
use std::ops::Mul;

/// 2 Dimensional vector.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vector2<F: Float> {
    pub x: F,
    pub y: F,
}

impl<F: Float> Vector2<F> {
    /// Creates new vector.
    #[inline]
    pub const fn new(x: F, y: F) -> Self {
        Self { x, y }
    }

    /// Converts a vector to a complex number with `real` = `x`, `imag` = `y`.
    #[inline]
    pub const fn to_complex(self) -> Complex<F> {
        Complex {
            real: self.x,
            imag: self.y,
        }
    }

    /// Converts a complex number to a vector with `x` = `real`, `y` = `imag`.
    #[inline]
    pub const fn from_complex(complex: Complex<F>) -> Self {
        Self {
            x: complex.real,
            y: complex.imag,
        }
    }

    /// Extends the vector with `z` component to create a [`Vector3`].
    #[inline]
    pub const fn extend(self, z: F) -> Vector3<F> {
        Vector3 {
            x: self.x,
            y: self.y,
            z,
        }
    }

    /// Computes dot product
    #[inline]
    pub fn dot(&self, other: Self) -> F {
        self.x * other.x + self.y * other.y
    }

    /// Rotates angle around origin by some angle `angle` in radians counter-clockwise.
    #[inline]
    pub fn rotate_by(&mut self, angle: F) {
        *self = *self * Complex::from_angle(angle)
    }

    /// Returns a rotated copy of a vector. See [`Self::rotate_by`].
    #[inline]
    pub fn rotated_by(self, angle: F) -> Self {
        self * Complex::from_angle(angle)
    }

    /// Rotates angle around origin by some angle `angle` in radians clockwise.
    #[inline]
    pub fn rotate_by_clockwise(&mut self, angle: F) {
        *self = *self * Complex::from_angle(angle).conjugate()
    }

    /// Returns a rotated copy of a vector. See [`Self::rotate_by_clockwise`].
    #[inline]
    pub fn rotated_by_clockwise(self, angle: F) -> Self {
        self * Complex::from_angle(angle).conjugate()
    }
}

impl<F: Float> From<Complex<F>> for Vector2<F> {
    #[inline]
    fn from(val: Complex<F>) -> Self {
        Self {
            x: val.real,
            y: val.imag,
        }
    }
}

impl<F: Float> Mul<Complex<F>> for Vector2<F> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Complex<F>) -> Self::Output {
        Self {
            x: self.x * rhs.real - self.y * rhs.imag,
            y: self.x * rhs.imag + self.y * rhs.real,
        }
    }
}

unsafe impl<F: Float> bytemuck::Pod for Vector2<F> {}
unsafe impl<F: Float> bytemuck::Zeroable for Vector2<F> {}
