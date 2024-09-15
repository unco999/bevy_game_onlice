use std::hash::Hash;

use bevy::{asset::Handle, prelude::Resource, utils::HashMap};
use serde::Deserialize;

#[derive(Debug)]
#[derive(Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct AnimationTableCache {
    pub hash_map: HashMap<String,i32>,
}

#[derive(Resource)]
pub struct AnimationTableRecord(pub Handle<AnimationTableCache>);