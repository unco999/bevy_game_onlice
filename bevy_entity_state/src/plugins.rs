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
            .add_systems(Update,
                         F::<op!(SubStateCheckTransition + TimePassOverChange),
                         StateProcessing,Content::<{const_base::creature},0,0,2>>::mask().in_set(BevyEntityStateSysSet::Run))
                         .add_systems(Update,F::<op!(SubStateCheckTransition),
                         StateProcessing,Content::<{const_base::creature},0,0,{const_creature_state::run}>>::sign().1.clone().in_set(BevyEntityStateSysSet::Run))
                         .add_systems(Update,F::<op!(SubStateCheckTransition),
                        StateProcessing,Content::<{const_base::creature},0,0,{const_creature_state::run}>>::sign().2.clone().in_set(BevyEntityStateSysSet::Run))
                        .add_systems(Update,F::<op!(MainStateCheckTransition),
                        StateProcessing,Content::<{const_base::creature},0,0,{const_creature_state::run}>>::sign().in_set(BevyEntityStateSysSet::Run));
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
            .insert(MainState::<{const_creature_state::idle}>)
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
        .insert(TimePass::<{ const_time::timer  }>{
            start_time: 0.0,
            max_time: 3.0,
            is_over: false,
            is_stop: false,
            elapsed_time: 0.0,
        })
        .insert(Marker::<{const_base::creature}>);;
}


#[derive(MaskSys)]
struct StateProcessing; //子状态处理



type TimePassOverChange = Tag_1_2;
type SubStateCheckTransition = Tag_2_4;
type MainStateCheckTransition = Tag_3_8;

impl<Content:MaskSystemContent + 'static> MaskSystem<MainStateCheckTransition,Content> for StateProcessing 
    where 
    [(); {Content::market}]:,
    [(); {Content::custom_val}]:
{
    const _marker:usize = 8;

    type Output = (
        fn(
            Commands,
            Query<(Entity,&Link<{const_link_type::sub_state}>),(With<MainState<{const_creature_state::run}>>,With<Marker<{Content::market}>>,Or<(With<DefualtStateTransition>,With<AppointStateTransition::<{Content::custom_val}>>)>)>
        ),
        fn(
            Commands,
            Query<(Entity,&Link::<{const_link_type::sub_state}>),(With<Marker<{Content::market}>>,With<MainState::<{const_creature_state::idle}>>,Added<AppointStateTransition::<{Content::custom_val}>>)>
        ),
    );

    fn export() -> Self::Output {
        (
            |
                mut cmd:Commands,
                mut query:Query<(Entity,&Link<{const_link_type::sub_state}>),(With<MainState<{const_creature_state::run}>>,With<Marker<{Content::market}>>,Or<(With<DefualtStateTransition>,With<AppointStateTransition::<{Content::custom_val}>>)>)>
            |{
                for (main_ent,link) in &mut query{
                    let sub_ent = link.link;
                    cmd.entity(main_ent).remove::<MainState::<{Content::custom_val}>>();
                    cmd.entity(main_ent).insert(MainState::<{const_creature_state::idle}>);
                    cmd.entity(main_ent).remove::<AppointStateTransition::<{Content::custom_val}>>();
                    cmd.entity(main_ent).insert(SubState::<{const_base_state::ENTRY}>);
                    println!("main state -> idle")
                };
            },
            |
            mut cmd:Commands,
            mut query:Query<(Entity,&Link::<{const_link_type::sub_state}>),(With<Marker<{Content::market}>>,With<MainState::<{const_creature_state::idle}>>,Added<AppointStateTransition::<{Content::custom_val}>>)>
            |{
            for (main_ent,link) in &mut query{
                cmd.entity(main_ent).remove::<MainState::<{const_creature_state::idle}>>();
                let sub_state_ent = link.link;
                cmd.entity(sub_state_ent).insert(MainState::<{Content::custom_val}>);
                cmd.entity(main_ent).insert(SubState::<{const_base_state::ENTRY}>);
                cmd.entity(main_ent).remove::<AppointStateTransition::<{Content::custom_val}>>();
                println!("main state -> run")
            };
        },
        )
    }
}

