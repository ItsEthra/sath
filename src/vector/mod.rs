mod d2;
pub use d2::*;
mod d3;
pub use d3::*;
mod d4;
pub use d4::*;

/// Creates new vector
/// If number of elements is `2` => Vector2 is created.
/// If number of elements is `3` => Vector3 is created.
/// If number of elements is `4` => Vector4 is created.
#[macro_export]
macro_rules! vector {
    ($x:expr, $y:expr $(,)?) => {
        $crate::Vector2::new($x as _, $y as _)
    };
    ($x:expr, $y:expr, $z:expr $(,)?) => {
        $crate::Vector3::new($x as _, $y as _, $z as _)
    };
    ($x:expr, $y:expr, $z:expr, $w:expr $(,)?) => {
        $crate::Vector4::new($x as _, $y as _, $z as _, $w as _)
    };
}
