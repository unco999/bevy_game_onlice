use std::{process::Command, time::Duration};

use bevy::{
    a11y::accesskit::Vec2, app::*, color::Color, input::{mouse::MouseMotion, ButtonInput}, math::{vec3, Quat, Vec2Swizzles, Vec3, Vec3Swizzles}, prelude::{
        in_state, Added, Camera, Changed, Commands, Entity, EventReader, Gizmos, IntoSystemConfigs, KeyCode, Local, MouseButton, OnEnter, ParamSet, Query, Res, SystemSet, Transform, With, Without
    }, time::{common_conditions::on_timer, Time}, window::CursorMoved
};
use bevy_base::structs::{
    comp::{Link, Marker, TimePass, TransformNote, TransformOffset}, const_base, const_link_type, const_time, const_transfrom_note, AppState
};
use bevy_debug::DebugSys;
use bevy_entity_state::structs::{comp::MainState, const_creature_state, special_aniamtion::ClimbUpWapll};
use bevy_mask_system::MaskSys;
use bevy_rapier3d::{na::clamp, prelude::*};
use mask_system_lib::*;

use crate::structs::{const_camera_view, CameraTarget, CameraView};
pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (key_code_input_controller,key_code_input_climb_up_wapll))
            .add_systems(
                FixedUpdate,
                F::<op!(FollowCameraTarget), MainCamera, Content<0, 0, 0, 0, 0, 0, 0>>::sign()
                    .in_set(CemeraSystemSet::FolLow1).run_if(in_state(AppState::GameStart)),
            )
            .add_systems(
                FixedUpdate,
                F::<op!(CameraOffset), MainCamera, Content<0, 0, 0, 0, 0, 0, 0>>::sign()
                .in_set(CemeraSystemSet::Follow2).after(CemeraSystemSet::FolLow1).run_if(in_state(AppState::GameStart)),
            )
            .add_systems(FixedUpdate,F::<op!(CameraTargetTick),MainCamera,Content::<0,0,0,0,0,0,0>>::sign())
            .add_systems(
                Update,
                F::<localPlayerUpdate, MainCamera, Content<0, 0, 0, 0, 0, 0, 0>>::sign().after(DebugSys::Init).run_if(on_timer(Duration::from_secs_f32(1.6))),
            )
            ;
    }
}


#[derive(SystemSet,Hash,Debug,PartialEq,Eq,Clone)]
enum CemeraSystemSet{
    FolLow1,
    Follow2,
}

pub fn key_code_input_climb_up_wapll(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut controllers: Query<
        (
            &mut KinematicCharacterController,
            &mut Transform,
            &TimePass<{ const_time::state_timer }>,
        ),
        (With<Marker<{ const_base::local_player }>>,With<ClimbUpWapll>),
    >,
) {
    if (input.pressed(KeyCode::KeyW)) {
        if(controllers.is_empty()) {return}
        let (mut controller, mut transform, time_pass) = controllers.single_mut();
        let forward = transform.up();
        let mut base_speed = 0.25;
        controller.translation = Some(
            forward
                // * time.delta_seconds()
                * std::f32::consts::PI
                * time_pass.elapsed_time
                * base_speed,
        );
        println!("采用向上的力量")
    }
    if (input.pressed(KeyCode::KeyS)) {
    }
    if (input.pressed(KeyCode::KeyA)) {

    }
    if (input.pressed(KeyCode::KeyD)) {

    }
}


