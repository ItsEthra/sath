use crate::{matrix, Euler, FloatType as F, Matrix3, Rad, Vector3};
use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

/// Quaternion representing a rotation in 3d space.
#[derive(Clone, Copy, PartialEq)]
pub struct Quaternion {
    /// Scalar part.
    pub scalar: F,
    /// Vector part.
    pub vector: Vector3,
}

impl Quaternion {
    /// Zero quaternion with all elements set to `0`.
    pub const ZERO: Self = Self {
        scalar: 0.0,
        vector: Vector3::ZERO,
    };

    /// Identity quaternion representing no rotation.
    pub const IDENTITY: Self = Self {
        scalar: 1.0,
        vector: Vector3::ZERO,
    };
}

impl Quaternion {
    /// Creates a new quaternion from individual elements.
    #[inline]
    pub fn new(scalar: F, vector: Vector3) -> Self {
        Self { scalar, vector }
    }

    /// Converts axis, angle represention to a quaternion that represents a rotation around `axis`
    /// on `angle` in radians.
    /// To avoid unexpected results, use normalized axis.
    #[inline]
    pub fn new_axis_angle(axis: Vector3, angle: F) -> Self {
        let half = angle / 2.;

        Self {
            scalar: half.cos(),
            vector: axis * half.sin(),
        }
    }

    /// Creates a rotation that rotates forward vector to face `target` from position `from`,
    /// aligned upwards.
    pub fn new_look_at(target: Vector3, eye: Vector3) -> Self {
        let q1 = Self::new_axis_angle(
            Vector3::FORWARD.cross(target - eye).normalized(),
            (target - eye).dot_normalized(Vector3::FORWARD).acos(),
        );

        let aligned_r = Vector3::FORWARD
            .rotated_by(q1)
            .cross(Vector3::UP)
            .normalized();
        let loc_r = Vector3::RIGHT.rotated_by(q1);

        let q2 = Self::new_axis_angle(
            loc_r.cross(aligned_r).normalized(),
            loc_r.dot_normalized(aligned_r).acos(),
        );

        q2 * q1
    }

    /// Recovers axis angle represention.
    #[inline]
    pub fn into_axis_angle(&self) -> (Vector3, F) {
        (
            self.vector.normalized(),
            self.vector.magnitude().atan2(self.scalar) * 2.,
        )
    }

    /// Converts quaternion into euler angles.
    #[inline]
    pub fn into_euler(&self) -> Euler<Rad> {
        Euler::<Rad>::new(
            (2. * (self.scalar * self.vector.z + self.vector.x * self.vector.y))
                .atan2(1. - 2. * (self.vector.y * self.vector.y + self.vector.z * self.vector.z)),
            (2. * (self.scalar * self.vector.x + self.vector.y * self.vector.z))
                .atan2(1. - 2. * (self.vector.x * self.vector.x + self.vector.y * self.vector.y)),
            (2. * (self.scalar * self.vector.y - self.vector.z * self.vector.x)).asin(),
        )
    }

    /// Converts euler angles to quaternion.
    #[inline]
    pub fn from_euler(angles: Euler<Rad>) -> Self {
        let half = angles / 2.;

        Self {
            scalar: half.pitch.cos() * half.roll.cos() * half.yaw.cos()
                + half.pitch.sin() * half.roll.sin() * half.yaw.sin(),
            vector: Vector3 {
                x: half.pitch.sin() * half.roll.cos() * half.yaw.cos()
                    - half.pitch.cos() * half.roll.sin() * half.yaw.sin(),
                y: half.pitch.cos() * half.roll.sin() * half.yaw.cos()
                    + half.pitch.sin() * half.roll.cos() * half.yaw.sin(),
                z: half.pitch.cos() * half.roll.cos() * half.yaw.sin()
                    - half.pitch.sin() * half.roll.sin() * half.yaw.cos(),
            },
        }
    }

    /// Creates a new quaternion with vector part equal to `vector` and scalar part to `0`.
    #[inline]
    pub fn from_vector(vector: Vector3) -> Self {
        Self {
            scalar: 0.0,
            vector,
        }
    }

    /// Returns the conjugate of the quaternion.
    /// `a + bi + cj + dk` -> `a - bi - cj - dk`
    #[inline]
    pub fn conjugate(self) -> Self {
        Self {
            vector: -self.vector,
            ..self
        }
    }

    /// Computes squared norm of the quaternion.
    #[inline]
    pub fn sqr_norm(&self) -> F {
        self.scalar * self.scalar + self.vector.sqr_magnitude()
    }

    /// Computes norm of the quaternion.
    #[inline]
    pub fn norm(&self) -> F {
        self.sqr_norm().sqrt()
    }

    /// Normalizes quaternion making its norm equal to `1`.
    #[inline]
    pub fn normalize(&mut self) {
        let norm = self.norm();

        self.scalar /= norm;
        self.vector /= norm;
    }

    /// Returns normalized copy of the quaternion. See [`Self::normalize`]
    #[inline]
    pub fn normalized(self) -> Self {
        let norm = self.norm();

        Self {
            scalar: self.scalar / norm,
            vector: self.vector / norm,
        }
    }

    /// Computes the reciprocal of the quaternion.
    #[inline]
    pub fn reciprocal(self) -> Self {
        self.conjugate() / (self.norm() * self.norm())
    }

