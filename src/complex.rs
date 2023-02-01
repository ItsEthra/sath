use crate::{FloatType as F, Matrix2, Vector2};
use std::ops::{Div, DivAssign, Mul, MulAssign};

/// Complex number
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub real: F,
    pub imag: F,
}

impl Complex {
    /// Converts complex number to a vector where `x` = `real`, `y` = `imag`.
    pub const fn to_vector2(self) -> Vector2 {
        Vector2 {
            x: self.real,
            y: self.imag,
        }
    }

    /// Converts vector to a complex number where `real` = `x`, `imag` = `y`.
    pub const fn from_vector2(vec: Vector2) -> Self {
        Self {
            real: vec.x,
            imag: vec.y,
        }
    }

    /// Converts complex number to a real matrix.
    #[inline]
    pub fn to_matrix2(self) -> Matrix2 {
        Matrix2::new(self.real, -self.imag, self.imag, self.real)
    }

    /// Converts `angle` in radians to a complex number.
    #[inline]
    pub fn from_angle(angle: F) -> Self {
        Self {
            real: angle.cos(),
            imag: angle.sin(),
        }
    }

    /// Extracts angle from complex number.
    #[inline]
    pub fn angle(self) -> F {
        self.imag.atan2(self.real)
    }

    /// Returns complex number's magnitude and angle in radians.
    #[inline]
    pub fn to_magnitude_angle(self) -> (F, F) {
        (self.magnitude(), self.angle())
    }

    /// Returns conjugate of the complex number.
    /// Conjugate is defined as `a - bi`.
    #[inline]
    pub fn conjugate(self) -> Self {
        Self {
            imag: -self.imag,
            ..self
        }
    }

    /// Returns `1 / (a + bi)`.
    #[inline]
    pub fn reciprocal(self) -> Self {
        let sqr_mag = self.sqr_magnitude();

        Self {
            real: self.real / sqr_mag,
            imag: -self.imag / sqr_mag,
        }
    }

    /// Computes square root of complex number.
    #[inline]
    pub fn sqrt(self) -> (Self, Self) {
        let mag = self.magnitude();
        let gamma = ((self.real + mag) / 2.).sqrt();
        let delta = self.imag.signum() * ((-self.real + mag) / 2.).sqrt();

        (
            Self {
                real: gamma,
                imag: delta,
            },
            Self {
                real: -gamma,
                imag: -delta,
            },
        )
    }

    /// Computes `e` raised to the complex power.
    #[inline]
    pub fn exp(&self) -> Self {
        Self {
            real: self.real.exp() * self.imag.cos(),
            imag: self.real.exp() * self.imag.sin(),
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl MulAssign for Complex {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.real = self.real * rhs.real - self.imag * rhs.imag;
        self.imag = self.real * rhs.imag + self.imag * rhs.real;
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for Complex {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.reciprocal()
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl DivAssign for Complex {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.reciprocal();
    }
}

impl From<Vector2> for Complex {
    #[inline]
    fn from(val: Vector2) -> Self {
        Self {
            real: val.x,
            imag: val.y,
        }
    }
}

crate::__impl_planar_ops!(Complex, [real, 0, F], [imag, 1, F]);

/// Creates new complex number where `real` = first argument and `imag` = second argument.
#[macro_export]
macro_rules! complex {
    ($real:expr, $imag:expr) => {
        $crate::Complex::new($real as _, $imag as _)
    };
}
