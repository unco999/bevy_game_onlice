use std::default;
use std::process::Command;
use std::task::Context;
use bevy::app::{Startup, Update};
use bevy::prelude::{Added, Changed, Entity, EventReader, IntoSystemConfigs, Or, Query, Res, StateTransitionEvent, SystemSet, With, Without};
use bevy::reflect::{Reflect, TypePath};
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
            // .add_systems(Update,F::<op!(MainTimePassTick ),TimePassTick,Content::<{const_base::creature},0,0,0,{const_creature_state::run},0,0>>::sign())
            .add_systems(Update,F::<op!(MainTimePassTick ),TimePassTick,Content::<{const_base::creature},0,0,0,{const_creature_state::spawn},0,0>>::sign())
            .add_systems(Update,F::<op!(DefualtTimePassTick ),TimePassTick,Content::<{const_base::creature},0,0,0,{const_creature_state::walk},0,0>>::sign())
            .add_systems(Update,F::<op!(DefualtTimePassTick ),TimePassTick,Content::<{const_base::creature},0,0,0,{const_creature_state::run},0,0>>::sign())
            .add_systems(Update,F::<op!(SubStateTimePassTick),TimePassTick,Content::<{const_base::creature},0,0,{const_base_state::ENTRY},0,0,0>>::sign())
            .add_systems(Update,F::<op!(SubStateTimePassTick),TimePassTick,Content::<{const_base::creature},0,0,{const_base_state::EXIT},0,0,0>>::sign())

            //单独的run状态改变
            .add_systems(Update,F::<op!(SubRun),StateChange,Content::<{const_base::creature},0,0,0,0,0,0>>::sign())
            //进入entry 重置tick计时器
            .add_systems(Update,F::<op!(EntryReloadtick),TimePassTick,Content::<{const_base::creature},0,0,0,0,0,0>>::sign())
            //tick时间到了转变状态的类型  tag1 是属于哪个状态  tag2是转变的状态
            .add_systems(Update,F::<op!(TickTransform),TimePassTick,Content::<{const_base::creature},0,0,{const_creature_state::walk},{const_creature_state::run},0,0>>::sign())

            //子状态转变使用的函数
            .add_systems(Update, F::<op!(DefualtSubStateChange),StateChange,Content::<{const_base::creature},0,0,{const_base_state::ENTRY},{const_base_state::RUN},0,0>>::sign())
            .add_systems(Update, F::<op!(DefualtSubStateChange),StateChange,Content::<{const_base::creature},0,0,{const_base_state::EXIT},{const_base_state::END},0,0>>::sign())


            //主转台转变使用的函数
            .add_systems(Update, F::<op!(DefualtMainStateChange),StateChange,Content::<{const_base::creature},0,0,{const_creature_state::run},{const_creature_state::idle},0,0>>::sign())
            .add_systems(Update, F::<op!(DefualtMainStateChange),StateChange,Content::<{const_base::creature},0,0,{const_creature_state::spawn},{const_creature_state::idle},0,0>>::sign())
            .add_systems(Update, F::<op!(DefualtMainStateChange),StateChange,Content::<{const_base::creature},0,0,{const_creature_state::run},{const_creature_state::attack1},0,0>>::sign())
            .add_systems(Update, F::<op!(DefualtMainStateChange),StateChange,Content::<{const_base::creature},0,0,{const_creature_state::walk},{const_creature_state::idle},0,0>>::sign())
            .add_systems(Update, F::<op!(DefualtMainStateChange),StateChange,Content::<{const_base::creature},0,0,{const_creature_state::walk},{const_creature_state::run},0,0>>::sign())
        
            //指定转换函数
            .add_systems(Update, F::<op!(MainAppointStateChange),StateChange,Content::<{const_base::creature},0,0,{const_creature_state::idle},{const_creature_state::walk},0,0>>::sign())
            .add_systems(Update, F::<op!(MainAppointStateChange),StateChange,Content::<{const_base::creature},0,0,{const_creature_state::run},{const_creature_state::idle},0,0>>::sign())
            .add_systems(Update, F::<op!(MainAppointStateChange),StateChange,Content::<{const_base::creature},0,0,{const_creature_state::run},{const_creature_state::walk},0,0>>::sign())
            .add_systems(Update, F::<op!(MainAppointStateChange),StateChange,Content::<{const_base::creature},0,0,{const_creature_state::walk},{const_creature_state::idle},0,0>>::sign())
            .add_systems(Update, F::<op!(MainAppointStateChange),StateChange,Content::<{const_base::creature},0,0,{const_creature_state::walk},{const_creature_state::run},0,0>>::sign())

            //指定子状态受到指令转变
            .add_systems(Update, F::<op!(SubAppointStateChange),StateChange,Content::<{const_base::creature},0,0,{const_base_state::RUN},{const_creature_state::idle},0,0>>::sign())
            .add_systems(Update, F::<op!(SubAppointStateChange),StateChange,Content::<{const_base::creature},0,0,{const_base_state::RUN},{const_creature_state::run},0,0>>::sign())
            .add_systems(Update, F::<op!(SubAppointStateChange),StateChange,Content::<{const_base::creature},0,0,{const_base_state::RUN},{const_creature_state::attack1},0,0>>::sign())
            .add_systems(Update, F::<op!(SubAppointStateChange),StateChange,Content::<{const_base::creature},0,0,{const_base_state::RUN},{const_creature_state::walk},0,0>>::sign())

            //子状态转变主状态
            // .add_systems(Update, F::<op!(SubChangeMainState),StateChange,Content::<{const_base::creature},0,0,{const_base_state::END},{const_creature_state::run},0,0>>::sign())
            .add_systems(Update, F::<op!(SubChangeMainState),StateChange,Content::<{const_base::creature},0,0,{const_base_state::END},{const_creature_state::walk},0,0>>::sign())
            .add_systems(Update, F::<op!(SubChangeMainState),StateChange,Content::<{const_base::creature},0,0,{const_base_state::END},{const_creature_state::idle},0,0>>::sign())
            .add_systems(Update, F::<op!(SubChangeMainState),StateChange,Content::<{const_base::creature},0,0,{const_base_state::END},{const_creature_state::attack1},0,0>>::sign())
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





