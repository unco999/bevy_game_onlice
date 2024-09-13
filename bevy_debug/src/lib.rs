use bevy::{app::{Plugin, Update}, input::ButtonInput, prelude::{Commands, Entity, KeyCode, Query, Res, With}};
use bevy_base::structs::{comp::{Link, Marker}, const_base, const_link_type};
use bevy_entity_state::structs::{comp::*, const_creature_state};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update,test_input);
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