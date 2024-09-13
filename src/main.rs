
#![recursion_limit = "256"]
#![feature(specialization)]
#![feature(trait_alias)]
#![feature(generic_const_exprs)]
#![feature(const_type_id)]
#![feature(associated_type_defaults)]
#![feature(unboxed_closures)]
#![feature(unsize)]
#![feature(const_trait_impl)]
#[warn(incomplete_features)]
use bevy::{
    app::{App, Startup, Update}, ecs::{component::Components, query::{QueryData, QueryFilter}}, math::bool, prelude::{default, Commands, Component, Entity, IntoSystem, Query, QueryBuilder, With, Without}, reflect::DynamicTuple, ui::shader_flags::BORDER, utils::all_tuples, DefaultPlugins
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_entity_state::plugins::BevyEntityStatePlugin;
use bevy_mask_system::MaskSys;
use mask_system_lib::{*};
use bevy_debug::DebugPlugin;
fn main() {
    App::new()
        .add_plugins(BevyEntityStatePlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(DebugPlugin)
        .add_plugins(WorldInspectorPlugin::new())

        .run();
}

#[derive(MaskSys)]
struct Weapon;




