use crate::{vector, FloatType as F, Matrix4, Quaternion, Vector3};
use std::{
    fmt,
    mem::swap,
    ops::{Mul, MulAssign},
};

/// Row major 3x3 matrix.
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Matrix3 {
    /// First row.
    pub row1: Vector3,
    /// Second row.
    pub row2: Vector3,
    /// Third row.
    pub row3: Vector3,
}

impl Matrix3 {
    /// Matrix with all elements equal to `0`.
    pub const ZERO: Matrix3 = Matrix3 {
        row1: Vector3::ZERO,
        row2: Vector3::ZERO,
        row3: Vector3::ZERO,
    };

    /// Matrix with all elements equal to `1`.
    pub const ONE: Matrix3 = Matrix3 {
        row1: Vector3::ONE,
        row2: Vector3::ONE,
        row3: Vector3::ONE,
    };

    /// Identity matrix with diagonal elements equal to `1` and `0` for every other.
    pub const IDENTITY: Matrix3 = Matrix3 {
        row1: Vector3::new(1., 0., 0.),
        row2: Vector3::new(0., 1., 0.),
        row3: Vector3::new(0., 0., 1.),
    };
}

impl Matrix3 {
    /// Creates a new matrix from individual elements.
    #[allow(clippy::too_many_arguments)]
    #[rustfmt::skip]
    pub const fn new(
        m11: F, m12: F, m13: F,
        m21: F, m22: F, m23: F,
        m31: F, m32: F,
        m33: F,
    ) -> Self {
        Self {
            row1: Vector3::new(m11, m12, m13),
            row2: Vector3::new(m21, m22, m23),
            row3: Vector3::new(m31, m32, m33),
        }
    }

    /// Extends matrix by adding a bottom and right vectors and a corner to form 4x4 matrix.
    pub const fn extend(&self, bottom: Vector3, right: Vector3, corner: F) -> Matrix4 {
        Matrix4 {
            row1: self.row1.extend(right.x),
            row2: self.row2.extend(right.y),
            row3: self.row3.extend(right.z),
            row4: bottom.extend(corner),
        }
    }

    /// Expands matrix by adding zero vectors to the bottom and right with `1` in the corner.
    /// Shortcut for `.extend(Vector3::ZERO, Vector3::ZERO, 1.0)`.
    pub const fn expand(&self) -> Matrix4 {
        self.extend(Vector3::ZERO, Vector3::ZERO, 1.)
    }

    /// Creates a new matrix from diagonal vector. All other elements are equal to `0`.
    pub const fn new_diagonal(diag: Vector3) -> Self {
        Self {
            row1: Vector3::new(diag.x, 0., 0.),
            row2: Vector3::new(0., diag.y, 0.),
            row3: Vector3::new(0., 0., diag.z),
        }
    }

    /// Creates a matrix from individual rows.
    pub const fn from_rows(row1: Vector3, row2: Vector3, row3: Vector3) -> Self {
        Self { row1, row2, row3 }
    }

    /// Creates a matrix from individual columns.
    pub const fn from_columns(col1: Vector3, col2: Vector3, col3: Vector3) -> Self {
        Self {
            row1: Vector3::new(col1.x, col2.x, col3.x),
            row2: Vector3::new(col1.y, col2.y, col3.y),
            row3: Vector3::new(col1.z, col2.z, col3.z),
        }
    }

    /// Creates a matrix which specifies a rotation around `X` axis.
    pub fn new_rotation_x(angle: F) -> Self {
        Self {
            row1: Vector3::X,
            row2: Vector3::new(0., angle.cos(), -angle.sin()),
            row3: Vector3::new(0., angle.sin(), angle.cos()),
        }
    }

    /// Creates a matrix which specifies a rotation around `Y` axis.
    pub fn new_rotation_y(angle: F) -> Self {
        Self {
            row1: vector!(angle.cos(), 0, angle.sin()),
            row2: Vector3::Y,
            row3: vector!(-angle.sin(), 0, angle.cos()),
        }
    }

    /// Creates a matrix which specifies a rotation around `Z` axis.
    pub fn new_rotation_z(angle: F) -> Self {
        Self {
            row1: vector!(angle.cos(), -angle.sin(), 0),
            row2: vector!(angle.sin(), angle.cos(), 0),
            row3: Vector3::Z,
        }
    }

