/// Radians marker type.
#[derive(Debug, Clone, Copy)]
pub struct Rad;

/// Degrees marker type.
#[derive(Debug, Clone, Copy)]
pub struct Deg;

mod private {
    pub trait Sealed {}
    impl Sealed for super::Rad {}
    impl Sealed for super::Deg {}
}

/// Helper trait to distinguish between radians and degrees.
pub trait Angle: private::Sealed {}

impl Angle for Rad {}
impl Angle for Deg {}
