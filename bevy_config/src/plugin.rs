use bevy::{app::{Plugin, PostStartup, Startup, Update}, asset::{AssetServer, Assets, Handle}, prelude::{in_state, Commands, IntoSystemConfigs, Local, NextState, Res, ResMut, SystemSet}};
use bevy_base::structs::AppState;
use bevy_common_assets::{ron::RonAssetPlugin};

use crate::structs::{AnimationTableCache, AnimationTableRecord, ConfigCache};

pub struct Config;

impl Plugin for  Config{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // .init_resource::<BootyResouce>()
            .add_plugins(RonAssetPlugin::<AnimationTableCache>::new(&["config/animation.ron"]))
            .add_systems(Update,animation_table_record.run_if(in_state(AppState::Init)))
            .add_systems(Update,derf.run_if(in_state(AppState::ConfigProcessing)))
            ;
    }
}



pub fn animation_table_record(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
) {
    let table_res = AnimationTableRecord(asset_server.load("config/animation.ron"));
    cmd.insert_resource(table_res);
}

fn derf(
    mut read_cache: ResMut<Assets<AnimationTableCache>>,
    mut res:Res<AnimationTableRecord>,
    mut cmd: Commands,
    mut state:ResMut<NextState<AppState>>
){
    
    if let Some(mut data) = read_cache.remove(res.0.id()) {
        println!("读取的数据 {:#?}",data);
        let mut main_config_cache = ConfigCache{
            animation_table_cache:data
        };
        cmd.insert_resource(main_config_cache);
        state.set(AppState::ConfigProcessingOver);
    }
}
