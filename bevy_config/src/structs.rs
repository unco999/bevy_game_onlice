use std::{hash::Hash, string};

use bevy::{asset::{AssetId, Handle}, gltf::Gltf, prelude::Resource, utils::{HashMap, HashSet}};
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
    pub state_to_animation_id:HashMap<usize,(usize,f32)>,
}

#[derive(Resource)]
pub struct AnimationTableRecord(pub Handle<AnimationTableCache>);

#[derive(Resource,Default)]
pub struct ConfigCache{
    pub db_config:Option<LoadTable>,
    pub animation_table_cache:Option<AnimationTableCache>,
}


#[derive(Resource)]
pub struct LoadTableResouce(pub Handle<LoadTable>);


#[derive(Debug)]
#[derive(Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct LoadTable {
    pub models: Vec<ResourceInfo>,
    pub scenes: Vec<ResourceInfo>,
    pub animation: Vec<ResourceInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ResourceInfo {
    pub name: String,
    pub path: String,
}
#[derive(Resource,Default)]
pub struct GltfHandleTable{
    pub hash_set:HashSet<AssetId<Gltf>>
}

#[derive(Resource,Default)]
pub struct ScenesHandleTable{
    pub hash_set:HashSet<AssetId<Gltf>>
}

#[derive(Resource,Default)]
pub struct AnimationHandleTable{
    pub hash_set:HashSet<AssetId<Gltf>>
}