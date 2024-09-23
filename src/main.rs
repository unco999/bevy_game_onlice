
#![recursion_limit = "256"]
#![feature(specialization)]
#![feature(trait_alias)]
#![feature(generic_const_exprs)]
#![feature(const_type_id)]
#![feature(associated_type_defaults)]
#![feature(unboxed_closures)]
#![feature(unsize)]
#![feature(const_trait_impl)]
use std::{default, sync::{atomic::AtomicBool, Arc}};

use bevy::{asset::{AssetEvent, AssetServer, Assets, Handle}, gltf::{Gltf, GltfAssetLabel}, pbr::DirectionalLightShadowMap, prelude::{in_state, on_event, AppExtStates, EventReader, IntoSystemConfigs, Local, NextState, Res, ResMut, States, World}, tasks::{futures_lite::future, AsyncComputeTaskPool}};
#[warn(incomplete_features)]
use bevy::{
    app::{App, Startup, Update}, ecs::{component::Components, query::{QueryData, QueryFilter}}, math::bool, prelude::{default, Commands, Component, Entity, IntoSystem, Query, QueryBuilder, With, Without}, reflect::DynamicTuple, ui::shader_flags::BORDER, utils::all_tuples, DefaultPlugins
};
use bevy_animation::plugin::AnimationPlugin;
use bevy_base::structs::AppState;
use bevy_blendy_cameras::BlendyCamerasPlugin;
use bevy_collision::plugin::ColliderPlugin;
use bevy_config::{plugin::Config, structs::{AnimationTableCache, AnimationTableRecord}};
use bevy_controller::plugin::ControllerPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_entity_state::plugins::BevyEntityStatePlugin;
use bevy_mask_system::MaskSys;
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use mask_system_lib::{*};
use bevy_debug::DebugPlugin;

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(BevyEntityStatePlugin)
        .add_plugins(
            (
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(ColliderPlugin)
        .add_plugins(ControllerPlugin)
        .add_plugins(DebugPlugin)
        .init_state::<AppState>()
        .add_plugins(Config)
        // .add_plugins(BlendyCamerasPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AnimationPlugin)
        .run();
}


// fn test_init_assets(
//     asset_server: Res<AssetServer>,
// ){
//     // let a1:Handle<Gltf> = asset_server.load(GltfAssetLabel::Scene(0).from_asset("scenes/city/chinese.glb"));
//     let a1: Handle<Gltf> = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/xiake.glb"));
//     let loading_state = Arc::new(AtomicBool::new(false));
//     AsyncComputeTaskPool::get()
//     .spawn(async move {
//         println!("延迟打印");
//         loading_state.store(true, order);
//     })
//     .detach();
// }