pub fn key_code_input_controller(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut controllers: Query<
        (
            &mut KinematicCharacterController,
            &mut Transform,
            Option<&MainState<{ const_creature_state::run }>>,
            Option<&MainState<{ const_creature_state::walk }>>,
            &TimePass<{ const_time::state_timer }>,
        ),
        (With<Marker<{ const_base::local_player }>>,Without<ClimbUpWapll>),
    >,
) {
    if(controllers.is_empty()) {return}
    if (input.pressed(KeyCode::KeyW)) {
        let (mut controller, mut transform, is_run, is_walk, time_pass) = controllers.single_mut();
        let forward = transform.forward();
        let mut base_speed = 1.0;
        if (is_run.is_some()) {
            base_speed = 1.4;
        }
        if (is_walk.is_some()) {
            base_speed = 1.0;
        }
        controller.translation = Some(
            -forward.normalize()
                * time.delta_seconds()
                * std::f32::consts::PI
                * time_pass.elapsed_time
                * base_speed,
        );
    }
    if (input.pressed(KeyCode::KeyS)) {
        let (mut controller, mut transform, is_run, is_walk, time_pass) = controllers.single_mut();
        let forward = transform.forward();
        controller.translation = Some(
            forward.normalize()
                * time.delta_seconds()
                * std::f32::consts::PI
                * time_pass.elapsed_time,
        );
    }
    if (input.pressed(KeyCode::KeyA)) {
        let (mut controller, mut transform, is_run, is_walk, time_pass) = controllers.single_mut();
        let rotation_speed = time.delta_seconds() * std::f32::consts::PI * 2.0; // 每秒 45 度旋转
        transform.rotate(Quat::from_rotation_y(rotation_speed));
    }
    if (input.pressed(KeyCode::KeyD)) {
        let (mut controller, mut transform, is_run, is_walk, time_pass) = controllers.single_mut();
        let rotation_speed = time.delta_seconds() * std::f32::consts::PI * 2.0; // 每秒 45 度旋转
        transform.rotate(Quat::from_rotation_y(-rotation_speed));
    }
}

#[derive(MaskSys)]
struct MainCamera;

type FollowCameraTarget = Tag_1_2;
type CameraTargetTick = Tag_2_4;
type localPlayerUpdate = Tag_3_8;
type CameraOffset = Tag_4_16;


impl<Content: MaskSystemContent + 'static> MaskSystem<FollowCameraTarget, Content> for MainCamera {
    const _marker: usize = 2;

    type Output = (
        fn(
            Commands,
            Gizmos,
            Res<Time>,
            ParamSet<(
                Query<
                    (&Transform),
                    (
                        With<CameraView<{ const_camera_view::extreme_long_shot }>>,
                        With<CameraTarget>,
                    ),
                >,
                Query<
                    (
                        Entity,
                        &mut Transform,
                        &TimePass<{ const_time::camera_timer }>,
                        &Link<{ const_link_type::camera_target }>,
                        &mut TransformNote::<{const_transfrom_note::camera_target}>,
                        &TransformOffset::<{const_transfrom_note::camera_target}>
                    ),
                    (
                        With<Camera>,
                        With<Link<{ const_link_type::camera_target }>>,
                        With<TimePass<{ const_time::camera_timer }>>,
                        With<TransformNote::<{const_transfrom_note::camera_target}>>
                    ),
                >,
            )>,
        ),
    );

    fn export() -> Self::Output {
        (
            |
                mut cmd:Commands,
                mut gz:Gizmos,
                mut time:Res<Time>,
                mut query: ParamSet<(
                Query<
                    (&Transform),
                    (
                        With<CameraView<{ const_camera_view::extreme_long_shot }>>,
                        With<CameraTarget>,
                    ),
                >,
                Query<
                    (
                        Entity,
                        &mut Transform,
                        &TimePass<{ const_time::camera_timer }>,
                        &Link<{ const_link_type::camera_target }>,
                        &mut TransformNote::<{const_transfrom_note::camera_target}>,
                        &TransformOffset::<{const_transfrom_note::camera_target}>
                    ),
                    (
                        With<Camera>,
                        With<Link<{ const_link_type::camera_target }>>,
                        With<TimePass<{ const_time::camera_timer }>>,
                        With<TransformNote::<{const_transfrom_note::camera_target}>>
                    ),
                >,
            )>| {
                // 获取主相机和关联的目标
                let binding = query.p1(); // 可变借用相机的 Transform
                // let (mut main_camera_transform, time_pass, link) = binding.single_mut();
                if(binding.is_empty()) {return};
                let target_ent = binding.single().3.link.clone();

                drop(binding);
                // 获取目标实体的 Transform（仅读取）
                let target_transform = {
                    query.p0() // 不可变借用目标的 Transform
                        .get(target_ent)
                        .expect("camera target error")
                        .clone()
                };
                
                let mut binding = query.p1();
                let (ent,mut main_camera_transform, time_pass, link,mut transfrom_note,transform_offset) = binding.single_mut();

                let curr_lerp = berlin_curve(time_pass.elapsed_time / time_pass.max_time);

                let offset_distance = 10.0; // 你可以调整这个值

                // 获取目标物体的朝向
                let target_forward = target_transform.rotation  * Vec3::Z ; // 假设Z轴是目标的朝向
                
                // 计算相机的偏移量，确保相机在目标身后
                let offset = -target_forward.normalize() * offset_distance;

                let offset_Y = Vec3::Y * 3.4;

                
                
                // 计算新的相机位置
                let mut new_camera_position = target_transform.translation + offset + offset_Y ;
  
            

                  let curr_position = lerp_vec3(
                      transfrom_note.0.translation.xyz(),
                      new_camera_position,
                      curr_lerp,
                  );

                  gz.sphere(target_transform.translation, Quat::IDENTITY,2.0, Color::WHITE);

                main_camera_transform.translation = curr_position;
                main_camera_transform.translate_around(target_transform.translation + offset_Y, transform_offset.0.rotation);
                let new_rotation = main_camera_transform.looking_at(target_transform.translation + offset_Y, Vec3::Y);
                main_camera_transform.rotation = new_rotation.rotation;

                
            },
        )
    }
}

