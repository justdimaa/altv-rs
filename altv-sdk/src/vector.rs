use crate::natives::*;
use std::fmt;

#[derive(Copy, Clone, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

impl fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector3: [x: {}, y: {}, z: {}]", self.x, self.y, self.z)
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector3: [x: {}, y: {}, z: {}]", self.x, self.y, self.z)
    }
}

impl From<alt_Vector_float_3_VectorLayout_float_3> for Vector3 {
    fn from(_v: alt_Vector_float_3_VectorLayout_float_3) -> Self {
        // Vector3::new((*v.elements), v.y, v.z)
        Vector3::zero()
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

impl From<alt_RotationLayout> for Vector3 {
    fn from(v: alt_RotationLayout) -> Self {
        Vector3::new(v.roll, v.pitch, v.yaw)
    }
}

impl From<Vector3> for alt_RotationLayout {
    fn from(v: Vector3) -> Self {
        alt_RotationLayout {
            roll: v.x,
            pitch: v.y,
            yaw: v.z,
        }
    }
}