#[derive(MaskSys)]
struct TimePassTick; //子状态处理



type MainTimePassTick = Tag_1_2;
type SubStateTimePassTick = Tag_2_4;
type DefualtTimePassTick = Tag_3_8;
type EntryReloadtick = Tag_4_16;
type TickTransform = Tag_5_32;

impl<Content:MaskSystemContent + 'static> MaskSystem<TickTransform,Content> for TimePassTick
where 
    [(); {Content::marker}]:,
    [(); {Content::tag_1_c}]:,   
    [(); {Content::tag_2_c}]:,   
{
    const _marker:usize = 32;

    type Output = 
        (
            fn(Commands,Query<(Entity,&Link<{const_link_type::sub_state}>,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<MainState::<{Content::tag_1_c}>>)>)
        )
    ;

    fn export(
    ) -> Self::Output {
        (
            |mut cmd:Commands,mut query:Query<(Entity,&Link<{const_link_type::sub_state}>,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<MainState::<{Content::tag_1_c}>>)>|{
                for (ent,link,mut time_pass) in &mut query{
                    if(time_pass.is_over){
                        println!("某个主状态时间结束了 转换到指定转台");
                        cmd.entity(link.link).insert(AppointStateTransition::<{Content::tag_2_c}>);
                    }
                }
            }
        )
    }
}

impl<Content:MaskSystemContent + 'static> MaskSystem<EntryReloadtick,Content> for TimePassTick
    where 
    [(); {Content::marker}]:,
    [(); {Content::tag_2_c}]:,     
{
    const _marker:usize = 16;

    type Output = 
        (
            fn(Query<(Entity,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,Added<SubState<{const_base_state::ENTRY}>>)>)
        )
    ;

    fn export(
    ) -> Self::Output {
        (
            |mut query:Query<(Entity,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,Added<SubState<{const_base_state::ENTRY}>>)>|{
                for (ent,mut time_pass) in &mut query{
                    time_pass.reset();
                }
            }
        )
    }
}

