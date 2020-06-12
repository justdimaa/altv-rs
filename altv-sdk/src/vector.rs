use crate::natives::*;
use std::fmt;

pub type Vector3 = nalgebra::core::Vector3<f32>;
pub type Rotation3 = nalgebra::geometry::Rotation3<f32>;

impl From<alt_Vector_float_3_VectorLayout_float_3> for Vector3 {
    fn from(_v: alt_Vector_float_3_VectorLayout_float_3) -> Self {
        // Vector3::new((*v.elements), v.y, v.z)
        Vector3::default()
    }
}

impl From<Vector3> for alt_Vector_float_3_VectorLayout_float_3 {
    fn from(_v: Vector3) -> Self {
        alt_Vector_float_3_VectorLayout_float_3 {
            elements: std::ptr::null_mut(),
        }
    }
}

impl From<alt_Vector_float_3_PointLayout> for Vector3 {
    fn from(v: alt_Vector_float_3_PointLayout) -> Self {
        Vector3::new(v.x, v.y, v.z)
    }
}

impl From<Vector3> for alt_Vector_float_3_PointLayout {
    fn from(v: Vector3) -> Self {
        alt_Vector_float_3_PointLayout {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<alt_RotationLayout> for Rotation3 {
    fn from(v: alt_RotationLayout) -> Self {
        Rotation3::from_euler_angles(v.roll, v.pitch, v.yaw)
    }
}

impl From<Rotation3> for alt_RotationLayout {
    fn from(v: Rotation3) -> Self {
        let euler = v.euler_angles();
        alt_RotationLayout {
            roll: euler.0,
            pitch: euler.1,
            yaw: euler.2,
        }
    }
}
