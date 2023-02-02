use crate::{Float, Quaternion, Vector2, Vector4};

/// 3 Dimensional vector.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vector3<F: Float> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Float> Vector3<F> {
    /// Creates new vector from `x`, `y`, `z` components.
    #[inline]
    pub const fn new(x: F, y: F, z: F) -> Self {
        Self { x, y, z }
    }

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

    /// Computes dot product between two vectors.
    #[inline]
    pub fn dot(&self, other: Self) -> F {
        self.x * other.x + self.y * other.y + self.z * other.z
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
}

unsafe impl<F: Float> bytemuck::Pod for Vector3<F> {}
unsafe impl<F: Float> bytemuck::Zeroable for Vector3<F> {}
