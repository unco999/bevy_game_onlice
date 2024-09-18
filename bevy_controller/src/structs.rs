use bevy::{math::Vec3, prelude::{Bundle, Component}};
use bevy_base::structs::{comp::TimePass, const_time};

pub mod const_camera_view{
    pub const extreme_long_shot:usize = 2;  
}



#[derive(Component)]
pub struct CameraTarget;

#[derive(Component)]
pub struct CameraView<const camera_view:usize>;
