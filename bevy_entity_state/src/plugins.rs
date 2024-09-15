use std::default;
use std::process::Command;
use std::task::Context;
use bevy::app::{Startup, Update};
use bevy::prelude::{Added, Changed, Entity, EventReader, IntoSystemConfigs, Or, Query, Res, StateTransitionEvent, SystemSet, With, Without};
use bevy::time::Time;
use bevy::{app::Plugin, prelude::Commands};
use bevy_base::structs::const_link_type::sub_state;
use bevy_base::structs::{self, const_base, const_bool, const_link_type, const_time};
use bevy_base::structs::comp::{*};
use bevy_mask_system::MaskSys;
use mask_system_lib::{*};
// use mask_system_lib::{*};
use bevy_base::structs::comp::{TimePass,Marker};
use crate::structs::comp::{AppointStateTransition, CheckStateTransition, DefualtStateTransition, MainState, SubState};
pub struct BevyEntityStatePlugin;




use crate::structs::const_base_state;
use crate::structs::const_creature_state;

impl Plugin for BevyEntityStatePlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Startup, test.in_set(BevyEntityStateSysSet::Init).after(BevyEntityStateSysSet::Run))
            .add_systems(Update,F::<op!(MainTimePassTick ),TimePassTick,Content::<{const_base::creature},0,0,0,{const_creature_state::run},0,0>>::sign())
            .add_systems(Update,F::<op!(MainTimePassTick ),TimePassTick,Content::<{const_base::creature},0,0,0,{const_creature_state::spawn},0,0>>::sign())
            .add_systems(Update,F::<op!(SubStateTimePassTick),TimePassTick,Content::<{const_base::creature},0,0,{const_base_state::ENTRY},0,0,0>>::sign())
            .add_systems(Update,F::<op!(SubStateTimePassTick),TimePassTick,Content::<{const_base::creature},0,0,{const_base_state::EXIT},0,0,0>>::sign())

            //单独的run状态改变
            .add_systems(Update,F::<op!(SubRun),StateChange,Content::<{const_base::creature},0,0,0,0,0,0>>::sign())

            //子状态转变使用的函数
            .add_systems(Update, F::<op!(DefualtSubStateChange),StateChange,Content::<{const_base::creature},0,0,{const_base_state::ENTRY},{const_base_state::RUN},0,0>>::sign())
            .add_systems(Update, F::<op!(DefualtSubStateChange),StateChange,Content::<{const_base::creature},0,0,{const_base_state::EXIT},{const_base_state::END},0,0>>::sign())


            //主转台转变使用的函数
            .add_systems(Update, F::<op!(DefualtMainStateChange),StateChange,Content::<{const_base::creature},0,0,{const_creature_state::run},{const_creature_state::idle},0,0>>::sign())
            .add_systems(Update, F::<op!(DefualtMainStateChange),StateChange,Content::<{const_base::creature},0,0,{const_creature_state::spawn},{const_creature_state::idle},0,0>>::sign())
        
            //指定转换函数
            .add_systems(Update, F::<op!(MainAppointStateChange),StateChange,Content::<{const_base::creature},0,0,{const_creature_state::idle},{const_creature_state::run},0,0>>::sign())

            //指定子状态受到指令转变
            .add_systems(Update, F::<op!(DefualtSubAppointStateChange),StateChange,Content::<{const_base::creature},0,0,{const_base_state::RUN},{const_base_state::EXIT},0,0>>::sign())

            //子状态转变主状态
            .add_systems(Update, F::<op!(SubChangeMainState),StateChange,Content::<{const_base::creature},0,0,{const_base_state::END},{const_creature_state::run},0,0>>::sign())

        ;
    }
}

#[derive(SystemSet,Hash,PartialEq,PartialOrd,Debug,Eq,Clone,Default)]
pub enum BevyEntityStateSysSet{
    #[default]
    Init,
    Run,
}

fn test(
    mut cmd:Commands
){
    let main_ent = 
            cmd.spawn_empty()
            .insert(MainState::<{const_creature_state::spawn}>)
            .insert(TimePass::<{ const_time::state_timer  }>{
                start_time: 0.0,
                max_time: 0.25,
                is_over: false,
                is_stop: false,
                elapsed_time: 0.0,
            })
            .id();
    let sub_ent = cmd.spawn_empty()
            .insert(SubState::<{const_base_state::ENTRY}>)
            .id();
    let m2clink = Link::<{ const_link_type::sub_state }>{
        srouce:main_ent,
        link:sub_ent
    };
    let c2mlink = Link::<{ const_link_type::state }>{
        srouce:sub_ent,
        link:main_ent
    };
    cmd.entity(main_ent)
        .insert(m2clink)
        .insert(Marker::<{const_base::creature}>);
    cmd.entity(sub_ent)
        .insert(c2mlink)
        .insert(TimePass::<{ const_time::state_timer  }>{
            start_time: 0.0,
            max_time: 0.25,
            is_over: false,
            is_stop: false,
            elapsed_time: 0.0,
        })
        .insert(Marker::<{const_base::creature}>);;
}


