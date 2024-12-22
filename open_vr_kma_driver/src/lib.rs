use std::ffi::{c_char, c_void};

use cgmath::{Quaternion, Vector3};

#[allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    dead_code,
    unused_imports
)]
mod cpp {
    include!(concat!(env!("OUT_DIR"), "/cpp_bindings.rs"));
}
#[allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    dead_code,
    unused_imports
)]
mod vr {
    include!(concat!(env!("OUT_DIR"), "/openvr_driver_bindings.rs"));
}

#[no_mangle]
pub unsafe extern "C" fn HmdDriverFactory(
    p_interface_name: *const c_char,
    p_return_code: *mut i32,
) -> *mut c_void {
    println!("HmdDriverFactory:KawaiiMoveAssistDriver");
    unsafe {
        cpp::CallRustHandleDevicePoseUpdated = Some(raw_handle_pose_updated);
    }
    return cpp::CppOpenVREntryPoint(p_interface_name, p_return_code);
}

pub unsafe extern "C" fn raw_handle_pose_updated(
    open_vr_id: ::std::os::raw::c_int,
    pose: *mut ::std::os::raw::c_void,
) {
    handle_pose_updated(
        open_vr_id as i32,
        (pose as *mut vr::vr_DriverPose_t).as_mut().unwrap(),
    )
}

fn handle_pose_updated(open_vr_id: i32, pose: &mut vr::vr_DriverPose_t) {
    let mut device_pose = DevicePose::read_from_vr_driver_pose(pose);
    DevicePose::reset_world_from_driver_pose(pose);

    device_pose.position.x += 5f64;

    device_pose.write_vr_driver_pose(pose);
}

struct DevicePose {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
    acceleration: Vector3<f64>,
    rotation: Quaternion<f64>,
}

impl DevicePose {
    fn read_from_vr_driver_pose(pose: &mut vr::vr_DriverPose_t) -> Self {
        let mut device_pos = to_rs_vec3(&pose.vecPosition);
        let mut device_rot = to_rs_quaternion(&pose.qRotation);

        let mut device_pos_vec = to_rs_vec3(&pose.vecVelocity);
        let mut device_pos_acc = to_rs_vec3(&pose.vecAcceleration);

        let d2w_quaternion = to_rs_quaternion(&pose.qWorldFromDriverRotation);
        let d2w_translation = to_rs_vec3(&pose.vecWorldFromDriverTranslation);

        device_pos = d2w_quaternion * device_pos + d2w_translation;
        device_pos_vec = d2w_quaternion * device_pos_vec;
        device_pos_acc = d2w_quaternion * device_pos_acc;
        device_rot = d2w_quaternion * device_rot;

        DevicePose {
            position: device_pos,
            velocity: device_pos_vec,
            acceleration: device_pos_acc,
            rotation: device_rot,
        }
    }
    fn reset_world_from_driver_pose(pose: &mut vr::vr_DriverPose_t) {
        pose.qWorldFromDriverRotation = quaternion_to_vr_hmd_quaternion(QUATERNION_IDENTITY);
        pose.vecWorldFromDriverTranslation.fill(0f64);
    }

    fn write_vr_driver_pose(&mut self, pose: &mut vr::vr_DriverPose_t) {
        pose.vecPosition
            .copy_from_slice(vec3_as_slice(&self.position));
        pose.vecVelocity
            .copy_from_slice(vec3_as_slice(&self.velocity));
        pose.vecAcceleration
            .copy_from_slice(vec3_as_slice(&self.acceleration));
        pose.qRotation = quaternion_to_vr_hmd_quaternion(self.rotation);
    }
}

fn to_rs_vec3(vec: &[f64]) -> Vector3<f64> {
    Vector3 {
        x: vec[0],
        y: vec[1],
        z: vec[2],
    }
}
fn vec3_as_slice<T>(vec3: &Vector3<T>) -> &[T] {
    unsafe { std::slice::from_raw_parts(vec3 as *const Vector3<T> as *const T, 3) }
}
fn to_rs_quaternion(q: &vr::vr_HmdQuaternion_t) -> Quaternion<f64> {
    Quaternion {
        v: Vector3 {
            x: q.x,
            y: q.y,
            z: q.z,
        },
        s: q.w,
    }
}

fn quaternion_to_vr_hmd_quaternion(qua: Quaternion<f64>) -> vr::vr_HmdQuaternion_t {
    vr::vr_HmdQuaternion_t {
        w: qua.s,
        x: qua.v.x,
        y: qua.v.y,
        z: qua.v.z,
    }
}

const QUATERNION_IDENTITY: Quaternion<f64> = Quaternion {
    v: Vector3 {
        x: 0f64,
        y: 0f64,
        z: 0f64,
    },
    s: 1f64,
};
