use std::{f32::consts::PI, time::Duration};

use bevy::{
    app::{FixedUpdate, Plugin, PostStartup, Startup, Update}, asset::{AssetEvent, AssetServer, Assets, Handle}, color::{palettes::css::{BLUE, LIME, ORANGE_RED, RED, WHITE}, Color}, ecs::query, gltf::{Gltf, GltfAssetLabel, GltfSceneExtras,*}, input::ButtonInput, math::{Quat, Vec3, Vec4}, pbr::{
        light_consts, AmbientLight, CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle, PbrBundle, PointLight, PointLightBundle, SpotLight, SpotLightBundle, StandardMaterial
    }, prelude::{
        default, in_state, run_once, Added, AnimationGraph, AnimationNodeIndex, AnimationPlayer, AnimationTransitions, Camera3dBundle, Commands, Component, Entity, EventReader, IntoSystemConfigs, KeyCode, Local, Mesh, Meshable, NextState, OnEnter, Plane3d, Query, Res, ResMut, Resource, SystemSet, Transform, TransformBundle, Visibility, VisibilityBundle, With, Without
    }, render::view::GpuCulling, scene::SceneBundle, time::common_conditions::on_timer
};
use bevy_animation::plugin::AnimationSystem;
use bevy_base::structs::{
    comp::{Link, Marker, Name, SpawnCollision, TimePass}, const_base, const_link_type, const_time, AppState
};
use bevy_blendy_cameras::{FlyCameraController, OrbitCameraController};
use bevy_collision::structs::{comp::ColliderType, const_collider_type};
use bevy_entity_state::structs::{comp::*, const_base_state, const_creature_state};

use bevy_rapier3d::prelude::*;


pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, test_input)
            .add_systems(OnEnter(AppState::GameStart), (test_init).in_set(DebugSys::Init))
            .add_systems(Update, display_events);
            // .add_systems(Update,collider_check);
            
    }
}


#[derive(SystemSet,Debug,PartialEq,Eq,Clone,Hash)]
pub enum DebugSys{
    Init,
    Over
}

fn collider_check(
    mut cmd:Commands,
    mut query:Query<(Entity,&Handle<Mesh>),(Without<Collider>)>,
    meshes: Res<Assets<Mesh>>,
){
    for (ent,mesh) in &mut query{
        cmd.entity(ent).insert(
            Collider::from_bevy_mesh(
                meshes.get(mesh).unwrap(),
                &ComputedColliderShape::TriMesh,
            )
            .unwrap(),
        );
    }
}

fn test_input(
    mut cmd: Commands,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<
        (Entity),
        (
            With<Marker<{ const_base::creature }>>,
            With<Link<{ const_link_type::state }>>,
        ),
    >,
    mut input_lock:Local<bool>
) {

    if input.just_released(KeyCode::KeyW) {
        println!("just_released W");
        query.iter().for_each(|e| {
            cmd.entity(e)
                .insert(AppointStateTransition::<{ const_creature_state::idle }>);
        });

        *input_lock = false;
        return;
    }

    if (*input_lock){
        return;
    }


    if (input.pressed(KeyCode::KeyW)) {
        println!("pressed W");
        query.iter().for_each(|e| {
            cmd.entity(e)
                .insert(AppointStateTransition::<{ const_creature_state::walk }>);
        });

        *input_lock = true;
    }
    if (input.pressed(KeyCode::KeyS)) {
        println!("pressed S");

        query.iter().for_each(|e| {
            cmd.entity(e).insert(AppointStateTransition::<{ const_creature_state::idle }>);
        });

        *input_lock = true;

    }
    if (input.pressed(KeyCode::KeyU)) {
        query.iter().for_each(|e| {
            cmd.entity(e).insert(AppointStateTransition::<{ const_creature_state::attack1 }>);
        });

        *input_lock = true;

    }

    if input.just_released(KeyCode::KeyS) {
    }
    if input.just_released(KeyCode::KeyU) {
    }
}

static CLIP_NODE_INDICES: [u32; 2] = [13, 20];


fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

