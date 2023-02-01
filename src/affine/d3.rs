use crate::{Matrix3, Matrix4, Quaternion, Vector3};

/// Affine transformation in 3D space.
#[derive(Default, Clone, Copy)]
pub struct Affine3 {
    pub translation: Vector3,
    pub matrix: Matrix3,
}

impl Affine3 {
    #[inline]
    pub fn new_scale_rotation_translation(
        scale: Vector3,
        rotation: Quaternion,
        translation: Vector3,
    ) -> Self {
        Self {
            translation,
            matrix: rotation.into_matrix3() * Matrix3::new_scale(scale),
        }
    }

    #[inline]
    pub fn new_rotation_translation(rotation: Quaternion, translation: Vector3) -> Self {
        Self {
            translation,
            matrix: rotation.into_matrix3(),
        }
    }

    #[inline]
    pub fn new_scale_rotation(scale: Vector3, rotation: Quaternion) -> Self {
        Self {
            translation: Vector3::ZERO,
            matrix: rotation.into_matrix3() * Matrix3::new_scale(scale),
        }
    }

    #[inline]
    pub fn new_scale_translation(scale: Vector3, translation: Vector3) -> Self {
        Self {
            translation,
            matrix: Matrix3::new_scale(scale),
        }
    }

    #[inline]
    pub fn new_translation(translation: Vector3) -> Self {
        Self {
            translation,
            matrix: Matrix3::IDENTITY,
        }
    }

    #[inline]
    pub fn new_rotation(rotation: Quaternion) -> Self {
        Self {
            translation: Vector3::ZERO,
            matrix: Matrix3::from_quaternion(rotation),
        }
    }

    #[inline]
    pub fn new_scale(scale: Vector3) -> Self {
        Self {
            translation: Vector3::ZERO,
            matrix: Matrix3::new_diagonal(scale),
        }
    }

    #[inline]
    pub fn into_matrix4(&self) -> Matrix4 {
        Matrix4::new_translation(self.translation) * self.matrix.expand()
    }

    pub fn apply(&self, vector: Vector3) -> Vector3 {
        (self.matrix * vector) + self.translation
    }
}
