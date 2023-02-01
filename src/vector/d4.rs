use crate::{vector, FloatType as F, Vector3};
use std::cmp::Ordering;

/// 4 Dimensional vector.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vector4 {
    pub x: F,
    pub y: F,
    pub z: F,
    pub w: F,
}

impl Vector4 {
    pub const X: Self = vector!(1, 0, 0, 0);
    pub const Y: Self = vector!(0, 1, 0, 0);
    pub const Z: Self = vector!(0, 0, 1, 0);
    pub const W: Self = vector!(0, 0, 0, 1);

    pub const XY: Self = vector!(1, 1, 0, 0);
    pub const XZ: Self = vector!(1, 0, 1, 0);
    pub const XW: Self = vector!(1, 0, 0, 1);

    pub const YZ: Self = vector!(0, 1, 1, 0);
    pub const YW: Self = vector!(0, 1, 0, 1);

    pub const ZW: Self = vector!(0, 0, 1, 1);

    pub const XYZ: Self = vector!(1, 1, 1, 0);
    pub const XYW: Self = vector!(1, 1, 0, 1);
    pub const XZW: Self = vector!(1, 0, 1, 1);
    pub const YZW: Self = vector!(0, 1, 1, 1);

    pub const XYZW: Self = vector!(1, 1, 1, 1);
}

impl Vector4 {
    /// All elements are `0`.
    pub const ZERO: Self = vector!(0, 0, 0, 0);
    /// All elements are `1`.
    pub const ONE: Self = vector!(1, 1, 1, 1);
}

impl Vector4 {
    /// Truncates vector to [`Vector3`], removing `w` component.
    pub const fn truncate(self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    /// Returns maximum element of the vector.
    #[inline]
    pub fn max_element(&self) -> F {
        self.x.max(self.y.max(self.z.max(self.w)))
    }

    /// Returns minumum element of the vector.
    #[inline]
    pub fn min_element(&self) -> F {
        self.x.min(self.y.min(self.z.min(self.w)))
    }

    /// Returns index of the maximum element.
    /// Index is in `0..=3` range.
    #[inline]
    pub fn max_index(&self) -> usize {
        [(self.x, 0), (self.y, 1), (self.z, 2), (self.w, 3)]
            .iter()
            .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(_, i)| *i)
            .unwrap()
    }

    /// Returns index of the minimum element.
    /// Index is in `0..=3` range.
    #[inline]
    pub fn min_index(&self) -> usize {
        [(self.x, 0), (self.y, 1), (self.z, 2), (self.w, 3)]
            .iter()
            .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(_, i)| *i)
            .unwrap()
    }
}

unsafe impl bytemuck::Pod for Vector4 {}
unsafe impl bytemuck::Zeroable for Vector4 {}

crate::__impl_vec_ops!(Vector4, 3, x, y, z, w);
crate::__impl_planar_ops!(Vector4, [x, 0, F], [y, 1, F], [z, 2, F], [w, 3, F]);
