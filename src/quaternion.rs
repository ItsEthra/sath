use crate::{matrix, Euler, Float, Matrix3, Rad, Vector3};
use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

/// Quaternion representing a rotation in 3d space.
#[derive(Clone, Copy, PartialEq)]
pub struct Quaternion<F: Float> {
    /// Scalar part.
    pub scalar: F,
    /// Vector part.
    pub vector: Vector3<F>,
}

impl<F: Float> Quaternion<F> {
    /// Creates a new quaternion from individual elements.
    #[inline]
    pub fn new(scalar: F, vector: Vector3<F>) -> Self {
        Self { scalar, vector }
    }

    /// Converts axis, angle represention to a quaternion that represents a rotation around `axis`
    /// on `angle` in radians.
    /// To avoid unexpected results, use normalized axis.
    #[inline]
    pub fn new_axis_angle(axis: Vector3<F>, angle: F) -> Self {
        let half = angle / (F::ONE + F::ONE);

        Self {
            scalar: half.cos(),
            vector: axis * half.sin(),
        }
    }

    // Creates a rotation that rotates forward vector to face `target` from position `from`,
    // aligned upwards.
    // pub fn new_look_at(target: Vector3<F>, eye: Vector3<F>) -> Self {
    //     let q1 = Self::new_axis_angle(
    //         Vector3::FORWARD.cross(target - eye).normalized(),
    //         (target - eye).dot_normalized(Vector3::FORWARD).acos(),
    //     );

    //     let aligned_r = Vector3::FORWARD
    //         .rotated_by(q1)
    //         .cross(Vector3::UP)
    //         .normalized();
    //     let loc_r = Vector3::RIGHT.rotated_by(q1);

    //     let q2 = Self::new_axis_angle(
    //         loc_r.cross(aligned_r).normalized(),
    //         loc_r.dot_normalized(aligned_r).acos(),
    //     );

    //     q2 * q1
    // }

    /// Recovers axis angle represention.
    #[inline]
    pub fn into_axis_angle(&self) -> (Vector3<F>, F) {
        (
            self.vector.normalized(),
            self.vector.magnitude().atan2(self.scalar) * F::TWO,
        )
    }
    /// Converts quaternion into euler angles.
    #[inline]
    pub fn into_euler(&self) -> Euler<Rad, F> {
        Euler::new(
            (F::TWO * (self.scalar * self.vector.z + self.vector.x * self.vector.y)).atan2(
                F::ONE - F::TWO * (self.vector.y * self.vector.y + self.vector.z * self.vector.z),
            ),
            (F::TWO * (self.scalar * self.vector.x + self.vector.y * self.vector.z)).atan2(
                F::ONE - F::TWO * (self.vector.x * self.vector.x + self.vector.y * self.vector.y),
            ),
            (F::TWO * (self.scalar * self.vector.y - self.vector.z * self.vector.x)).asin(),
        )
    }

    /// Converts euler angles to quaternion.
    #[inline]
    pub fn from_euler(angles: Euler<Rad, F>) -> Self {
        let half = angles / F::TWO;

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
    pub fn from_vector(vector: Vector3<F>) -> Self {
        Self {
            scalar: F::ZERO,
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
        self * (F::ONE - t) + end * t
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
    pub fn into_matrix3(self) -> Matrix3<F> {
        matrix!(
            self.scalar * self.scalar + self.vector.x * self.vector.x
                - self.vector.y * self.vector.y
                - self.vector.z * self.vector.z,
            (F::ONE + F::ONE) * self.vector.x * self.vector.y
                - (F::ONE + F::ONE) * self.scalar * self.vector.z,
            (F::ONE + F::ONE) * self.vector.x * self.vector.z
                + (F::ONE + F::ONE) * self.scalar * self.vector.y,
            (F::ONE + F::ONE) * self.vector.x * self.vector.y
                + (F::ONE + F::ONE) * self.scalar * self.vector.z,
            self.scalar * self.scalar - self.vector.x * self.vector.x
                + self.vector.y * self.vector.y
                - self.vector.z * self.vector.z,
            (F::ONE + F::ONE) * self.vector.y * self.vector.z
                - (F::ONE + F::ONE) * self.scalar * self.vector.x,
            (F::ONE + F::ONE) * self.vector.x * self.vector.z
                - (F::ONE + F::ONE) * self.scalar * self.vector.y,
            (F::ONE + F::ONE) * self.vector.y * self.vector.z
                + (F::ONE + F::ONE) * self.scalar * self.vector.x,
            self.scalar * self.scalar
                - self.vector.x * self.vector.x
                - self.vector.y * self.vector.y
                + self.vector.z * self.vector.z,
        )
    }
}

impl<F: Float> From<(Vector3<F>, F)> for Quaternion<F> {
    /// Converts from axis, angle to quaternion.
    fn from((axis, angle): (Vector3<F>, F)) -> Self {
        Self::new_axis_angle(axis, angle)
    }
}

impl<F: Float> Mul for Quaternion<F> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.hamilton_product(&rhs)
    }
}

impl<F: Float> MulAssign for Quaternion<F> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.hamilton_product(&rhs);
    }
}

impl<F: Float> Mul<F> for Quaternion<F> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: F) -> Self::Output {
        Self {
            scalar: self.scalar * rhs,
            vector: self.vector * rhs,
        }
    }
}

impl<F: Float> MulAssign<F> for Quaternion<F> {
    #[inline]
    fn mul_assign(&mut self, rhs: F) {
        self.scalar *= rhs;
        self.vector *= rhs;
    }
}

impl<F: Float> Div<F> for Quaternion<F> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: F) -> Self::Output {
        Self {
            scalar: self.scalar / rhs,
            vector: self.vector / rhs,
        }
    }
}

impl<F: Float> DivAssign<F> for Quaternion<F> {
    #[inline]
    fn div_assign(&mut self, rhs: F) {
        self.scalar /= rhs;
        self.vector /= rhs;
    }
}

impl<F: Float> Add for Quaternion<F> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            scalar: self.scalar + rhs.scalar,
            vector: self.vector + rhs.vector,
        }
    }
}

impl<F: Float> AddAssign for Quaternion<F> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.scalar += rhs.scalar;
        self.vector += rhs.vector;
    }
}

impl<F: Float> Sub for Quaternion<F> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            scalar: self.scalar - rhs.scalar,
            vector: self.vector - rhs.vector,
        }
    }
}

impl<F: Float> SubAssign for Quaternion<F> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.scalar -= rhs.scalar;
        self.vector -= rhs.vector;
    }
}

impl<F: Float> fmt::Debug for Quaternion<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (axis, angle) = self.into_axis_angle();

        f.debug_struct("Quaternion")
            .field("axix", &axis)
            .field("angle", &angle.to_degrees())
            .finish()
    }
}
