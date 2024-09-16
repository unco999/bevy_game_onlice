use std::{hash::Hash, string};

use bevy::{asset::Handle, prelude::Resource, utils::HashMap};
use serde::Deserialize;

#[derive(Debug)]
#[derive(Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct AnimationTableCache {
    pub hash_map: HashMap<String,AnimationTag>,
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct AnimationTag{
    pub name:String,
    pub state_to_animation_id:HashMap<usize,usize>,
}

#[derive(Resource)]
pub struct AnimationTableRecord(pub Handle<AnimationTableCache>);

#[derive(Resource)]
pub struct ConfigCache{
    pub animation_table_cache:AnimationTableCache,
}