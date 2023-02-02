use crate::{Float, Quaternion, Vector2, Vector4};
use std::cmp::Ordering;

/// Single precession Vector3.
pub type Vector3f = Vector3<f32>;
/// Double precession Vector3.
pub type Vector3d = Vector3<f64>;

/// 3 Dimensional vector.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vector3<F: Float> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Float> Vector3<F> {
    pub const ZERO: Self = Self::new(F::ZERO, F::ZERO, F::ZERO);
    pub const ONE: Self = Self::new(F::ONE, F::ONE, F::ONE);

    pub const X: Self = Self::new(F::ONE, F::ZERO, F::ZERO);
    pub const Y: Self = Self::new(F::ZERO, F::ONE, F::ZERO);
    pub const Z: Self = Self::new(F::ZERO, F::ZERO, F::ONE);

    pub const XY: Self = Self::new(F::ONE, F::ONE, F::ZERO);
    pub const YZ: Self = Self::new(F::ZERO, F::ONE, F::ONE);
    pub const XZ: Self = Self::new(F::ONE, F::ZERO, F::ONE);
    pub const XYZ: Self = Self::new(F::ONE, F::ONE, F::ONE);
}

impl<F: Float> Vector3<F> {
    /// Extends the vector with `w` component to create a [`Vector4`].
    pub const fn extend(self, w: F) -> Vector4<F> {
        Vector4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w,
        }
    }

    /// Truncates vector to [`Vector2`], removing `z` component.
    pub const fn truncate(self) -> Vector2<F> {
        Vector2 {
            x: self.x,
            y: self.y,
        }
    }

    /// Computes cross product between two vectors.
    /// Cross product is a vector which is perpendicular to both `self` and `other`.
    #[inline]
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Computes triple product between three vectors.
    /// Triple product is a signed volume of a parallelopiped formed by three vectors.
    #[inline]
    pub fn triple(&self, b: Self, c: Self) -> F {
        self.dot(b.cross(c))
    }

    /// Rotates the vector by a rotation specified by `rotation` quaternion.
    #[inline]
    pub fn rotate_by(&mut self, rotation: Quaternion<F>) {
        *self = rotation
            .hamilton_product(&Quaternion::from_vector(*self))
            .hamilton_product(&rotation.reciprocal())
            .vector;
    }

    /// Returns a rotated copy of the vector by a rotation specified by `rotation` quaternion.
    #[inline]
    pub fn rotated_by(self, rotation: Quaternion<F>) -> Self {
        rotation
            .hamilton_product(&Quaternion::from_vector(self))
            .hamilton_product(&rotation.reciprocal())
            .vector
    }

    /// Returns maximum element of the vector.
    #[inline]
    pub fn max_element(&self) -> F {
        self.x.max(self.y.max(self.z))
    }

    /// Returns minumum element of the vector.
    #[inline]
    pub fn min_element(&self) -> F {
        self.x.min(self.y.min(self.z))
    }

    /// Returns index of the maximum element.
    /// Index is in `0..=2` range.
    #[inline]
    pub fn max_index(&self) -> usize {
        [(self.x, 0), (self.y, 1), (self.z, 2)]
            .iter()
            .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(_, i)| *i)
            .unwrap()
    }

    /// Returns index of the minumum element.
    /// Index is in `0..=2` range.
    #[inline]
    pub fn min_index(&self) -> usize {
        [(self.x, 0), (self.y, 1), (self.z, 2)]
            .iter()
            .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(_, i)| *i)
            .unwrap()
    }
}

unsafe impl<F: Float> bytemuck::Pod for Vector3<F> {}
unsafe impl<F: Float> bytemuck::Zeroable for Vector3<F> {}

crate::__impl_vec_ops!(Vector3, 2, x, y, z);
crate::__impl_planar_ops!(Vector3, [x, 0, F], [y, 1, F], [z, 2, F]);
