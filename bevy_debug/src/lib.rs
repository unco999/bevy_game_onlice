use std::{f32::consts::PI, time::Duration};

use bevy::{app::{Plugin, PostStartup, Startup, Update}, asset::{AssetEvent, AssetServer, Assets, Handle}, color::palettes::css::{BLUE, LIME, ORANGE_RED, RED, WHITE}, gltf::{Gltf, GltfAssetLabel}, input::ButtonInput, math::{Quat, Vec3}, pbr::{light_consts, AmbientLight, CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle, PointLight, PointLightBundle, SpotLight, SpotLightBundle}, prelude::{default, run_once, Added, AnimationGraph, AnimationNodeIndex, AnimationPlayer, AnimationTransitions, Camera3dBundle, Commands, Component, Entity, EventReader, IntoSystemConfigs, KeyCode, Local, Query, Res, ResMut, Resource, Transform, With}, scene::SceneBundle, time::common_conditions::on_timer};
use bevy_base::structs::{comp::{Link, Marker}, const_base, const_link_type};
use bevy_blendy_cameras::{FlyCameraController, OrbitCameraController};
use bevy_entity_state::structs::{comp::*, const_creature_state};
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update,test_input)
        .add_systems(Startup,model)
        .add_systems(Startup,setup)
        .add_systems(Update,setup_scene_once_loaded)

        ;
    }
}

fn test_input(
    mut cmd:Commands,
    input:Res<ButtonInput<KeyCode>>,
    query:Query<(Entity),(With<Marker::<{const_base::creature}>>,With<Link::<{ const_link_type::state }>>)>
){
    if(input.just_pressed(KeyCode::KeyW)){
        println!("pressed W");
        query.iter().for_each(|e|{
            cmd.entity(e).insert(AppointStateTransition::<{const_creature_state::run}>);
        });
    }
    if(input.just_pressed(KeyCode::KeyS)){
        println!("pressed S");
        query.iter().for_each(|e|{
            cmd.entity(e).insert(DefualtStateTransition);
        });
    }
}

static CLIP_NODE_INDICES: [u32; 2] = [13,20];

fn model(
    mut cmd:Commands,
    asset_server: Res<AssetServer>
){
    cmd.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        OrbitCameraController::default(),
        FlyCameraController {
            is_enabled: false,
            ..default()
        },
    ));
    cmd.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 5.0, 5.0),
            rotation: Quat::from_rotation_x(-PI * 1.5 ),
            ..default()
        },
        // This is a relatively small scene, so use tighter shadow
        // cascade bounds than the default for better quality.
        // We also adjusted the shadow map to be larger since we're
        // only using a single cascade.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 0.5,
            ..default()
        }
        .into(),
        ..default()
    });
    cmd.insert_resource(AmbientLight {
        color: ORANGE_RED.into(),
        brightness: 0.05,
    });
    cmd.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 1.0,
    });
    cmd
    .spawn(SpotLightBundle {
        transform: Transform::from_xyz(3.0, 0.0, 13.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
        spot_light: SpotLight {
            intensity: 1800_000.0,
            color: BLUE.into(),
            shadows_enabled: true,
            inner_angle: 0.6,
            outer_angle: 0.8,
            ..default()
        },
        ..default()
    });
    cmd.spawn(PointLightBundle {
        // transform: Transform::from_xyz(5.0, 8.0, 2.0),
        transform: Transform::from_xyz(-3.0, 0.0, 13.0),
        point_light: PointLight {
            intensity: 1550_000.0,
            color: ORANGE_RED.into(),
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    cmd.spawn(SceneBundle {
        scene: asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("models/xiake.glb")),
        ..default()
    })
    ;


}
#[derive(Resource)]
struct Animations {
    animations: Vec<AnimationNodeIndex>,
    #[allow(dead_code)]
    graph: Handle<AnimationGraph>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // Build the animation graph
    let mut graph = AnimationGraph::new();
    let animations1 = graph.add_clip( asset_server.load(GltfAssetLabel::Animation(13).from_asset("models/xiake.glb")), 1.0, graph.root);
    let animations2 = graph.add_clip( asset_server.load(GltfAssetLabel::Animation(20).from_asset("models/xiake.glb")), 1.0, graph.root);
    // Insert a resource with the current scene information
    let graph = graphs.add(graph);
    commands.insert_resource(Animations {
        animations:vec![animations1,animations2],
        graph: graph.clone(),
    });
    println!("Animation controls:");
    println!("  - spacebar: play / pause");
    println!("  - arrow up / down: speed up / slow down animation playback");
    println!("  - arrow left / right: seek backward / forward");
    println!("  - digit 1 / 3 / 5: play the animation <digit> times");
    println!("  - L: loop the animation forever");
    println!("  - return: change animation");
}

fn setup_scene_once_loaded(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    asset_server: Res<AssetServer>,
    mut be:Local<bool>
) {
    if(*be){
        return;
    }
        let mut id: Option<Handle<Gltf>> = asset_server.get_handle("models/xiake.glb");
        let assets_id;
        if(id.is_some()){
            assets_id = id.unwrap().id(); 
        }else{
            return;
        }
        
        if asset_server.is_loaded_with_dependencies(assets_id) {
            for (entity, mut player) in &mut players {
                let mut transitions = AnimationTransitions::new();
        
                // Make sure to start the animation via the `AnimationTransitions`
                // component. The `AnimationTransitions` component wants to manage all
                // the animations and will get confused if the animations are started
                // directly via the `AnimationPlayer`.

                for &node_index in &CLIP_NODE_INDICES{
                    println!("播放了node——index {}",node_index);
                    player.play(node_index.into()).repeat();

                    // if let Some(active_animation) =
                    //     player.animation_mut(node_index.into())
                    // {
                    //     println!("设置了权重为0.5");
                    //     active_animation.set_weight(0.5);
                    // }
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
                commands
                    .entity(entity)
                    .insert(animations.graph.clone())
                    .insert(transitions);
        
            
            *be = true;

        }
        
    }
}