    /// Computes the hamilton product of two quaternions.
    pub fn hamilton_product(self, rhs: &Self) -> Self {
        Self {
            scalar: self.scalar * rhs.scalar
                - self.vector.x * rhs.vector.x
                - self.vector.y * rhs.vector.y
                - self.vector.z * rhs.vector.z,
            vector: Vector3 {
                x: self.scalar * rhs.vector.x
                    + self.vector.x * rhs.scalar
                    + self.vector.y * rhs.vector.z
                    - self.vector.z * rhs.vector.y,
                y: self.scalar * rhs.vector.y - self.vector.x * rhs.vector.z
                    + self.vector.y * rhs.scalar
                    + self.vector.z * rhs.vector.x,
                z: self.scalar * rhs.vector.z + self.vector.x * rhs.vector.y
                    - self.vector.y * rhs.vector.x
                    + self.vector.z * rhs.scalar,
            },
        }
    }

    /// Computes the exponent raised to a quaternion power.
    pub fn exp(self) -> Self {
        let mag = self.vector.magnitude();

        Self {
            scalar: mag.cos(),
            vector: self.vector / mag * mag.sin(),
        } * self.scalar.exp()
    }

    /// Computes the natural logarithm of the quaternion.
    pub fn ln(self) -> Self {
        Self {
            scalar: self.norm().ln(),
            vector: self.vector.normalized() * (self.scalar / self.norm()).acos(),
        }
    }

    /// Raises quaternion to `x` power.
    pub fn powf(self, x: F) -> Self {
        (self.ln() * x).exp()
    }

    /// Linearly interpolates the quaternion.
    pub fn lerp(self, end: Self, t: F) -> Self {
        self * (1.0 - t) + end * t
    }

    /// Returns a normalized copy of linear interpolation.
    pub fn nlerp(self, end: Self, t: F) -> Self {
        self.lerp(end, t).normalized()
    }

    /// Spherically interpolates quaternions.
    pub fn slerp(self, end: Self, t: F) -> Self {
        self * (self.reciprocal() * end).powf(t)
    }

    /// Converts a quaternion representing rotation to a matrix representing the same rotation.
    pub fn into_matrix3(self) -> Matrix3 {
        matrix!(
            self.scalar * self.scalar + self.vector.x * self.vector.x
                - self.vector.y * self.vector.y
                - self.vector.z * self.vector.z,
            2. * self.vector.x * self.vector.y - 2. * self.scalar * self.vector.z,
            2. * self.vector.x * self.vector.z + 2. * self.scalar * self.vector.y,
            2. * self.vector.x * self.vector.y + 2. * self.scalar * self.vector.z,
            self.scalar * self.scalar - self.vector.x * self.vector.x
                + self.vector.y * self.vector.y
                - self.vector.z * self.vector.z,
            2. * self.vector.y * self.vector.z - 2. * self.scalar * self.vector.x,
            2. * self.vector.x * self.vector.z - 2. * self.scalar * self.vector.y,
            2. * self.vector.y * self.vector.z + 2. * self.scalar * self.vector.x,
            self.scalar * self.scalar
                - self.vector.x * self.vector.x
                - self.vector.y * self.vector.y
                + self.vector.z * self.vector.z,
        )
    }
}

impl From<(Vector3, F)> for Quaternion {
    /// Converts from axis, angle to quaternion.
    fn from((axis, angle): (Vector3, F)) -> Self {
        Self::new_axis_angle(axis, angle)
    }
}

impl Mul for Quaternion {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.hamilton_product(&rhs)
    }
}

impl MulAssign for Quaternion {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.hamilton_product(&rhs);
    }
}

impl Mul<F> for Quaternion {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: F) -> Self::Output {
        Self {
            scalar: self.scalar * rhs,
            vector: self.vector * rhs,
        }
    }
}

impl MulAssign<F> for Quaternion {
    #[inline]
    fn mul_assign(&mut self, rhs: F) {
        self.scalar *= rhs;
        self.vector *= rhs;
    }
}

impl Div<F> for Quaternion {
    type Output = Self;

    #[inline]
    fn div(self, rhs: F) -> Self::Output {
        Self {
            scalar: self.scalar / rhs,
            vector: self.vector / rhs,
        }
    }
}

impl DivAssign<F> for Quaternion {
    #[inline]
    fn div_assign(&mut self, rhs: F) {
        self.scalar /= rhs;
        self.vector /= rhs;
    }
}

impl Add for Quaternion {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            scalar: self.scalar + rhs.scalar,
            vector: self.vector + rhs.vector,
        }
    }
}

impl AddAssign for Quaternion {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.scalar += rhs.scalar;
        self.vector += rhs.vector;
    }
}

impl Sub for Quaternion {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            scalar: self.scalar - rhs.scalar,
            vector: self.vector - rhs.vector,
        }
    }
}

impl SubAssign for Quaternion {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.scalar -= rhs.scalar;
        self.vector -= rhs.vector;
    }
}

impl fmt::Debug for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (axis, angle) = self.into_axis_angle();

        f.debug_struct("Quaternion")
            .field("axix", &axis)
            .field("angle", &angle.to_degrees())
            .finish()
    }
}