fn test_init(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut meshs:ResMut<Assets<Mesh>>,
    mut mat:ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(200.0, 200.5, 5.0)),
            ..default()
        },
        // OrbitCameraController::default(),
        // FlyCameraController {
        //     is_enabled: false,
        //     ..default()
        // },
    ));
    cmd.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::srgb(1.0, 0.9, 0.8), // 模拟自然光的暖色调
                illuminance: 7_000.0, // 合适的强度
                shadows_enabled: false,
                ..default()
            },
            transform: Transform {
                rotation: Quat::from_vec4(Vec4::new(6.0, -0.2, -5.1, 1.0)), // 光线从45°角照射
                ..default()
            },
            ..default()
        },
    ));

    cmd.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::srgb(0.95, 0.9, 0.8), // 模拟自然光的暖色调
                illuminance: 500.0, // 合适的强度
                shadows_enabled: false,
                ..default()
            },
            transform: Transform {
                rotation: Quat::from_rotation_x(-std::f32::consts::PI / 1.5), // 光线从45°角照射
                ..default()
            },
            ..default()
        },
    ));

    cmd.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::srgb(0.95, 0.9, 0.8), // 模拟自然光的暖色调
                illuminance: 500.0, // 合适的强度
                shadows_enabled: false,
                ..default()
            },
            transform: Transform {
                rotation: Quat::from_rotation_x(-std::f32::consts::PI), // 光线从45°角照射
                ..default()
            },
            ..default()
        },
    ));

    cmd.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::srgb(0.95, 0.9, 0.8), // 模拟自然光的暖色调
                illuminance: 500.0, // 合适的强度
                shadows_enabled: false,
                ..default()
            },
            transform: Transform {
                rotation: Quat::from_rotation_x(-std::f32::consts::PI * 1.5), // 光线从45°角照射
                ..default()
            },
            ..default()
        },
    ));

    // 环境光 - 用来填充阴影
    cmd.insert_resource(AmbientLight {
        color: Color::srgb(0.9, 0.7, 0.65), // 稍微带冷色调的环境光
        brightness: 0.8, // 增加亮度来填充黑暗的阴影
    });



    // // 补光源2 - 右侧的点光源
    // cmd.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 500_000.0,
    //         range: 300.0,
    //         shadows_enabled: false,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(200.0, 150.0, 100.0), // 另一侧的补光
    //     ..default()
    // });

    // // 补光源3 - 背景补光
    // cmd.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 400_000.0, // 稍弱的背景补光
    //         range: 500.0,
    //         shadows_enabled: false, // 不需要产生阴影
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(0.0, 500.0, -400.0), // 放置在场景的背后
    //     ..default()
    // });


    let scenes = cmd.spawn((SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("scenes/city/chinese_house_collision.glb")),
        ..default()
        })
    ).insert(
        (
        AsyncSceneCollider::default(),
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS | ActiveEvents::CONTACT_FORCE_EVENTS,
        ColliderType::<{const_collider_type::wall}>,
        Visibility::Hidden
        )
    );

    let scenes = cmd.spawn((SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("scenes/city/chinese_house.glb")),
        ..default()
        })
    );

    let ground_size = 100.0;
    let ground_height = 0.1;

    cmd.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, -ground_height, 0.0)),
        Collider::cuboid(ground_size, ground_height, ground_size),
        RigidBody::Fixed
    ));

    let main_ent = cmd.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/xiake.glb")),
        ..default()
    })
    .insert(Marker::<{const_base::local_player}>)
    .insert(Name("models/xiake.glb".into()))
    .insert(RigidBody::KinematicPositionBased)
    .insert(ContactForceEventThreshold(1.0))
    .insert(Collider::cuboid(0.2,1.0,0.1))
    .insert(        TransformBundle::from(Transform::from_xyz(1.0, 0.0, 1.0)))
    .insert(KinematicCharacterController {
        ..KinematicCharacterController::default()
    })
    .insert(ActiveEvents::COLLISION_EVENTS | ActiveEvents::CONTACT_FORCE_EVENTS )
    .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC)
    .insert(MainState::<{ const_creature_state::spawn }>)
    .insert(TimePass::<{ const_time::state_timer }> {
            start_time: 0.0,
            max_time: 0.614,
            is_over: false,
            is_stop: false,
            elapsed_time: 0.0,
        })
        .id();
    let sub_ent = cmd
        .spawn_empty()
        .insert(SubState::<{ const_base_state::ENTRY }>)
        .id();
    let m2clink = Link::<{ const_link_type::sub_state }> {
        srouce: main_ent,
        link: sub_ent,
    };
    let c2mlink = Link::<{ const_link_type::state }> {
        srouce: sub_ent,
        link: main_ent,
    };
    cmd.entity(main_ent)
        .insert(m2clink)
        .insert(Marker::<{ const_base::creature }>);
    cmd.entity(sub_ent)
        .insert(c2mlink)
        .insert(TimePass::<{ const_time::state_timer }> {
            start_time: 0.0,
            max_time: 0.08,
            is_over: false,
            is_stop: false,
            elapsed_time: 0.0,
        })
        .insert(Marker::<{ const_base::creature }>);

    println!("初始化了系统");
}
#[derive(Resource)]
struct Animations {
    animations: Vec<AnimationNodeIndex>,
    #[allow(dead_code)]
    graph: Handle<AnimationGraph>,
}


fn setup_scene_once_loaded(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    mut state: ResMut<NextState<AppState>>,
) {
    for (entity, mut player) in &mut players {
        // Make sure to start the animation via the `AnimationTransitions`
        // component. The `AnimationTransitions` component wants to manage all
        // the animations and will get confused if the animations are started
        // directly via the `AnimationPlayer`.

        for &node_index in &CLIP_NODE_INDICES {
            println!("播放了node——index {}", node_index);
            player.play(node_index.into()).repeat();

        }
        // let test_anim = player.animation_mut(13u32.into());
        // if let Some(t) = test_anim{
        //     println!("播放2个动画");
        //     t.set_weight(0.50);
        // }

        // transitions
        //     .play(&mut player, animations.animations[1], Duration::ZERO)
        //     .repeat();
        // let test_anim = player.animation_mut(20u32.into());
        // if let Some(t) = test_anim{
        //     println!("播放2个动画");
        //     t.set_weight(0.50);
        // }
        println!("开始播放");
        state.set(AppState::GameStart);
    }
}

fn check_for_gltf_extras(
    gltf_extras_per_entity: Query<(
        Entity,
        Option<&Name>,
        Option<&GltfSceneExtras>,
        Option<&GltfExtras>,
        Option<&GltfMeshExtras>,
        Option<&GltfMaterialExtras>,
    )>,
) {
    let mut gltf_extra_infos_lines: Vec<String> = vec![];

    for (id, name, scene_extras, extras, mesh_extras, material_extras) in
        gltf_extras_per_entity.iter()
    {
        if scene_extras.is_some()
            || extras.is_some()
            || mesh_extras.is_some()
            || material_extras.is_some()
        {
            let mut formatted_extras = format!(
                "Extras per entity {}
    - scene extras:     {:?}
    - primitive extras: {:?}
    - mesh extras:      {:?}
    - material extras:  {:?}
                ",
                id,
                scene_extras,
                extras,
                mesh_extras,
                material_extras
            );
            println!("{}",formatted_extras.clone());
        }
    }
}