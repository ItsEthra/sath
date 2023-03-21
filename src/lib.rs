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
mod aabb;
pub use aabb::*;
