use std::{process::Command, time::Duration};

use bevy::{
    app::{App, Plugin, Update},
    asset::{AssetServer, Assets},
    gltf::GltfAssetLabel,
    prelude::{
        in_state, Added, AnimationGraph, AnimationNodeIndex, AnimationPlayer, AnimationTransitions,
        Changed, Commands, Entity, IntoSystemConfigs, Parent, Query, Res, ResMut, SystemSet, With,
    },
};
use bevy_base::structs::{comp::{Link, Name}, const_link_type};
use bevy_base::structs::{comp::Marker, const_base, AppState};
use bevy_config::structs::ConfigCache;
use bevy_entity_state::structs::const_creature_state::*;
use bevy_entity_state::structs::{comp::MainState, const_creature_state};
use bevy_mask_system::MaskSys;
use mask_system_lib::*;
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            F::<
                StateToAnimation,
                MainStateAnimationChange,
                Content<{ const_base::creature }, 0, 0, { const_creature_state::run }, 0, 0, 0>,
            >::sign()
            .in_set(AnimationSystem::Init)
            .run_if(in_state(AppState::GameStart)),
        )
        .add_systems(
            Update,
            F::<
                StateToAnimation,
                MainStateAnimationChange,
                Content<{ const_base::creature }, 0, 0, { const_creature_state::idle }, 0, 0, 0>,
            >::sign()
            .in_set(AnimationSystem::Init)
            .run_if(in_state(AppState::GameStart)),
        )
        .add_systems(
            Update,
            F::<
                StateToAnimation,
                MainStateAnimationChange,
                Content<{ const_base::creature }, 0, 0, { const_creature_state::attack1 }, 0, 0, 0>,
            >::sign()
            .in_set(AnimationSystem::Init)
            .run_if(in_state(AppState::GameStart)),
        )
        .add_systems(
            Update,
            F::<
                StateToAnimation,
                MainStateAnimationChange,
                Content<{ const_base::creature }, 0, 0, { const_creature_state::walk }, 0, 0, 0>,
            >::sign()
            .in_set(AnimationSystem::Init)
            .run_if(in_state(AppState::GameStart)),
        )
        .add_systems(
            Update,
            F::<
                MarkerAnimationInit,
                MainStateAnimationChange,
                Content<{ const_base::creature }, 0, 0, 0, 0, 0, 0>,
            >::sign()
            .in_set(AnimationSystem::Init)
            .run_if(in_state(AppState::GameStart)),
        );
        
    }
}

#[derive(SystemSet, Hash, PartialEq, PartialOrd, Debug, Eq, Clone, Default)]
pub enum AnimationSystem {
    #[default]
    Init,
    Run,
}

#[derive(MaskSys)]
pub struct MainStateAnimationChange;

type StateToAnimation = Tag_1_2;
type MarkerAnimationInit = Tag_2_4;

impl<Content: MaskSystemContent + 'static> MaskSystem<StateToAnimation, Content>
    for MainStateAnimationChange
where
    [(); { Content::marker }]:,
    [(); { Content::tag_1_c }]:,
{
    const _marker: usize = 2;

    type Output = (fn(
        Res<ConfigCache>,
        Query<
            (&mut AnimationTransitions, &mut AnimationPlayer),
            (
                With<AnimationTransitions>,
                With<Link::<{const_link_type::animation}>>
            ),
        >,
        Query<
            (&Name,&Link::<{const_link_type::animation}>),
            (
                With<Marker<{ Content::marker }>>,
                Added<MainState<{ Content::tag_1_c }>>,
            ),
        >
    ));

    fn export() -> Self::Output {
        |config: Res<ConfigCache>,
         mut all_animation:Query<
         (&mut AnimationTransitions, &mut AnimationPlayer),
         (
             With<AnimationTransitions>,
             With<Link::<{const_link_type::animation}>>
         ),
         >,
         mut trigger_query:Query<
             (&Name,&Link::<{const_link_type::animation}>),
             (
                 With<Marker<{ Content::marker }>>,
                 Added<MainState<{ Content::tag_1_c }>>,
             ),
         >| {
            for (name,mut link) in &mut trigger_query{
                let (mut transition,mut anim_player) = all_animation.get_mut(link.link).expect("not find animation data");
                let data1 = config.animation_table_cache.as_ref().expect("db_error").hash_map.get(&name.0).expect("not find animation with name");
                let to_anim_nodeindex  = data1.state_to_animation_id.get(&Content::tag_1_c).expect("not find animation with animation id with state");
                transition
                .play(&mut anim_player, (to_anim_nodeindex.0 as u32).into(), Duration::from_secs_f32(to_anim_nodeindex.1))
                .repeat();
            }
        }
    }
}

/**
 * 这个函数对某个初始化的marker标签添加动画信息
 * 区分 marker和name标签
 */
impl<Content: MaskSystemContent + 'static> MaskSystem<MarkerAnimationInit, Content>
    for MainStateAnimationChange
    where 
    [(); {Content::marker}]:,
{
    const _marker: usize = 4;

    type Output = (fn(
        Commands,
        Res<AssetServer>,
        ResMut<Assets<AnimationGraph>>,
        Query<(Entity),(With<Marker<{Content::marker}>>)>,
        Query<&Parent>,
        Query<(Entity, &Parent, &mut AnimationPlayer), Added<AnimationPlayer>>,
    ));

    fn export() -> Self::Output {
        |mut cmd: Commands,
         mut asset_server:Res<AssetServer>,
         mut graphs:ResMut<Assets<AnimationGraph>>,
         mut parent_query:Query<(Entity),(With<Marker<{Content::marker}>>)>,
         mut parent_h:Query<&Parent>,
         mut players: Query<(Entity,&Parent,&mut AnimationPlayer), Added<AnimationPlayer>>| {
            for (anim_ent,parent,mut player) in &mut players {
                let mut graph = AnimationGraph::new();
                let animations = graph
                    .add_clips(
                        [
                            GltfAssetLabel::Animation(1).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(2).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(3).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(4).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(5).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(6).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(7).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(8).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(9).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(10).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(11).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(12).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(13).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(14).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(15).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(16).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(17).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(18).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(19).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(20).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(21).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(22).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(23).from_asset("models/xiake.glb"),
                            GltfAssetLabel::Animation(24).from_asset("models/xiake.glb"),
                        ]
                        .into_iter()
                        .map(|path| asset_server.load(path)),
                        1.0,
                        graph.root,
                    )
                    .collect::<Vec<_>>();
                let graph = graphs.add(graph);

                let mut transitions = AnimationTransitions::new();
                transitions
                    .play(&mut player, 13.into(), Duration::ZERO)
                    .repeat();
                let h_1_parent = parent_h.get(anim_ent).expect("get parent error").get();
                let h_2_parent = parent_h.get(h_1_parent).expect("get parent error").get();
                let main = parent_query.get(h_2_parent).expect("h2_parent parent error");
                cmd.entity(anim_ent).insert(transitions).insert(graph.clone());
                cmd.entity(anim_ent).insert(Link::<{const_link_type::animation}>{srouce:anim_ent,link:main});
                cmd.entity(main).insert(Link::<{const_link_type::animation}>{srouce:main,link:anim_ent});
                
                println!("初始化所有的状态");
            }
        }
    }
}
