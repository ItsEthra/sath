use crate::{Float, Vector3, Vector4};
use std::{
    fmt,
    mem::swap,
    ops::{Mul, MulAssign},
};

/// Row major 4x4 matrix.
#[derive(Clone, Copy, PartialEq)]
pub struct Matrix4<F: Float> {
    pub row1: Vector4<F>,
    pub row2: Vector4<F>,
    pub row3: Vector4<F>,
    pub row4: Vector4<F>,
}

impl<F: Float> Matrix4<F> {
    pub const ZERO: Self = Self {
        row1: Vector4::ZERO,
        row2: Vector4::ZERO,
        row3: Vector4::ZERO,
        row4: Vector4::ZERO,
    };

    pub const ONE: Self = Self {
        row1: Vector4::ONE,
        row2: Vector4::ONE,
        row3: Vector4::ONE,
        row4: Vector4::ONE,
    };

    pub const IDENTITY: Self = Self {
        row1: Vector4::X,
        row2: Vector4::Y,
        row3: Vector4::Z,
        row4: Vector4::W,
    };
}

impl<F: Float> Matrix4<F> {
    #[rustfmt::skip]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        m11: F, m12: F, m13: F, m14: F,
        m21: F, m22: F, m23: F, m24: F,
        m31: F, m32: F, m33: F, m34: F,
        m41: F, m42: F, m43: F, m44: F,
    ) -> Self {
        Self {
            row1: Vector4::new(m11, m12, m13, m14),
            row2: Vector4::new(m21, m22, m23, m24),
            row3: Vector4::new(m31, m32, m33, m34),
            row4: Vector4::new(m41, m42, m43, m44),
        }
    }

    /// Creates new matrix where diagonal entries set to the elements of the vector.
    pub const fn new_diagonal(diag: Vector4<F>) -> Self {
        Self {
            row1: Vector4::new(diag.x, F::ZERO, F::ZERO, F::ZERO),
            row2: Vector4::new(F::ZERO, diag.y, F::ZERO, F::ZERO),
            row3: Vector4::new(F::ZERO, F::ZERO, diag.z, F::ZERO),
            row4: Vector4::new(F::ZERO, F::ZERO, F::ZERO, diag.w),
        }
    }

    /// Creates new matrix that represents translation in 3D space.
    pub const fn new_translation(translation: Vector3<F>) -> Self {
        let mut m = Self::IDENTITY;
        m.row1.w = translation.x;
        m.row2.w = translation.y;
        m.row3.w = translation.z;
        m
    }

    /// Creates a matrix from individual rows.
    pub const fn from_rows(
        row1: Vector4<F>,
        row2: Vector4<F>,
        row3: Vector4<F>,
        row4: Vector4<F>,
    ) -> Self {
        Self {
            row1,
            row2,
            row3,
            row4,
        }
    }

    /// Creates a matrix from individual columns.
    pub const fn from_columns(
        col1: Vector4<F>,
        col2: Vector4<F>,
        col3: Vector4<F>,
        col4: Vector4<F>,
    ) -> Self {
        Self {
            row1: Vector4::new(col1.x, col2.x, col3.x, col4.x),
            row2: Vector4::new(col1.y, col2.y, col3.y, col4.y),
            row3: Vector4::new(col1.z, col2.z, col3.z, col4.z),
            row4: Vector4::new(col1.w, col2.w, col3.w, col4.w),
        }
    }

    /// Returns the nth row.
    /// # Panics
    /// If `n` is not 1, 2, 3 or 4.
    pub const fn row(&self, n: usize) -> Vector4<F> {
        match n {
            1 => self.row1,
            2 => self.row2,
            3 => self.row3,
            4 => self.row4,
            _ => panic!("Row must be either 1, 2, 3 or 4"),
        }
    }

    /// Sets the nth row.
    /// # Panics
    /// If `n` is not 1, 2, 3 or 4.
    pub fn set_row(&mut self, n: usize, row: Vector4<F>) {
        match n {
            1 => self.row1 = row,
            2 => self.row2 = row,
            3 => self.row3 = row,
            4 => self.row4 = row,
            n => panic!("Row must be either 1, 2, 3 or 4. Found: {n}"),
        };
    }

    /// Returns the nth column.
    /// # Panics
    /// If `n` is not 1, 2, 3 or 4.
    pub const fn column(&self, n: usize) -> Vector4<F> {
        match n {
            1 => Vector4::new(self.row1.x, self.row2.x, self.row3.x, self.row4.x),
            2 => Vector4::new(self.row1.y, self.row2.y, self.row3.y, self.row4.y),
            3 => Vector4::new(self.row1.z, self.row2.z, self.row3.z, self.row4.z),
            4 => Vector4::new(self.row1.w, self.row2.w, self.row3.w, self.row4.w),
            _ => panic!("Colunm must be either 1, 2 or 3"),
        }
    }

    /// Sets the nth column.
    /// # Panics
    /// If `n` is not 1, 2, 3 or 4.
    pub fn set_column(&mut self, n: usize, column: Vector4<F>) {
        match n {
            1 => {
                self.row1.x = column.x;
                self.row2.x = column.y;
                self.row3.x = column.z;
                self.row4.x = column.w;
            }
            2 => {
                self.row1.y = column.x;
                self.row2.y = column.y;
                self.row3.y = column.z;
                self.row4.y = column.w;
            }
            3 => {
                self.row1.z = column.x;
                self.row2.z = column.y;
                self.row3.z = column.z;
                self.row4.z = column.w;
            }
            4 => {
                self.row1.w = column.x;
                self.row2.w = column.y;
                self.row3.w = column.z;
                self.row4.w = column.w;
            }
            n => panic!("Column must be either 1, 2, 3 or 4. Found: {n}"),
        };
    }

    /// Returns matrix's diagonal.
    pub const fn diagonal(&self) -> Vector4<F> {
        Vector4 {
            x: self.row1.x,
            y: self.row2.y,
            z: self.row3.z,
            w: self.row4.w,
        }
    }

    /// Sets matrix's diagonal.
    pub fn set_diagonal(&mut self, new: Vector4<F>) {
        self.row1.x = new.x;
        self.row2.y = new.y;
        self.row3.z = new.z;
        self.row4.w = new.w;
    }

    /// Transposes matrix matrix, swapping row and columns.
    pub fn transpose(&mut self) {
        swap(&mut self.row1.y, &mut self.row2.x);
        swap(&mut self.row1.z, &mut self.row3.x);
        swap(&mut self.row1.w, &mut self.row4.x);

        swap(&mut self.row2.z, &mut self.row3.y);
        swap(&mut self.row2.w, &mut self.row4.y);

        swap(&mut self.row3.w, &mut self.row4.z);
    }

    /// Returns a transposed copy of the matrix.
    pub const fn transposed(&self) -> Self {
        Self::from_columns(self.row1, self.row2, self.row3, self.row4)
    }

    /// Computes the determinant of the matrix.
    pub fn det(&self) -> F {
        let mut copy = *self;
        copy.to_row_echelon();

        copy.diagonal().product()
    }
}

