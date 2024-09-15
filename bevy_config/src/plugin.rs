use bevy::{app::{Plugin, PostStartup, Startup, Update}, asset::{AssetServer, Assets}, prelude::{Commands, IntoSystemConfigs, Res, ResMut, SystemSet}};
use bevy_common_assets::{ron::RonAssetPlugin};

use crate::structs::{AnimationTableCache, AnimationTableRecord};

pub struct Config;

impl Plugin for  Config{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // .init_resource::<BootyResouce>()
            .add_plugins(RonAssetPlugin::<AnimationTableCache>::new(&["config/animation.ron"]))
            .add_systems(Startup,animation_table_record.in_set(ConfigReadSys::INIT).before(ConfigReadSys::LOADING))
            .add_systems(Update,derf.in_set(ConfigReadSys::LOADING).after(ConfigReadSys::INIT))
            ;
    }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub enum ConfigReadSys{
    INIT,
    LOADING
}


pub fn animation_table_record(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let table_res = AnimationTableRecord(asset_server.load("config/animation.ron"));
    commands.insert_resource(table_res);
}

fn derf(
    mut read_cache: ResMut<Assets<AnimationTableCache>>,
    mut res:Res<AnimationTableRecord>
){
    if let Some(mut data) = read_cache.remove(res.0.id()) {
        println!("读取的数据 {:#?}",data);
    }
}
