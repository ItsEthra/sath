use crate::{Complex, Float, Matrix3, Vector2};
use std::{
    fmt::{self, Debug},
    mem::swap,
    ops::{Mul, MulAssign},
};

/// Row major 2x2 matrix.
#[derive(Clone, Copy, PartialEq)]
pub struct Matrix2<F: Float> {
    /// First row.
    pub row1: Vector2<F>,
    /// Second row.
    pub row2: Vector2<F>,
}

impl<F: Float> Matrix2<F> {
    /// Creates a new matrix from individual elements.
    pub const fn new(m11: F, m12: F, m21: F, m22: F) -> Self {
        Matrix2 {
            row1: Vector2::new(m11, m12),
            row2: Vector2::new(m21, m22),
        }
    }

    /// Creates a new matrix from diagonal vector. All other elements are equal to `0`.
    pub const fn new_diagonal(diag: Vector2<F>) -> Self {
        Self {
            row1: Vector2::new(diag.x, 0.),
            row2: Vector2::new(0., diag.y),
        }
    }

    /// Extends matrix by adding a bottom and right vectors and a corner to form 3x3 matrix.
    pub const fn extend(&self, bottom: Vector2<F>, right: Vector2<F>, corner: F) -> Matrix3<F> {
        Matrix3 {
            row1: self.row1.extend(right.x),
            row2: self.row2.extend(right.y),
            row3: bottom.extend(corner),
        }
    }

    /// Creates a matrix from individual rows.
    pub const fn from_rows(row1: Vector2<F>, row2: Vector2<F>) -> Self {
        Self { row1, row2 }
    }

    /// Creates a matrix from individual columns.
    pub const fn from_columns(col1: Vector2<F>, col2: Vector2<F>) -> Self {
        Self {
            row1: Vector2::new(col1.x, col2.x),
            row2: Vector2::new(col1.y, col2.y),
        }
    }

    /// Returns the nth row.
    /// # Panics
    /// If `n` is not 1 or 2.
    pub const fn row(&self, n: usize) -> Vector2<F> {
        match n {
            1 => self.row1,
            2 => self.row2,
            _ => panic!("Row must be either 1 or 2."),
        }
    }

    /// Sets nth row.
    /// # Panics
    /// If `n` is not 1 or 2.
    pub fn set_row(&mut self, n: usize, row: Vector2<F>) {
        match n {
            1 => self.row1 = row,
            2 => self.row2 = row,
            n => panic!("Row must be either 1 or 2. Found: {n}"),
        };
    }

    /// Returns a nth column.
    /// # Panics
    /// If `n` is not 1 or 2.
    pub const fn column(&self, n: usize) -> Vector2<F> {
        match n {
            1 => Vector2::new(self.row1.x, self.row2.x),
            2 => Vector2::new(self.row1.y, self.row2.y),
            _ => panic!("Colunm must be either 1 or 2."),
        }
    }

    /// Sets nth column.
    /// # Panics
    /// If `n` is not 1 or 2.
    pub fn set_column(&mut self, n: usize, column: Vector2<F>) {
        match n {
            1 => {
                self.row1.x = column.x;
                self.row2.x = column.x;
            }
            2 => {
                self.row1.y = column.x;
                self.row2.y = column.x;
            }
            n => panic!("Column must be either 1 or 2. Found: {n}"),
        };
    }

    /// Returns matrix's diagonal.
    pub const fn diagonal(&self) -> Vector2<F> {
        Vector2 {
            x: self.row1.x,
            y: self.row2.y,
        }
    }

    /// Sets matrix's diagonal.
    pub fn set_diagonal(&mut self, new: Vector2<F>) {
        self.row1.x = new.x;
        self.row2.y = new.y;
    }

    /// Creates a matrix that represents a 2d rotation by `angle` around origin counter-clockwise.
    #[inline]
    pub fn from_angle(angle: F) -> Self {
        Self {
            row1: Vector2::new(angle.cos(), -angle.sin()),
            row2: Vector2::new(angle.sin(), angle.cos()),
        }
    }

    /// Converts a complex number to a matrix.
    #[inline]
    pub fn from_complex(cpx: Complex<F>) -> Self {
        cpx.to_matrix2()
    }

    /// Computes the determinant of the matrix.
    #[inline]
    pub fn det(&self) -> F {
        self.row1.x * self.row2.y - self.row1.y * self.row2.x
    }

    /// Transposes matrix matrix, swapping row and columns.
    #[inline]
    pub fn transpose(&mut self) {
        swap(&mut self.row2.x, &mut self.row1.y);
    }

    /// Returns a transposed copy of the matrix.
    pub const fn transposed(self) -> Self {
        Self {
            row1: Vector2::new(self.row1.x, self.row2.x),
            row2: Vector2::new(self.row1.y, self.row2.y),
        }
    }
}

impl<F: Float> Mul for Matrix2<F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            row1: Vector2 {
                x: self.row1.dot(rhs.column(1)),
                y: self.row1.dot(rhs.column(2)),
            },
            row2: Vector2 {
                x: self.row2.dot(rhs.column(1)),
                y: self.row2.dot(rhs.column(2)),
            },
        }
    }
}

impl<F: Float> MulAssign for Matrix2<F> {
    fn mul_assign(&mut self, rhs: Self) {
        self.row1 = Vector2 {
            x: self.row1.dot(rhs.column(1)),
            y: self.row1.dot(rhs.column(2)),
        };
        self.row2 = Vector2 {
            x: self.row2.dot(rhs.column(1)),
            y: self.row2.dot(rhs.column(2)),
        };
    }
}

impl<F: Float> Mul<Vector2<F>> for Matrix2<F> {
    type Output = Vector2<F>;

    #[inline]
    fn mul(self, rhs: Vector2<F>) -> Self::Output {
        Vector2::new(self.row1.dot(rhs), self.row2.dot(rhs))
    }
}

impl<F: Float> Debug for Matrix2<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[\n\t{}\t{}\n\t{}\t{}\n]",
            self.row1.x, self.row1.y, self.row2.x, self.row2.y
        )
    }
}

impl<F: Float> Default for Matrix2<F> {
    fn default() -> Self {
        Self::IDENTITY
    }
}

unsafe impl<F: Float> bytemuck::Pod for Matrix2<F> {}
unsafe impl<F: Float> bytemuck::Zeroable for Matrix2<F> {}