impl<F: Float> Mul for Matrix4<F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix4 {
            row1: Vector4 {
                x: self.row1.dot(rhs.column(1)),
                y: self.row1.dot(rhs.column(2)),
                z: self.row1.dot(rhs.column(3)),
                w: self.row1.dot(rhs.column(4)),
            },
            row2: Vector4 {
                x: self.row2.dot(rhs.column(1)),
                y: self.row2.dot(rhs.column(2)),
                z: self.row2.dot(rhs.column(3)),
                w: self.row2.dot(rhs.column(4)),
            },
            row3: Vector4 {
                x: self.row3.dot(rhs.column(1)),
                y: self.row3.dot(rhs.column(2)),
                z: self.row3.dot(rhs.column(3)),
                w: self.row3.dot(rhs.column(4)),
            },
            row4: Vector4 {
                x: self.row4.dot(rhs.column(1)),
                y: self.row4.dot(rhs.column(2)),
                z: self.row4.dot(rhs.column(3)),
                w: self.row4.dot(rhs.column(4)),
            },
        }
    }
}

impl<F: Float> MulAssign for Matrix4<F> {
    fn mul_assign(&mut self, rhs: Self) {
        self.row1 = Vector4 {
            x: self.row1.dot(rhs.column(1)),
            y: self.row1.dot(rhs.column(2)),
            z: self.row1.dot(rhs.column(3)),
            w: self.row1.dot(rhs.column(4)),
        };
        self.row2 = Vector4 {
            x: self.row2.dot(rhs.column(1)),
            y: self.row2.dot(rhs.column(2)),
            z: self.row2.dot(rhs.column(3)),
            w: self.row2.dot(rhs.column(4)),
        };
        self.row3 = Vector4 {
            x: self.row3.dot(rhs.column(1)),
            y: self.row3.dot(rhs.column(2)),
            z: self.row3.dot(rhs.column(3)),
            w: self.row3.dot(rhs.column(4)),
        };
        self.row4 = Vector4 {
            x: self.row4.dot(rhs.column(1)),
            y: self.row4.dot(rhs.column(2)),
            z: self.row4.dot(rhs.column(3)),
            w: self.row4.dot(rhs.column(4)),
        };
    }
}

impl<F: Float> Mul<Vector4<F>> for Matrix4<F> {
    type Output = Vector4<F>;

    fn mul(self, rhs: Vector4<F>) -> Self::Output {
        Vector4 {
            x: self.row1.dot(rhs),
            y: self.row2.dot(rhs),
            z: self.row3.dot(rhs),
            w: self.row4.dot(rhs),
        }
    }
}

impl<F: Float> fmt::Debug for Matrix4<F> {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[\n\t{}\t{}\t{}\t{}\n\t{}\t{}\t{}\t{}\n\t{}\t{}\t{}\t{}\n\t{}\t{}\t{}\t{}\n]",
            self.row1.x, self.row1.y, self.row1.z, self.row1.w,
            self.row2.x, self.row2.y, self.row2.z, self.row2.w,
            self.row3.x, self.row3.y, self.row3.z, self.row3.w,
            self.row4.x, self.row4.y, self.row4.z, self.row4.w,
        )
    }
}

unsafe impl<F: Float> bytemuck::Pod for Matrix4<F> {}
unsafe impl<F: Float> bytemuck::Zeroable for Matrix4<F> {}

crate::__impl_mat_ops!(Matrix4, Vector4, 4, row1, row2, row3, row4);