fn berlin_curve(t: f32) -> f32 {
    6.0 * t.powi(5) - 15.0 * t.powi(4) + 10.0 * t.powi(3)
}


impl<Content: MaskSystemContent + 'static> MaskSystem<CameraTargetTick, Content> for MainCamera {
    const _marker: usize = 4;

    type Output = (
        fn(
            Commands,
            Res<Time>,
            Query<
                (
                    Entity,
                    &mut Transform,
                    &mut TimePass<{ const_time::camera_timer }>,
                    &Link<{ const_link_type::camera_target }>,
                ),
                (With<Camera>, With<Link<{ const_link_type::camera_target }>>),
            >,
        ),
    );

    fn export() -> Self::Output {
        (
            |mut cmd: Commands,
             mut time: Res<Time>,
             mut main_camera: Query<
                (
                    Entity,
                    &mut Transform,
                    &mut TimePass<{ const_time::camera_timer }>,
                    &Link<{ const_link_type::camera_target }>,
                ),
                (With<Camera>, With<Link<{ const_link_type::camera_target }>>),
            >| {
                if(main_camera.is_empty()) {return}
                let (ent, mut mian_camera_transfrom, mut time_pass, link) =
                    main_camera.single_mut();
                if (time_pass.is_over) {
                    cmd.entity(ent)
                        .remove::<TimePass<{ const_time::camera_timer }>>();
                    cmd.entity(ent)
                        .remove::<Link<{ const_link_type::camera_target }>>();
                    cmd.entity(ent)
                    .remove::<TransformNote<{ const_transfrom_note::camera_target }>>();
                }
                time_pass.tick(time.delta_seconds());
            },
        )
    }
}

