
#![recursion_limit = "256"]
#![feature(specialization)]
#![feature(trait_alias)]
#![feature(generic_const_exprs)]
#![feature(const_type_id)]
#![feature(associated_type_defaults)]
#![feature(unboxed_closures)]
#![feature(unsize)]
#![feature(const_trait_impl)]
use std::default;

use bevy::{asset::{AssetServer, Handle}, gltf::{Gltf, GltfAssetLabel}, pbr::DirectionalLightShadowMap, prelude::{in_state, AppExtStates, IntoSystemConfigs, Local, NextState, Res, ResMut, States}};
#[warn(incomplete_features)]
use bevy::{
    app::{App, Startup, Update}, ecs::{component::Components, query::{QueryData, QueryFilter}}, math::bool, prelude::{default, Commands, Component, Entity, IntoSystem, Query, QueryBuilder, With, Without}, reflect::DynamicTuple, ui::shader_flags::BORDER, utils::all_tuples, DefaultPlugins
};
use bevy_animation::plugin::AnimationPlugin;
use bevy_base::structs::AppState;
use bevy_blendy_cameras::BlendyCamerasPlugin;
use bevy_config::{plugin::Config, structs::{AnimationTableCache, AnimationTableRecord}};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_entity_state::plugins::BevyEntityStatePlugin;
use bevy_mask_system::MaskSys;
use mask_system_lib::{*};
use bevy_debug::DebugPlugin;

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(BevyEntityStatePlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(DebugPlugin)
        .init_state::<AppState>()
        .add_plugins(Config)
        .add_plugins(BlendyCamerasPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AnimationPlugin)
        .add_systems(Update, app_state_check_gltf.run_if(in_state(AppState::ConfigProcessingOver)))
        .add_systems(Update, app_state_check_config.run_if(in_state(AppState::Init)))
        .run();
}

fn app_state_check_config(
    asset_server: Res<AssetServer>,
    mut state:ResMut<NextState<AppState>>,
    mut open:Local<bool>
){
    if(*open){
        return;
    }
    let r1: Option<Handle<AnimationTableCache>> = asset_server.get_handle("config/animation.ron");

    let app_config_read_ok = vec![r1];

    let check = app_config_read_ok.iter()
            .all(|e|{
                e.is_some()
            });

    if(check){
        state.set(AppState::ConfigProcessing);
        println!("game state -> ConfigProcessing");
        *open = true
    }
}

fn app_state_check_gltf(
    asset_server: Res<AssetServer>,
    mut state:ResMut<NextState<AppState>>,
    mut open:Local<bool>
){
    if(*open){
        return;
    }
    let g1: Option<Handle<Gltf>> = asset_server.get_handle("models/xiake.glb");

    let app_config_read_ok = vec![g1];

    let check = app_config_read_ok.iter()
            .all(|e|{
                e.is_some() && asset_server.is_loaded_with_dependencies(e.clone().unwrap().id())
            });

    if(check){
        state.set(AppState::GameStart);
        println!("game state -> ResourceProcessing");
        *open = true;
    }else {
        let a1: Handle<Gltf> = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/xiake.glb"));
        let a1: Handle<Gltf> = asset_server.load(GltfAssetLabel::Animation(13).from_asset("models/xiake.glb"));
        let a1: Handle<Gltf> = asset_server.load(GltfAssetLabel::Animation(20).from_asset("models/xiake.glb"));
    }
}

