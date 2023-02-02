#![doc = include_str!("../README.md")]

mod codegen;
mod macros;

mod matrix;
pub use matrix::*;
mod quaternion;
pub use quaternion::*;
mod vector;
pub use vector::*;
mod complex;
pub use complex::*;
mod euler;
pub use euler::*;
mod angle;
pub use angle::*;
mod float;
pub use float::*;

/// Type of float used.
#[cfg(not(feature = "use-f64"))]
pub type FloatType = f32;

/// Type of float used.
#[cfg(feature = "use-f64")]
pub type FloatType = f64;

/// Number `PI`.
#[cfg(not(feature = "use-f64"))]
pub const PI: f32 = core::f32::consts::PI;

/// Number `PI`.
#[cfg(feature = "use-f64")]
pub const PI: f64 = core::f64::consts::PI;
