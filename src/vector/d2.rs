use crate::{Complex, Float, Vector3};
use std::{cmp::Ordering, ops::Mul};

/// 2 Dimensional vector.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vector2<F: Float> {
    pub x: F,
    pub y: F,
}

impl<F: Float> Vector2<F> {
    pub const ZERO: Self = Self::new(F::ZERO, F::ZERO);
    pub const ONE: Self = Self::new(F::ONE, F::ONE);

    pub const X: Self = Self::new(F::ONE, F::ZERO);
    pub const Y: Self = Self::new(F::ZERO, F::ONE);
    pub const XY: Self = Self::new(F::ONE, F::ONE);
}

impl<F: Float> Vector2<F> {
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

    /// Returns maximum element of the vector.
    #[inline]
    pub fn max_element(&self) -> F {
        self.x.max(self.y)
    }

    /// Returns minumum element of the vector.
    #[inline]
    pub fn min_element(&self) -> F {
        self.x.min(self.y)
    }

    /// Returns index of the maximum element.
    /// Index is in `0..=1` range.
    #[inline]
    pub fn max_index(&self) -> usize {
        [(self.x, 0), (self.y, 1)]
            .iter()
            .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(_, i)| *i)
            .unwrap()
    }

    /// Returns index of the minumum element.
    /// Index is in `0..=1` range.
    #[inline]
    pub fn min_index(&self) -> usize {
        [(self.x, 0), (self.y, 1)]
            .iter()
            .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(_, i)| *i)
            .unwrap()
    }

    #[inline]
    pub fn reflect(&self, axis: Self) -> Self {
        self.projected_onto(axis) * F::TWO - *self
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

crate::__impl_vec_ops!(Vector2, 1, x, y);
crate::__impl_planar_ops!(Vector2, [x, 0, F], [y, 1, F]);
