use glam::{Quat, Vec3};

use crate::{Fov, Pose, ViewParams};

#[repr(C)]
pub struct NanvrFov {
    /// Negative, radians
    pub left: f32,
    /// Positive, radians
    pub right: f32,
    /// Positive, radians
    pub up: f32,
    /// Negative, radians
    pub down: f32,
}

#[repr(C)]
#[derive(Clone, Default)]
pub struct NanvrQuat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Clone, Default)]
pub struct NanvrPose {
    pub orientation: NanvrQuat,
    pub position: [f32; 3],
}

#[repr(C)]
pub struct NanvrViewParams {
    pub pose: NanvrPose,
    pub fov: NanvrFov,
}

#[repr(u8)]
pub enum NanvrCodecType {
    H264 = 0,
    Hevc = 1,
    AV1 = 2,
}

pub fn to_capi_fov(fov: &Fov) -> NanvrFov {
    NanvrFov {
        left: fov.left,
        right: fov.right,
        up: fov.up,
        down: fov.down,
    }
}

pub fn from_capi_fov(fov: &NanvrFov) -> Fov {
    Fov {
        left: fov.left,
        right: fov.right,
        up: fov.up,
        down: fov.down,
    }
}

pub fn from_capi_quat(quat: &NanvrQuat) -> Quat {
    Quat::from_xyzw(quat.x, quat.y, quat.z, quat.w)
}

pub fn to_capi_quat(quat: &Quat) -> NanvrQuat {
    NanvrQuat {
        x: quat.x,
        y: quat.y,
        z: quat.z,
        w: quat.w,
    }
}

pub fn to_capi_pose(pose: &Pose) -> NanvrPose {
    NanvrPose {
        orientation: to_capi_quat(&pose.orientation),
        position: pose.position.to_array(),
    }
}

pub fn from_capi_pose(pose: &NanvrPose) -> Pose {
    Pose {
        orientation: from_capi_quat(&pose.orientation),
        position: Vec3::from_slice(&pose.position),
    }
}

pub fn to_capi_view_params(view_params: &ViewParams) -> NanvrViewParams {
    NanvrViewParams {
        pose: to_capi_pose(&view_params.pose),
        fov: to_capi_fov(&view_params.fov),
    }
}

pub fn from_capi_view_params(view_params: &NanvrViewParams) -> ViewParams {
    ViewParams {
        pose: from_capi_pose(&view_params.pose),
        fov: from_capi_fov(&view_params.fov),
    }
}
