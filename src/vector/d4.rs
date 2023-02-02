use crate::{Float, Vector3};
use std::cmp::Ordering;

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
    #[inline]
    pub const fn new(x: F, y: F, z: F, w: F) -> Self {
        Self { x, y, z, w }
    }

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
