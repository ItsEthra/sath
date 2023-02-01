mod d2;
pub use d2::*;
mod d3;
pub use d3::*;
mod d4;
pub use d4::*;

/// Creates new matrix from individual elements.
/// If number of elements is `4` => Matrix2 is created.
/// If number of elements is `9` => Matrix3 is created.
/// If number of elements is `16` => Matrix4 is created.
#[macro_export]
macro_rules! matrix {
    [
        $m11:expr, $m12:expr,
        $m21:expr, $m22:expr $(,)?
    ] => {
        $crate::Matrix2::new(
            $m11 as _, $m12 as _,
            $m21 as _, $m22 as _,
        )
    };
    [
        $m11:expr, $m12:expr, $m13:expr,
        $m21:expr, $m22:expr, $m23:expr,
        $m31:expr, $m32:expr, $m33:expr $(,)?
    ] => {
        $crate::Matrix3::new(
            $m11 as _, $m12 as _, $m13 as _,
            $m21 as _, $m22 as _, $m23 as _,
            $m31 as _, $m32 as _, $m33 as _,
        )
    };
    [
        $m11:expr, $m12:expr, $m13:expr, $m14:expr,
        $m21:expr, $m22:expr, $m23:expr, $m24:expr,
        $m31:expr, $m32:expr, $m33:expr, $m34:expr,
        $m41:expr, $m42:expr, $m43:expr, $m44:expr $(,)?
    ] => {
        $crate::Matrix4::new(
            $m11 as _, $m12 as _, $m13 as _, $m14 as _,
            $m21 as _, $m22 as _, $m23 as _, $m24 as _,
            $m31 as _, $m32 as _, $m33 as _, $m34 as _,
            $m41 as _, $m42 as _, $m43 as _, $m44 as _
        )
    };
}
