use bevy::{app::{Plugin, PostStartup, Startup, Update}, asset::{AssetEvent, AssetServer, Assets, Handle}, gltf::{Gltf, GltfAssetLabel}, prelude::{in_state, on_event, run_once, Commands, EventReader, IntoSystemConfigs, Local, NextState, OnEnter, Res, ResMut, SystemSet}};
use bevy_base::structs::AppState;
use bevy_common_assets::{ron::RonAssetPlugin};

use crate::structs::{AnimationHandleTable, AnimationTableCache, AnimationTableRecord, ConfigCache, GltfHandleTable, LoadTable, LoadTableResouce, ScenesHandleTable};

pub struct Config;

impl Plugin for  Config{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // .init_resource::<BootyResouce>()
            .add_plugins(RonAssetPlugin::<LoadTable>::new(&["config/read_db.ron"]))
            .add_plugins(RonAssetPlugin::<AnimationTableCache>::new(&["config/animation.ron"]))
            .add_systems(Update,(read_db_init,read_db).chain().run_if(in_state(AppState::Init)))
            .add_systems(OnEnter(AppState::ConfigProcessing),read_base_config)
            .add_systems(OnEnter(AppState::ConfigProcessingOver),read_gltf)
            .add_systems(OnEnter(AppState::ConfigProcessingOver),read_animation_table)
            .add_systems(OnEnter(AppState::ResourceProcessingOver),read_animation)
            .add_systems(Update, check_model_read_over.run_if(on_event::<AssetEvent<Gltf>>()))
            ;
    }
}



pub fn read_db_init(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
) {
    let table_res = LoadTableResouce(asset_server.load("config/read_db.ron"));
    cmd.insert_resource(table_res);
    cmd.insert_resource(ConfigCache::default());
    cmd.insert_resource(GltfHandleTable::default());
    cmd.insert_resource(ScenesHandleTable::default());
    cmd.insert_resource(AnimationHandleTable::default());

}

fn read_db(
    mut read_cache: ResMut<Assets<LoadTable>>,
    mut res:Res<LoadTableResouce>,
    mut cmd: Commands,
    mut state:ResMut<NextState<AppState>>,
    mut config_db:ResMut<ConfigCache>
){
    
    if let Some(mut data) = read_cache.remove(res.0.id()) {
        config_db.db_config = Some(data);
        state.set(AppState::ConfigProcessing);
    }
}

fn read_base_config(
    mut cmd: Commands,
    mut config_db:ResMut<ConfigCache>,
    asset_server: Res<AssetServer>,
    mut state:ResMut<NextState<AppState>>,
){
    config_db.db_config.as_mut().expect("db_error").animation.iter().for_each(|e|{
        println!("read config db in {}",e.name);
        let table_res = AnimationTableRecord(asset_server.load(e.path.clone()));
        cmd.insert_resource(table_res);
        state.set(AppState::ConfigProcessingOver);
    });
}

fn read_animation_table(
    mut config_db:ResMut<ConfigCache>,
    mut animation_table:Res<AnimationTableRecord>,
    mut read_cache: ResMut<Assets<AnimationTableCache>>,

){
    if let Some(mut data) = read_cache.remove(animation_table.0.id()) {
        println!("animation table read ok");
        config_db.animation_table_cache = Some(data);
    }
}

fn read_gltf(
    mut asset_server:Res<AssetServer>,
    mut config_db:ResMut<ConfigCache>,
    mut gltf_handle_reacord:ResMut<GltfHandleTable>,
    mut scenes_handle_reacord:ResMut<ScenesHandleTable>,
    mut state:ResMut<NextState<AppState>>,
){
    for gltf_info in config_db.db_config.as_ref().expect("db eroor").models.iter(){
        println!("read gltf model in {} path:{}",gltf_info.name,gltf_info.path);
        let handle: Handle<Gltf> = asset_server.load(&gltf_info.path);
        gltf_handle_reacord.hash_set.insert(handle.id());
    }
    for gltf_info in config_db.db_config.as_ref().expect("db eroor").scenes.iter(){
        println!("read gltf model in {} path:{}",gltf_info.name,gltf_info.path);
        let handle: Handle<Gltf> = asset_server.load(&gltf_info.path);
        scenes_handle_reacord.hash_set.insert(handle.id());
    }
    state.set(AppState::ResourceProcessing);
}

fn read_animation(
    mut asset_server:Res<AssetServer>,
    mut config_db:ResMut<ConfigCache>,
    mut state:ResMut<NextState<AppState>>,
    mut read_animation_tabel:ResMut<AnimationHandleTable>,
    mut lock:Local<bool>

){
    if(*lock){
        return;
    }
    for animation_info in config_db.animation_table_cache.as_ref().expect("db_error").hash_map.iter(){
        let name = animation_info.0;
        for (key,val) in &animation_info.1.state_to_animation_id{
            let handle: Handle<Gltf> = asset_server.load(GltfAssetLabel::Animation(val.0.clone()).from_asset(name.clone()));
            read_animation_tabel.hash_set.insert(handle.id());
            println!("read animation with {} index :{} handle:{}",name,val.0,handle.id());
        }
    }
    state.set(AppState::GameStart);
    *lock = true;
}


fn check_model_read_over(
    mut asset_events: EventReader<AssetEvent<Gltf>>,
    mut gltf_handle_reacord:Res<GltfHandleTable>,
    mut scenes_handle_reacord:ResMut<ScenesHandleTable>,
    mut read_animation_tabel:ResMut<AnimationHandleTable>,
    mut read_gltf_count:Local<usize>,
    mut read_scenes:Local<usize>,
    mut state:ResMut<NextState<AppState>>,
    mut read_animation_count:Local<usize>,
    mut lock:Local<bool>
){
    if(*lock){
        return;
    }
    for event in asset_events.read(){
        match event {
            AssetEvent::LoadedWithDependencies { id } => {

                if (gltf_handle_reacord.hash_set.contains(id)){
                    *read_gltf_count += 1;
                    if(*read_gltf_count == gltf_handle_reacord.hash_set.len()){
                        state.set(AppState::ResourceProcessingOver);
                    }
                }

                if (read_animation_tabel.hash_set.contains(id)){
                    *read_animation_count += 1;
                }


                if(scenes_handle_reacord.hash_set.contains(id)){
                    *read_scenes += 1;
                }

                // if(
                // *read_animation_count != 0 &&    
                // *read_gltf_count == gltf_handle_reacord.hash_set.len() 
                // && *read_scenes == scenes_handle_reacord.hash_set.len()
                // && *read_animation_count == read_animation_tabel.hash_set.len()
                // ){
                //     println!("all resouce read ok!!!");
                //     state.set(AppState::GameStart);
                //     *lock = true;
                // }
            },
            __=>{
            }
        }
    }
}