    /// Creates a matrix which specifies a rotation around `X` and `Y` axis.
    /// Order is: `X` first, then `Y`.
    pub fn new_rotation_xy(x: F, y: F) -> Self {
        Self::new_rotation_y(y) * Self::new_rotation_x(x)
    }

    /// Creates a matrix which specifies a rotation around `X` and `Z` axis.
    /// Order is: `Z` first, then `X`.
    pub fn new_rotation_zx(z: F, x: F) -> Self {
        Self::new_rotation_x(x) * Self::new_rotation_z(z)
    }

    /// Creates a matrix which specifies a rotation around `Y` and `Z` axis.
    /// Order is: `Z` first, then `Y`.
    pub fn new_rotation_zy(z: F, y: F) -> Self {
        Self::new_rotation_y(y) * Self::new_rotation_z(z)
    }

    /// Creates a matrix which specifies a rotation around `X`, `Y` and `Z` axis.
    /// Order is: `X` first, then `X`, then `Y`.
    pub fn new_rotation_zxy(z: F, x: F, y: F) -> Self {
        Self::new_rotation_y(y) * Self::new_rotation_x(x) * Self::new_rotation_z(z)
    }

    pub const fn new_scale(scale: Vector3) -> Self {
        Self::new_diagonal(scale)
    }

    /// Extracts an axis of rotation if matrix represents a rotation.
    pub fn rotation_axis(&self) -> Vector3 {
        vector!(
            self.row3.y - self.row2.z,
            self.row1.z - self.row3.x,
            self.row2.x - self.row1.y,
        )
        .normalized()
    }

    /// Extracts an angle of rotation if matrix represents a rotation.
    pub fn rotation_angle(&self) -> F {
        ((self.trace() - 1.) / 2.).acos()
    }

    /// Extracts axis and angle of rotation if matrix represents a rotation.
    pub fn to_axis_angle(&self) -> (Vector3, F) {
        (self.rotation_axis(), self.rotation_angle())
    }

    /// Converts axis, angle representation to a rotation matrix that represents a rotation in 3d
    /// space around `axis` on `angle` in radians.
    /// To avoid unexpected results, use normalized axis.
    pub fn from_axis_angle(axis: Vector3, angle: F) -> Self {
        Self {
            row1: vector!(
                angle.cos() + axis.x * axis.x * (1. - angle.cos()),
                axis.x * axis.y * (1. - angle.cos()) - axis.z * angle.sin(),
                axis.x * axis.z * (1. - angle.cos()) + axis.y * angle.sin(),
            ),
            row2: vector!(
                axis.y * axis.x * (1. - angle.cos()) + axis.z * angle.sin(),
                angle.cos() + axis.y * axis.y * (1. - angle.cos()),
                axis.y * axis.z * (1. - angle.cos()) - axis.x * angle.sin(),
            ),
            row3: vector!(
                axis.z * axis.x * (1. - angle.cos()) - axis.y * angle.sin(),
                axis.z * axis.y * (1. - angle.cos()) + axis.x * angle.sin(),
                angle.cos() + axis.z * axis.z * (1. - angle.cos())
            ),
        }
    }

    /// Sum of diagonal elements.
    #[inline]
    pub fn trace(&self) -> F {
        self.diagonal().sum()
    }

    /// Returns the nth row.
    /// # Panics
    /// If `n` is not 1, 2 or 3.
    pub const fn row(&self, n: usize) -> Vector3 {
        match n {
            1 => self.row1,
            2 => self.row2,
            3 => self.row3,
            _ => panic!("Row must be either 1, 2 or 3"),
        }
    }

    /// Sets the nth row.
    /// # Panics
    /// If `n` is not 1, 2 or 3.
    pub fn set_row(&mut self, n: usize, row: Vector3) {
        match n {
            1 => self.row1 = row,
            2 => self.row2 = row,
            3 => self.row3 = row,
            n => panic!("Row must be either 1, 2 or 3. Found: {n}"),
        };
    }

    /// Returns the nth column.
    /// # Panics
    /// If `n` is not 1, 2 or 3.
    pub const fn column(&self, n: usize) -> Vector3 {
        match n {
            1 => Vector3::new(self.row1.x, self.row2.x, self.row3.x),
            2 => Vector3::new(self.row1.y, self.row2.y, self.row3.y),
            3 => Vector3::new(self.row1.z, self.row2.z, self.row3.z),
            _ => panic!("Colunm must be either 1, 2 or 3"),
        }
    }

