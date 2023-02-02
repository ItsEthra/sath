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

    /// Computes dot product
    #[inline]
    pub fn dot(&self, other: Self) -> F {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

unsafe impl<F: Float> bytemuck::Pod for Vector4<F> {}
unsafe impl<F: Float> bytemuck::Zeroable for Vector4<F> {}