impl<Content:MaskSystemContent + 'static> MaskSystem<SubStateCheckTransition,Content> for StateProcessing 
    where 
    [(); {Content::custom_val}]:,
    [(); {Content::market}]:
{
    const _marker:usize = 4;

    type Output = (
        fn(
            Commands,
            Query<(Entity),(With<SubState<{const_base_state::ENTRY}>>,With<Marker<{Content::market}>>,With<DefualtStateTransition>)>
        ),
        fn(
            Commands,
            Query<(Entity,&AppointStateTransition::<{Content::custom_val}>,&Link<{const_link_type::state}>),(With<Link<{const_link_type::state}>>,With<SubState<{const_base_state::RUN}>>,With<Marker<{Content::market}>>,With<AppointStateTransition::<{Content::custom_val}>>)>
        ),
        fn(
            Commands,
            Query<(Entity,&AppointStateTransition::<{Content::custom_val}>,&mut TimePass<{const_time::timer}>,&Link<{const_link_type::state}>),(With<Link<{const_link_type::state}>>,With<SubState<{const_base_state::EXIT}>>,With<Marker<{Content::market}>>,With<AppointStateTransition::<{Content::custom_val}>>)>
        )
    );

    fn export() -> Self::Output {
        (
        |
            mut cmd:Commands,
            mut query:Query<(Entity),(With<SubState<{const_base_state::ENTRY}>>,With<Marker<{Content::market}>>,With<DefualtStateTransition>)>
        |{
            for (ent) in &mut query{
                cmd.entity(ent).remove::<SubState<{const_base_state::ENTRY}>>();
                cmd.entity(ent).insert(SubState::<{const_base_state::RUN}>);
                cmd.entity(ent).remove::<DefualtStateTransition>();
                println!("entry -> run");
            }
        },
        |
            mut cmd:Commands,
            mut query:Query<(Entity,&AppointStateTransition::<{Content::custom_val}>,&Link<{const_link_type::state}>),(With<Link<{const_link_type::state}>>,With<SubState<{const_base_state::RUN}>>,With<Marker<{Content::market}>>,With<AppointStateTransition::<{Content::custom_val}>>)>
        |{
            for (ent,appoint,link) in &mut query{
                cmd.entity(ent).remove::<SubState<{const_base_state::RUN}>>();
                cmd.entity(ent).insert(SubState::<{const_base_state::EXIT}>);
                println!("run -> exit");
            }
        },
        |
            mut cmd:Commands,
            mut query:Query<(Entity,&AppointStateTransition::<{Content::custom_val}>,&mut TimePass<{const_time::timer}>,&Link<{const_link_type::state}>),(With<Link<{const_link_type::state}>>,With<SubState<{const_base_state::EXIT}>>,With<Marker<{Content::market}>>,With<AppointStateTransition::<{Content::custom_val}>>)>
        |{
            for (ent,appoint,mut time_pass,link) in &mut query{
                cmd.entity(ent).remove::<SubState<{const_base_state::EXIT}>>();
                let main_state_ent = link.link;
                cmd.entity(ent).remove::<DefualtStateTransition>();
                cmd.entity(ent).remove::<AppointStateTransition::<{Content::custom_val}>>();
                cmd.entity(main_state_ent).insert(AppointStateTransition::<{Content::custom_val}>);
                time_pass.reset();
                println!("exit -> over");
            }
        }
        )
    }
}

impl<Content:MaskSystemContent + 'static> MaskSystem<TimePassOverChange,Content> for StateProcessing
    where 
    [(); {Content::custom_val}]:,
    [(); {Content::market}]:
{
    const _marker:usize = 2;

    type Output = 
        fn(
            Res<Time>,
            Commands,
            Query<(Entity,&mut TimePass<{const_time::timer}>),(Without<SubState<{const_base_state::EXIT}>>,Without<SubState<{const_base_state::RUN}>>,Without<DefualtStateTransition>,With<Marker<{Content::market}>>)>
        )
    ;

    fn export(
    ) -> Self::Output {
    
        |
            time:Res<Time>,
            mut cmd:Commands,
            mut query:Query<(Entity,&mut TimePass<{const_time::timer}>),(Without<SubState<{const_base_state::EXIT}>>,Without<SubState<{const_base_state::RUN}>>,Without<DefualtStateTransition>,With<Marker<{Content::market}>>)>
        |{
            for (ent,mut time_pass) in &mut query{
                if(time_pass.is_over){
                    cmd.entity(ent).insert(DefualtStateTransition);
                    println!("sub state over");
                    continue;
                }
                if(time_pass.is_stop){
                    continue;
                }
                time_pass.tick(time.delta_seconds());
            }
        }
    }
}