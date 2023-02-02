use std::cmp::Ordering;

use crate::{Float, Vector3};

/// 4 Dimensional vector.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vector4<F: Float> {
    pub x: F,
    pub y: F,
    pub z: F,
    pub w: F,
}

impl<F: Float> Vector4<F> {
    pub const ZERO: Self = Self::new(F::ZERO, F::ZERO, F::ZERO, F::ZERO);
    pub const ONE: Self = Self::new(F::ONE, F::ONE, F::ONE, F::ONE);

    pub const X: Self = Self::new(F::ONE, F::ZERO, F::ZERO, F::ZERO);
    pub const Y: Self = Self::new(F::ZERO, F::ONE, F::ZERO, F::ZERO);
    pub const Z: Self = Self::new(F::ZERO, F::ZERO, F::ONE, F::ZERO);
    pub const W: Self = Self::new(F::ZERO, F::ZERO, F::ZERO, F::ONE);

    pub const XY: Self = Self::new(F::ONE, F::ONE, F::ZERO, F::ZERO);
    pub const XZ: Self = Self::new(F::ONE, F::ZERO, F::ONE, F::ZERO);
    pub const XW: Self = Self::new(F::ONE, F::ZERO, F::ZERO, F::ONE);

    pub const YZ: Self = Self::new(F::ZERO, F::ONE, F::ONE, F::ZERO);
    pub const YW: Self = Self::new(F::ZERO, F::ONE, F::ZERO, F::ONE);

    pub const ZW: Self = Self::new(F::ZERO, F::ZERO, F::ONE, F::ONE);

    pub const XYZ: Self = Self::new(F::ONE, F::ONE, F::ONE, F::ZERO);
    pub const XYW: Self = Self::new(F::ONE, F::ONE, F::ZERO, F::ONE);
    pub const XZW: Self = Self::new(F::ONE, F::ZERO, F::ONE, F::ONE);
    pub const YZW: Self = Self::new(F::ZERO, F::ONE, F::ONE, F::ONE);

    pub const XYZW: Self = Self::new(F::ONE, F::ONE, F::ONE, F::ONE);
}

impl<F: Float> Vector4<F> {
    /// Truncates vector to [`Vector3`], removing `w` component.
    pub const fn truncate(self) -> Vector3<F> {
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

unsafe impl<F: Float> bytemuck::Pod for Vector4<F> {}
unsafe impl<F: Float> bytemuck::Zeroable for Vector4<F> {}

crate::__impl_vec_ops!(Vector4, 3, x, y, z, w);
crate::__impl_planar_ops!(Vector4, [x, 0, F], [y, 1, F], [z, 2, F], [w, 3, F]);
