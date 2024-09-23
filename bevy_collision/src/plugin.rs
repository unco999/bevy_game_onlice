use bevy::{app::{FixedUpdate, Plugin}, ecs::query, math::Vec3, prelude::{in_state, Changed, Commands, Entity, IntoSystemConfigs, Query, With}};
use bevy_base::structs::{comp::Marker, const_base, AppState};
use bevy_entity_state::structs::{comp::AppointStateTransition, const_creature_state, special_aniamtion::ClimbUpWapll};
use bevy_rapier3d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};

use crate::structs::{comp::ColliderType, const_collider_type};

pub struct  ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(FixedUpdate,read_character_controller_collisions.run_if(in_state(AppState::GameStart)));
    }
}

fn read_character_controller_collisions(
    mut cmd:Commands,
    mut query:Query<(Entity),(With<ColliderType<{const_collider_type::wall}>>)>,
    mut local_entity:Query<Entity,(With<Marker<{const_base::local_player}>>)>,
    mut character_controller_outputs: Query<(Entity,&mut KinematicCharacterController,&mut KinematicCharacterControllerOutput),(Changed<KinematicCharacterController>)>,
) {
    for (ent,mut controller,mut output) in character_controller_outputs.iter_mut() {
        for collision in &output.collisions {
            // Do something with that collision information.
            let ent = query.get(collision.entity);
            cmd.entity(local_entity.single()).insert(AppointStateTransition::<{const_creature_state::climb_up}>);
            cmd.entity(local_entity.single()).insert(ClimbUpWapll{
                target_wall: collision.entity
            });
            // if(ent.is_ok()){
            // }
                    
        }
    }
}