#![no_main]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use common::{CharacterSkeleton, Metadata, SkeletonPassTrough, SkeletonTy};
use std::f32::consts::PI;
use vek::*;

static mut WASM_STAGING_BUFFER: [u8; 2048] = [0; 2048];

#[no_mangle]
extern "C" fn get_staging_buffer_ptr() -> *mut u8 {
    unsafe { WASM_STAGING_BUFFER.as_mut_ptr() }
}

#[no_mangle]
extern "C" fn metadata() -> *const u8 {
    let metadata = Metadata {
        version: 0,
        skeletons: vec![(
            SkeletonTy::Character,
            vec![("idle".to_string(), "character_idle".to_string())],
        )],
    };

    let ptr = get_staging_buffer_ptr();
    let staging = unsafe { std::slice::from_raw_parts_mut::<u8>(ptr, 2048) };
    bincode::serialize_into(staging, &metadata).unwrap();

    ptr as *const u8
}

#[no_mangle]
extern "C" fn character_idle(anim_time: f64, mut rate: f32) -> *const u8 {
    let ptr = get_staging_buffer_ptr();
    let staging = unsafe { std::slice::from_raw_parts_mut::<u8>(ptr, 2048) };
    let pass_trough =
        bincode::deserialize_from::<_, SkeletonPassTrough<CharacterSkeleton, f64>>(&*staging)
            .unwrap();

    let mut next = pass_trough.skeleton;
    let skeleton_attr = pass_trough.attr;
    let global_time = pass_trough.dependency;

    let wave_ultra_slow = (anim_time as f32 * 1.0).sin();
    let wave_ultra_slow_cos = (anim_time as f32 * 1.0 + PI / 2.0).sin();
    let head_abs = ((anim_time as f32 * 0.5 + PI).sin()) + 1.0;

    next.head.offset = Vec3::new(
        0.0,
        -2.0 + skeleton_attr.head.0,
        skeleton_attr.head.1 + wave_ultra_slow * 0.1 + head_abs * -0.5,
    );

    next.head.scale = Vec3::one() * skeleton_attr.head_scale - head_abs * 0.05;

    next.chest.offset = Vec3::new(
        0.0,
        skeleton_attr.chest.0,
        skeleton_attr.chest.1 + wave_ultra_slow * 0.1,
    );
    next.chest.scale = Vec3::one() + head_abs * 0.05;

    next.belt.offset = Vec3::new(
        0.0,
        skeleton_attr.belt.0,
        skeleton_attr.belt.1 + wave_ultra_slow * 0.1,
    );
    next.belt.ori = Quaternion::rotation_x(0.0);
    next.belt.scale = Vec3::one() - head_abs * 0.05;

    next.shorts.offset = Vec3::new(
        0.0,
        skeleton_attr.shorts.0,
        skeleton_attr.shorts.1 + wave_ultra_slow * 0.1,
    );
    next.shorts.ori = Quaternion::rotation_x(0.0);
    next.shorts.scale = Vec3::one();

    next.l_hand.offset = Vec3::new(
        -skeleton_attr.hand.0,
        skeleton_attr.hand.1 + wave_ultra_slow_cos * 0.15,
        skeleton_attr.hand.2 + wave_ultra_slow * 0.5,
    );

    next.l_hand.ori = Quaternion::rotation_x(0.0 + wave_ultra_slow * -0.06);
    next.l_hand.scale = Vec3::one();

    next.r_hand.offset = Vec3::new(
        skeleton_attr.hand.0,
        skeleton_attr.hand.1 + wave_ultra_slow_cos * 0.15,
        skeleton_attr.hand.2 + wave_ultra_slow * 0.5 + head_abs * -0.05,
    );
    next.r_hand.ori = Quaternion::rotation_x(0.0 + wave_ultra_slow * -0.06);
    next.r_hand.scale = Vec3::one() + head_abs * -0.05;

    next.l_foot.offset = Vec3::new(
        -skeleton_attr.foot.0,
        skeleton_attr.foot.1,
        skeleton_attr.foot.2,
    );
    next.l_foot.scale = Vec3::one();

    next.r_foot.offset = Vec3::new(
        skeleton_attr.foot.0,
        skeleton_attr.foot.1,
        skeleton_attr.foot.2,
    );
    next.r_foot.scale = Vec3::one();

    next.l_shoulder.offset = Vec3::new(
        -skeleton_attr.shoulder.0,
        skeleton_attr.shoulder.0,
        skeleton_attr.shoulder.2,
    );
    next.l_shoulder.ori = Quaternion::rotation_x(0.0);
    next.l_shoulder.scale = (Vec3::one() + head_abs * -0.05) * 1.15;

    next.r_shoulder.offset = Vec3::new(
        skeleton_attr.shoulder.0,
        skeleton_attr.shoulder.0,
        skeleton_attr.shoulder.2,
    );
    next.r_shoulder.ori = Quaternion::rotation_x(0.0);
    next.r_shoulder.scale = (Vec3::one() + head_abs * -0.05) * 1.15;

    next.glider.scale = Vec3::one() * 0.0;

    next.main.offset = Vec3::new(-7.0, -5.0, 18.0);
    next.main.ori = Quaternion::rotation_y(2.5) * Quaternion::rotation_z(1.57);
    next.main.scale = Vec3::one() + head_abs * -0.05;

    next.second.offset = Vec3::new(0.0, 0.0, 0.0);
    next.second.scale = Vec3::one() * 0.0;

    next.lantern.offset = Vec3::new(
        skeleton_attr.lantern.0,
        skeleton_attr.lantern.1,
        skeleton_attr.lantern.2,
    );
    next.lantern.ori = Quaternion::rotation_x(0.0);
    next.lantern.scale = Vec3::one() * 0.0;

    next.torso.offset = Vec3::new(0.0, -0.2, 0.1) * skeleton_attr.scaler;
    next.torso.ori = Quaternion::rotation_x(0.0);
    next.torso.scale = Vec3::one() / 11.0 * skeleton_attr.scaler;

    next.control.offset = Vec3::new(0.0, 0.0, 0.0);
    next.control.ori = Quaternion::rotation_x(0.0);
    next.control.scale = Vec3::one();

    next.l_control.offset = Vec3::new(0.0, 0.0, 0.0);
    next.l_control.ori = Quaternion::rotation_x(0.0);
    next.l_control.scale = Vec3::one();

    next.r_control.offset = Vec3::new(0.0, 0.0, 0.0);
    next.r_control.ori = Quaternion::rotation_x(0.0);
    next.r_control.scale = Vec3::one();

    let ret = common::AnimReturn(next, rate);
    bincode::serialize_into(staging, &ret).unwrap();

    ptr
}