impl<Content:MaskSystemContent + 'static> MaskSystem<DefualtTimePassTick,Content> for TimePassTick
    where 
    [(); {Content::marker}]:,
    [(); {Content::tag_2_c}]:,        
{
    const _marker:usize = 8;

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
                    time_pass.tick(time.delta_seconds());
                    println!("当前的记时 {}->{} -> {}",Content::tag_2_c,time_pass.elapsed_time,time_pass.is_over);
                }
            }
        )
    }
}

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
type SubAppointStateChange = Tag_4_16;
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
            fn(Commands,Query<(Entity,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,Without<MainState<{Content::tag_2_c}>>,With<MainState<{Content::tag_1_c}>>,With<AppointStateTransition<{Content::tag_2_c}>>)>),
        )
    ;

    fn export() -> Self::Output {
        (
            |mut cmd:Commands,mut query:Query<(Entity,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,Without<MainState<{Content::tag_2_c}>>,With<MainState<{Content::tag_1_c}>>,With<AppointStateTransition<{Content::tag_2_c}>>)>|{
                for (ent,mut time_pass) in &mut query{
                    cmd.entity(ent).remove::<AppointStateTransition<{Content::tag_2_c}>>();
                    cmd.entity(ent).remove::<MainState<{Content::tag_1_c}>>();
                    cmd.entity(ent).insert(MainState::<{Content::tag_2_c}>);
                    time_pass.reset();
                    println!("main当前转换为{}状态",Content::tag_2_c);
                }
            },
        )
    }
}


impl<Content:MaskSystemContent + 'static> MaskSystem<SubAppointStateChange,Content> for StateChange
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
                    cmd.entity(ent).insert(SubState::<{const_base_state::EXIT}>);
                    println!("子状态进入了exit");
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
            fn(Commands,Query<(Entity,&Link<{const_link_type::state}>,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<SubState<{const_base_state::END}>>,With<AppointStateTransition<{Content::tag_2_c}>>)>),
        )
    ;

    fn export() -> Self::Output {
        (
            |mut cmd:Commands,mut query:Query<(Entity,&Link<{const_link_type::state}>,&mut TimePass<{const_time::state_timer}>),(With<Marker<{Content::marker}>>,With<SubState<{const_base_state::END}>>,With<AppointStateTransition<{Content::tag_2_c}>>)>|{
                for (ent,mut link,mut time_pass) in &mut query{
                    cmd.entity(ent).remove::<AppointStateTransition::<{Content::tag_2_c}>>();
                    let main_state_ent = link.link;
                    cmd.entity(main_state_ent).insert(AppointStateTransition::<{Content::tag_2_c}>);
                    cmd.entity(ent).remove::<SubState::<{const_base_state::END}>>();
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

#[derive(MaskSys)]
struct BeforStateChange;

type BeforRunState = Tag_1_2;

impl<Content:MaskSystemContent + 'static> MaskSystem<BeforRunState,Content> for BeforStateChange
    where 
    [(); {Content::tag_1_c}]:,
    [(); {Content::tag_2_c}]:,
    [(); {Content::tag_3_c}]:,
{
    const _marker:usize = 2;

    type Output = (
        fn(Commands,Query<(Entity,&Link::<{const_link_type::sub_state}>),(With<MainState<{Content::tag_1_c}>>,With<AppointStateTransition<{Content::tag_2_c}>>)>)
    );

    fn export() -> Self::Output {
        |mut cmd:Commands,mut query:Query<(Entity,&Link<{const_link_type::sub_state}>),(With<MainState<{Content::tag_1_c}>>,With<AppointStateTransition<{Content::tag_2_c}>>)>|{
            for (ent,link) in &mut query{
                cmd.entity(ent).remove::<MainState<{Content::tag_1_c}>>();
                cmd.entity(ent).insert(MainState::<{Content::tag_3_c}>);
                cmd.entity(link.link).remove::<SubState<{const_base_state::END}>>();
                cmd.entity(link.link).remove::<SubState<{const_base_state::ENTRY}>>();
                cmd.entity(link.link).remove::<SubState<{const_base_state::EXIT}>>();
                cmd.entity(link.link).remove::<SubState<{const_base_state::RUN}>>();
                cmd.entity(link.link).insert(SubState::<{const_base_state::ENTRY}>);
                println!("状态转变了 现在是 {}",Content::tag_3_c);
                
            }
        }
        //如果 tag2 是现在要转换的状态  但是我们处于tag1状态  那么我们先到指定tag3状态 也就是中间过渡状态
        
    }
}