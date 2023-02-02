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
    /// Truncates vector to [`Vector3`], removing `w` component.
    pub const fn truncate(self) -> Vector3<F> {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

unsafe impl<F: Float> bytemuck::Pod for Vector4<F> {}
unsafe impl<F: Float> bytemuck::Zeroable for Vector4<F> {}

crate::__impl_vec_ops!(Vector4, 3, x, y, z, w);
crate::__impl_planar_ops!(Vector4, [x, 0, F], [y, 1, F], [z, 2, F], [w, 3, F]);
