#![allow(unused)]

use math::{
    aspect, matrix, rad, vector, Affine3, Euler, EulerD, Matrix2, Matrix3, Matrix4, Quaternion,
    Vector3,
};

#[rustfmt::skip]
fn main() {
    let persp = Matrix4::new_perspective_projection(rad!(90), aspect!(wh 1920. ; 1080.), 0.5, 2.);
    let (axis, angle) = Matrix3::new_rotation_zx(rad!(135), rad!(45)).to_axis_angle();
    let quat = Quaternion::new_axis_angle(axis, angle);
    dbg!(Vector3::FORWARD.rotated_by(quat));
}