    /// Sets the nth column.
    /// # Panics
    /// If `n` is not 1, 2 or 3.
    pub fn set_column(&mut self, n: usize, column: Vector3) {
        match n {
            1 => {
                self.row1.x = column.x;
                self.row2.x = column.y;
                self.row3.x = column.z;
            }
            2 => {
                self.row1.y = column.x;
                self.row2.y = column.y;
                self.row3.y = column.z;
            }
            3 => {
                self.row1.z = column.x;
                self.row2.z = column.y;
                self.row3.z = column.z;
            }
            n => panic!("Column must be either 1, 2 or 3. Found: {n}"),
        };
    }

    /// Returns matrix's diagonal.
    pub const fn diagonal(&self) -> Vector3 {
        Vector3 {
            x: self.row1.x,
            y: self.row2.y,
            z: self.row3.z,
        }
    }

    /// Sets matrix's diagonal.
    pub fn set_diagonal(&mut self, new: Vector3) {
        self.row1.x = new.x;
        self.row2.y = new.y;
        self.row3.z = new.z;
    }

    /// Transposes matrix matrix, swapping row and columns.
    pub fn transpose(&mut self) {
        swap(&mut self.row1.y, &mut self.row2.x);
        swap(&mut self.row1.z, &mut self.row3.x);
        swap(&mut self.row2.z, &mut self.row3.y);
    }

    /// Returns a transposed copy of the matrix.
    pub const fn transposed(&self) -> Self {
        Self {
            row1: Vector3::new(self.row1.x, self.row2.x, self.row3.x),
            row2: Vector3::new(self.row1.y, self.row2.y, self.row3.y),
            row3: Vector3::new(self.row1.z, self.row2.z, self.row3.z),
        }
    }

    /// Computes the determinant of the matrix.
    pub fn det(&self) -> F {
        let mut copy = *self;
        copy.to_row_echelon();

        copy.diagonal().product()
    }

    /// Converts from a quaternion to a matrix.
    /// # Warning
    /// If the quaternion represents identity rotation, extracting axis will result in `NaN` for
    /// every element of the axis vector.
    #[inline]
    pub fn from_quaternion(quat: Quaternion) -> Self {
        quat.into_matrix3()
    }
}

impl From<Quaternion> for Matrix3 {
    fn from(value: Quaternion) -> Self {
        value.into_matrix3()
    }
}

impl Mul for Matrix3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            row1: Vector3 {
                x: self.row1.dot(rhs.column(1)),
                y: self.row1.dot(rhs.column(2)),
                z: self.row1.dot(rhs.column(3)),
            },
            row2: Vector3 {
                x: self.row2.dot(rhs.column(1)),
                y: self.row2.dot(rhs.column(2)),
                z: self.row2.dot(rhs.column(3)),
            },
            row3: Vector3 {
                x: self.row3.dot(rhs.column(1)),
                y: self.row3.dot(rhs.column(2)),
                z: self.row3.dot(rhs.column(3)),
            },
        }
    }
}

impl MulAssign for Matrix3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.row1 = Vector3 {
            x: self.row1.dot(rhs.column(1)),
            y: self.row1.dot(rhs.column(2)),
            z: self.row1.dot(rhs.column(3)),
        };
        self.row2 = Vector3 {
            x: self.row2.dot(rhs.column(1)),
            y: self.row2.dot(rhs.column(2)),
            z: self.row2.dot(rhs.column(3)),
        };
        self.row3 = Vector3 {
            x: self.row3.dot(rhs.column(1)),
            y: self.row3.dot(rhs.column(2)),
            z: self.row3.dot(rhs.column(3)),
        };
    }
}

impl Mul<Vector3> for Matrix3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.row1.dot(rhs),
            y: self.row2.dot(rhs),
            z: self.row3.dot(rhs),
        }
    }
}

impl fmt::Debug for Matrix3 {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[\n\t{}\t{}\t{}\n\t{}\t{}\t{}\n\t{}\t{}\t{}\n]",
            self.row1.x, self.row1.y, self.row1.z,
            self.row2.x, self.row2.y, self.row2.z,
            self.row3.x, self.row3.y, self.row3.z
        )
    }
}

impl Default for Matrix3 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

unsafe impl bytemuck::Pod for Matrix3 {}
unsafe impl bytemuck::Zeroable for Matrix3 {}

crate::__impl_mat_ops!(Matrix3, Vector3, 3, row1, row2, row3);
