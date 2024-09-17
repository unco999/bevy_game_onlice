use bevy::{app::*, input::ButtonInput, math::{Quat, Vec3}, prelude::{KeyCode, Query, Res, Transform, With}, time::Time};
use bevy_base::structs::{comp::{Marker, TimePass}, const_base, const_time};
use bevy_entity_state::structs::{comp::MainState, const_creature_state};
use bevy_rapier3d::prelude::*;
pub struct ControllerPlugin;

impl Plugin for ControllerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, key_code_input_controller);
    }
}

pub fn key_code_input_controller(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut controllers: Query<(&mut KinematicCharacterController,&mut Transform,Option<&MainState<{const_creature_state::run}>>,Option<&MainState<{const_creature_state::walk}>>,&TimePass::<{const_time::state_timer}>),(With<Marker::<{const_base::local_player}>>)>
){

    if (input.pressed(KeyCode::KeyW)) {
        let (mut controller,mut transform,is_run,is_walk,time_pass) = controllers.single_mut();
            let forward = transform.forward();
            let mut base_speed = 1.0;
            if(is_run.is_some()){
                base_speed = 1.4;
            }
            if(is_walk.is_some()){
                base_speed = 1.0;
            }
            controller.translation = Some(-forward.normalize() * time.delta_seconds() * std::f32::consts::PI * time_pass.elapsed_time * base_speed);
    }
    if (input.pressed(KeyCode::KeyS)) {
        let (mut controller,mut transform,is_run,is_walk,time_pass) = controllers.single_mut();
        let forward = transform.forward();
        controller.translation = Some(forward.normalize() * time.delta_seconds() * std::f32::consts::PI * time_pass.elapsed_time );
    }
    if (input.pressed(KeyCode::KeyA)) {
        let (mut controller,mut transform,is_run,is_walk,time_pass) = controllers.single_mut();
        let rotation_speed = time.delta_seconds() * std::f32::consts::PI * 2.0; // 每秒 45 度旋转
        transform.rotate(Quat::from_rotation_y(rotation_speed));
    }
    if (input.pressed(KeyCode::KeyD)) {
        let (mut controller,mut transform,is_run,is_walk,time_pass) = controllers.single_mut();
        let rotation_speed = time.delta_seconds() * std::f32::consts::PI* 2.0; // 每秒 45 度旋转
        transform.rotate(Quat::from_rotation_y(-rotation_speed));        
    }
}