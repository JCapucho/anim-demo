use serde::{Deserialize, Serialize};
use vek::*;

type Animation = (String, String);

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub version: u32,
    pub skeletons: Vec<(SkeletonTy, Vec<Animation>)>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct Bone {
    pub offset: Vec3<f32>,
    pub ori: Quaternion<f32>,
    pub scale: Vec3<f32>,
}

impl Default for Bone {
    fn default() -> Self {
        Self {
            offset: Vec3::zero(),
            ori: Quaternion::identity(),
            scale: Vec3::broadcast(1.0 / 11.0),
        }
    }
}

impl Bone {
    pub fn compute_base_matrix(&self) -> Mat4<f32> {
        Mat4::<f32>::translation_3d(self.offset)
            * Mat4::scaling_3d(self.scale)
            * Mat4::from(self.ori)
    }

    /// Change the current bone to be more like `target`.
    fn interpolate(&mut self, target: &Bone, dt: f32) {
        // TODO: Make configurable.
        let factor = (15.0 * dt).min(1.0);
        self.offset += (target.offset - self.offset) * factor;
        self.ori = vek::Slerp::slerp(self.ori, target.ori, factor);
        self.scale += (target.scale - self.scale) * factor;
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[repr(C)]
pub enum SkeletonTy {
    Character,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct CharacterSkeleton {
    pub head: Bone,
    pub chest: Bone,
    pub belt: Bone,
    pub back: Bone,
    pub shorts: Bone,
    pub l_hand: Bone,
    pub r_hand: Bone,
    pub l_foot: Bone,
    pub r_foot: Bone,
    pub l_shoulder: Bone,
    pub r_shoulder: Bone,
    pub glider: Bone,
    pub main: Bone,
    pub second: Bone,
    pub lantern: Bone,
    pub hold: Bone,
    pub torso: Bone,
    pub control: Bone,
    pub l_control: Bone,
    pub r_control: Bone,
}

#[derive(Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct FigureBoneData {
    bone_mat: [[f32; 4]; 4],
}

impl FigureBoneData {
    pub fn new(bone_mat: vek::Mat4<f32>) -> Self {
        Self {
            bone_mat: bone_mat.into_col_arrays(),
        }
    }

    pub fn default() -> Self {
        Self::new(Mat4::identity())
    }
}

pub trait Skeleton: Send + Sync + 'static {
    type Attr;

    fn bone_count(&self) -> usize {
        16
    }

    fn compute_matrices(&self) -> ([FigureBoneData; 16], Vec3<f32>);

    /// Change the current skeleton to be more like `target`.
    fn interpolate(&mut self, target: &Self, dt: f32);
}

impl CharacterSkeleton {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Skeleton for CharacterSkeleton {
    type Attr = SkeletonAttr;

    fn bone_count(&self) -> usize {
        15
    }

    fn compute_matrices(&self) -> ([FigureBoneData; 16], Vec3<f32>) {
        let chest_mat = self.chest.compute_base_matrix();
        let torso_mat = self.torso.compute_base_matrix();
        let l_hand_mat = self.l_hand.compute_base_matrix();
        let r_hand_mat = self.r_hand.compute_base_matrix();
        let control_mat = self.control.compute_base_matrix();
        let l_control_mat = self.l_control.compute_base_matrix();
        let r_control_mat = self.r_control.compute_base_matrix();
        let main_mat = self.main.compute_base_matrix();
        let second_mat = self.second.compute_base_matrix();
        let shorts_mat = self.shorts.compute_base_matrix();
        let head_mat = self.head.compute_base_matrix();

        let lantern_final_mat =
            torso_mat * chest_mat * shorts_mat * self.lantern.compute_base_matrix();

        (
            [
                FigureBoneData::new(torso_mat * chest_mat * head_mat),
                FigureBoneData::new(torso_mat * chest_mat),
                FigureBoneData::new(torso_mat * chest_mat * self.belt.compute_base_matrix()),
                FigureBoneData::new(torso_mat * chest_mat * self.back.compute_base_matrix()),
                FigureBoneData::new(torso_mat * chest_mat * shorts_mat),
                FigureBoneData::new(
                    torso_mat * chest_mat * control_mat * l_control_mat * l_hand_mat,
                ),
                FigureBoneData::new(
                    torso_mat * chest_mat * control_mat * r_control_mat * r_hand_mat,
                ),
                FigureBoneData::new(torso_mat * self.l_foot.compute_base_matrix()),
                FigureBoneData::new(torso_mat * self.r_foot.compute_base_matrix()),
                FigureBoneData::new(torso_mat * chest_mat * self.l_shoulder.compute_base_matrix()),
                FigureBoneData::new(torso_mat * chest_mat * self.r_shoulder.compute_base_matrix()),
                FigureBoneData::new(torso_mat * self.glider.compute_base_matrix()),
                FigureBoneData::new(torso_mat * chest_mat * control_mat * l_control_mat * main_mat),
                FigureBoneData::new(
                    torso_mat * chest_mat * control_mat * r_control_mat * second_mat,
                ),
                FigureBoneData::new(lantern_final_mat),
                FigureBoneData::new(
                    torso_mat * chest_mat * l_hand_mat * self.hold.compute_base_matrix(),
                ),
            ],
            (lantern_final_mat * Vec4::new(0.0, 0.0, 0.0, 1.0)).xyz(),
        )
    }

    fn interpolate(&mut self, target: &Self, dt: f32) {
        self.head.interpolate(&target.head, dt);
        self.chest.interpolate(&target.chest, dt);
        self.belt.interpolate(&target.belt, dt);
        self.back.interpolate(&target.back, dt);
        self.shorts.interpolate(&target.shorts, dt);
        self.l_hand.interpolate(&target.l_hand, dt);
        self.r_hand.interpolate(&target.r_hand, dt);
        self.l_foot.interpolate(&target.l_foot, dt);
        self.r_foot.interpolate(&target.r_foot, dt);
        self.l_shoulder.interpolate(&target.l_shoulder, dt);
        self.r_shoulder.interpolate(&target.r_shoulder, dt);
        self.glider.interpolate(&target.glider, dt);
        self.main.interpolate(&target.main, dt);
        self.second.interpolate(&target.second, dt);
        self.lantern.interpolate(&target.lantern, dt);
        self.hold.interpolate(&target.hold, dt);
        self.torso.interpolate(&target.torso, dt);
        self.control.interpolate(&target.control, dt);
        self.l_control.interpolate(&target.l_control, dt);
        self.r_control.interpolate(&target.r_control, dt);
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[repr(C)]
pub struct SkeletonAttr {
    pub scaler: f32,
    pub head_scale: f32,
    pub head: (f32, f32),
    pub chest: (f32, f32),
    pub belt: (f32, f32),
    pub back: (f32, f32),
    pub shorts: (f32, f32),
    pub hand: (f32, f32, f32),
    pub foot: (f32, f32, f32),
    pub shoulder: (f32, f32, f32),
    pub lantern: (f32, f32, f32),
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[repr(C)]
pub struct AnimationPassTrough<S: Skeleton, Dep: Serialize> {
    pub dependency: Dep,
    pub skeleton: S,
    pub attr: S::Attr,
    pub rate: f32,
}
