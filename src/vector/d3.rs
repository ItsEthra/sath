use crate::{vector, FloatType as F, Quaternion, Vector2, Vector4};
use std::cmp::Ordering;

/// 3 Dimensional vector.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vector3 {
    pub x: F,
    pub y: F,
    pub z: F,
}

/// Constants with respective elements set to `-1`.
impl Vector3 {
    pub const X: Self = vector!(1, 0, 0);
    pub const Y: Self = vector!(0, 1, 0);
    pub const Z: Self = vector!(0, 0, 1);

    pub const XY: Self = vector!(1, 1, 0);
    pub const YZ: Self = vector!(0, 1, 1);
    pub const XZ: Self = vector!(1, 0, 1);
    pub const XYZ: Self = vector!(1, 1, 1);
}

impl Vector3 {
    /// (0, 0, 0)
    pub const ZERO: Self = vector!(0, 0, 0);
    /// (1, 1, 1)
    pub const ONE: Self = vector!(1, 1, 1);

    /// Right: (1, 0, 0)
    pub const RIGHT: Self = vector!(1, 0, 0);
    /// Left: (-1, 0, 0)
    pub const LEFT: Self = vector!(-1, 0, 0);

    /// Forward: (0, 1, 0)
    pub const FORWARD: Self = vector!(0, 1, 0);
    /// Back: (0, -1, 0)
    pub const BACK: Self = vector!(0, -1, 0);

    /// Up: (0, 0, 1)
    pub const UP: Self = vector!(0, 0, 1);
    /// Down: (0, 0, -1)
    pub const DOWN: Self = vector!(0, 0, -1);
}

impl Vector3 {
    /// Extends the vector with `w` component to create a [`Vector4`].
    pub const fn extend(self, w: F) -> Vector4 {
        Vector4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w,
        }
    }

    /// Truncates vector to [`Vector2`], removing `z` component.
    pub const fn truncate(self) -> Vector2 {
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
    pub fn rotate_by(&mut self, rotation: Quaternion) {
        *self = rotation
            .hamilton_product(&Quaternion::from_vector(*self))
            .hamilton_product(&rotation.reciprocal())
            .vector;
    }

    /// Returns a rotated copy of the vector by a rotation specified by `rotation` quaternion.
    #[inline]
    pub fn rotated_by(self, rotation: Quaternion) -> Self {
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

    /// Reflects a vector off the plane defined by its normal, resulting vector is coplanar to
    /// other two.
    #[inline]
    pub fn reflect(&self, normal: Self) -> Self {
        *self - self.projected_onto(normal) * 2.
    }
}

unsafe impl bytemuck::Pod for Vector3 {}
unsafe impl bytemuck::Zeroable for Vector3 {}

crate::__impl_vec_ops!(Vector3, 2, x, y, z);
crate::__impl_planar_ops!(Vector3, [x, 0, F], [y, 1, F], [z, 2, F]);