#[derive(MaskSys)]
struct TimePassTick; //子状态处理



type MainTimePassTick = Tag_1_2;
type SubStateTimePassTick = Tag_2_4;

impl<Content:MaskSystemContent + 'static> MaskSystem<MainTimePassTick,Content> for TimePassTick
    where 
    [(); {Content::marker}]:,
    [(); {Content::tag_2_c}]:,
{
    const _marker:usize = 2;

    type Output = 
        (
            fn(Commands,Res<Time>,Query<(Entity,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<MainState<{Content::tag_2_c}>>)>)
        )
    ;

    fn export(
    ) -> Self::Output {
        (
            |mut cmd:Commands,mut time:Res<Time>,mut query:Query<(Entity,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<MainState<{Content::tag_2_c}>>)>|{
                for (ent,mut time_pass) in &mut query{
                    if(time_pass.is_over){
                        cmd.entity(ent).insert(DefualtStateTransition);
                        continue;
                    }
                    time_pass.tick(time.delta_seconds());
                }
            }
        )
    }
}

impl<Content:MaskSystemContent + 'static> MaskSystem<SubStateTimePassTick,Content> for TimePassTick
    where 
    [(); {Content::marker}]:,
    [(); {Content::tag_1_c}]:,
{
    const _marker:usize = 4;

    type Output = 
        (
            fn(Commands,Res<Time>,Query<(Entity,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<SubState<{Content::tag_1_c}>>)>)
        )
    ;

    fn export(
    ) -> Self::Output {
        (
            |mut cmd:Commands,mut time:Res<Time>,mut query:Query<(Entity,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<SubState<{Content::tag_1_c}>>)>|{
                for (ent,mut time_pass) in &mut query{
                    if(time_pass.is_over){
                        cmd.entity(ent).insert(DefualtStateTransition);
                        continue;
                    }
                    time_pass.tick(time.delta_seconds());
                }
            }
        )
    }
}

#[derive(MaskSys)]
struct StateChange;

type DefualtSubStateChange = Tag_1_2;
type DefualtMainStateChange = Tag_2_4;
type MainAppointStateChange = Tag_3_8;
type DefualtSubAppointStateChange = Tag_4_16;
type SubChangeMainState = Tag_5_32;
type SubRun = Tag_6_64;

impl<Content:MaskSystemContent + 'static> MaskSystem<DefualtSubStateChange,Content> for StateChange
    where 
    [(); {Content::marker}]:,
    [(); {Content::tag_1_c}]:,
    [(); {Content::tag_2_c}]:
{
    const _marker:usize = 2;

    type Output = 
        (
            fn(Commands,Query<(Entity,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<SubState<{Content::tag_1_c}>>,With<DefualtStateTransition>)>),
        )
    ;

    fn export() -> Self::Output {
        (
            |mut cmd:Commands,mut query:Query<(Entity,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<SubState<{Content::tag_1_c}>>,With<DefualtStateTransition>)>|{
                for (ent,mut time_pass) in &mut query{
                    cmd.entity(ent).remove::<SubState<{Content::tag_1_c}>>();
                    cmd.entity(ent).insert(SubState::<{Content::tag_2_c}>);
                    time_pass.reset();
                }
            },
        )
    }
}

impl<Content:MaskSystemContent + 'static> MaskSystem<DefualtMainStateChange,Content> for StateChange
    where 
    [(); {Content::marker}]:,
    [(); {Content::tag_1_c}]:,
    [(); {Content::tag_2_c}]:
{
    const _marker:usize = 4;

    type Output = 
        (
            fn(Commands,Query<(Entity,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<MainState<{Content::tag_1_c}>>,With<DefualtStateTransition>)>),
        )
    ;

    fn export() -> Self::Output {
        (
            |mut cmd:Commands,mut query:Query<(Entity,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<MainState<{Content::tag_1_c}>>,With<DefualtStateTransition>)>|{
                for (ent,mut time_pass) in &mut query{
                    cmd.entity(ent).remove::<DefualtStateTransition>();
                    cmd.entity(ent).remove::<MainState<{Content::tag_1_c}>>();
                    cmd.entity(ent).insert(MainState::<{Content::tag_2_c}>);
                    time_pass.reset();
                    println!("主状态转变了");
                }
            },
        )
    }
}