impl<Content: MaskSystemContent + 'static> MaskSystem<localPlayerUpdate, Content> for MainCamera {
    const _marker: usize = 8;

    type Output = (
        fn(
            Commands,
            Res<Time>,
            Query<(Entity, &mut Transform), (With<Camera>)>,
            Query<(Entity), (With<Marker<{ const_base::local_player }>>)>,
        ),
    );

    fn export() -> Self::Output {
        (
            |
                  mut cmd:Commands,
                  mut time:Res<Time>,
                  mut main_camera:Query<(Entity,&mut Transform), (With<Camera>)>,
                  mut local_player:Query<(Entity), (With<Marker<{const_base::local_player}>>)>,
            |{
                 for local_player_ent in local_player.iter(){
                     println!("init camera target");
                     let (camera_ent,transform) = main_camera.single();
                     cmd.entity(local_player_ent).insert(CameraTarget);
                     cmd.entity(local_player_ent).insert(CameraView::<{const_camera_view::extreme_long_shot}>);
                     cmd.entity(camera_ent).insert(
                        TimePass::<{const_time::camera_timer}>{
                            start_time: 0.0,
                            max_time: 1.5,
                            is_over: false,
                            is_stop: false,
                            elapsed_time: 0.0,
                        }
                     )
                     .insert(TransformNote::<{const_transfrom_note::camera_target}>(transform.clone()))
                     .insert(Link::<{const_link_type::camera_target}>{
                          srouce:camera_ent,
                          link:local_player_ent
                     });
                     cmd.entity(camera_ent).insert(TransformOffset::<{const_transfrom_note::camera_target}>(Transform::from_xyz(0.0, 0.0, 0.0)));
                 }
            },
        )
    }
}

impl<Content: MaskSystemContent + 'static> MaskSystem<CameraOffset, Content> for MainCamera {
    const _marker: usize = 16;

    type Output = (
        fn(
            Commands,
            Res<Time>,
            Res<ButtonInput<MouseButton>>,
            EventReader<CursorMoved>,
            Query<(Entity,&Link<{const_link_type::camera_target}>,&mut TransformOffset<{const_transfrom_note::camera_target}>), (With<Camera>)>,
        )
    );

    fn export() -> Self::Output {
        (
            |
                mut cmd:Commands,
                mut time:Res<Time>,
                mouse_input:Res<ButtonInput<MouseButton>>,
                mut events:EventReader<CursorMoved>,
                mut query:Query<(Entity,&Link<{const_link_type::camera_target}>,&mut TransformOffset<{const_transfrom_note::camera_target}>), (With<Camera>)>,
            |{
                if(query.is_empty()){return}
                let (camera_ent,link,mut transfrom_offset) = query.single_mut();
                let event = events.read();
                for cursor_moved in event{
                if(mouse_input.pressed(MouseButton::Right)){
                    cursor_moved.delta.map(|m|{
                        println!("{}",m);
                        let new_m = m.normalize();
                        transfrom_offset.0.rotate_y(-new_m.x * time.delta_seconds() * 2.0);
                        transfrom_offset.0.rotate_x(new_m.y * time.delta_seconds()* 2.0);
                   });
                 }
              }
            }
        )
    }
}

fn step_normalize(value: f32, positive_output: f32, negative_output: f32) -> f32 {
    if value > 0.0 {
        positive_output // 返回第一个参数
    } else if value < 0.0 {
        negative_output // 返回第二个参数
    } else {
        0.0 // 零的情况下返回 0
    }
}

fn lerp_vec3(start: Vec3, end: Vec3, t: f32) -> Vec3 {
    start + t * (end - start)
}

fn fade_in_out(elapsed_time: f32, fade_duration: f32) -> f32 {
    let half_duration = fade_duration / 2.0;

    // 淡入
    if elapsed_time < half_duration {
        lerp(0.0, 1.0, elapsed_time / half_duration)
    }
    // 保持为 1
    else if elapsed_time < fade_duration - half_duration {
        1.0
    }
    // 淡出
    else if elapsed_time < fade_duration {
        lerp(1.0, 0.0, (elapsed_time - (fade_duration - half_duration)) / half_duration)
    }
    // 结束时保持透明
    else {
        0.0
    }
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + t * (end - start)
}