impl<Content:MaskSystemContent + 'static> MaskSystem<MainAppointStateChange,Content> for StateChange
    where 
    [(); {Content::marker}]:,
    [(); {Content::tag_1_c}]:,
    [(); {Content::tag_2_c}]:
{
    const _marker:usize = 8;

    type Output = 
        (
            fn(Commands,Query<(Entity,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<MainState<{Content::tag_1_c}>>,With<AppointStateTransition<{Content::tag_2_c}>>)>),
        )
    ;

    fn export() -> Self::Output {
        (
            |mut cmd:Commands,mut query:Query<(Entity,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<MainState<{Content::tag_1_c}>>,With<AppointStateTransition<{Content::tag_2_c}>>)>|{
                for (ent,mut time_pass) in &mut query{
                    cmd.entity(ent).remove::<AppointStateTransition<{Content::tag_2_c}>>();
                    cmd.entity(ent).remove::<MainState<{Content::tag_1_c}>>();
                    cmd.entity(ent).insert(MainState::<{Content::tag_2_c}>);
                    time_pass.reset();
                    println!("main当前转换为run状态");
                }
            },
        )
    }
}


impl<Content:MaskSystemContent + 'static> MaskSystem<DefualtSubAppointStateChange,Content> for StateChange
    where 
    [(); {Content::marker}]:,
    [(); {Content::tag_1_c}]:,
    [(); {Content::tag_2_c}]:
{
    const _marker:usize = 16;

    type Output = 
        (
            fn(Commands,Query<(Entity,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<SubState<{Content::tag_1_c}>>,Added<AppointStateTransition<{Content::tag_2_c}>>)>),
        )
    ;

    fn export() -> Self::Output {
        (
            |mut cmd:Commands,mut query:Query<(Entity,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<SubState<{Content::tag_1_c}>>,Added<AppointStateTransition<{Content::tag_2_c}>>)>|{
                for (ent,mut time_pass) in &mut query{
                    cmd.entity(ent).remove::<SubState<{Content::tag_1_c}>>();
                    cmd.entity(ent).insert(SubState::<{Content::tag_2_c}>);
                    time_pass.reset();
                }
            },
        )
    }
}

impl<Content:MaskSystemContent + 'static> MaskSystem<SubChangeMainState,Content> for StateChange
    where 
    [(); {Content::marker}]:,
    [(); {Content::tag_1_c}]:,
    [(); {Content::tag_2_c}]:
{
    const _marker:usize = 32;

    type Output = 
        (
            fn(Commands,Query<(Entity,&Link<{const_link_type::state}>,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<SubState<{Content::tag_1_c}>>,With<AppointStateTransition<{Content::tag_2_c}>>)>),
        )
    ;

    fn export() -> Self::Output {
        (
            |mut cmd:Commands,mut query:Query<(Entity,&Link<{const_link_type::state}>,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<SubState<{Content::tag_1_c}>>,With<AppointStateTransition<{Content::tag_2_c}>>)>|{
                for (ent,mut link,mut time_pass) in &mut query{
                    cmd.entity(ent).remove::<AppointStateTransition::<{Content::tag_2_c}>>();
                    let main_state_ent = link.link;
                    cmd.entity(main_state_ent).insert(AppointStateTransition::<{Content::tag_2_c}>);
                    cmd.entity(ent).remove::<SubState::<{Content::tag_1_c}>>();
                    cmd.entity(ent).insert(SubState::<{const_base_state::ENTRY}>);
                    time_pass.reset();
                    println!("添加了子状态转变到主状态");
                }
            },
        )
    }
}

impl<Content:MaskSystemContent + 'static> MaskSystem<SubRun,Content> for StateChange
    where 
    [(); {Content::marker}]:,
    [(); {Content::tag_1_c}]:,
    [(); {Content::tag_2_c}]:
{
    const _marker:usize = 64;

    type Output = 
        (
            fn(Commands,Query<(Entity),(With<Marker<{Content::marker}>>,Added<SubState<{const_base_state::RUN}>>)>),
        )
    ;

    fn export() -> Self::Output {
        (
            |mut cmd:Commands,mut query:Query<(Entity),(With<Marker<{Content::marker}>>,Added<SubState<{const_base_state::RUN}>>)>|{
                for (ent) in &mut query{
                    cmd.entity(ent).remove::<DefualtStateTransition>();
                }
            },
        )
    }
}