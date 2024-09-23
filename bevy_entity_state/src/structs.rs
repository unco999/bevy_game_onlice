use std::usize;

pub mod const_base_state{
    pub const ENTRY:usize = 2;
    pub const RUN:usize = 4;
    pub const EXIT:usize = 8;
    pub const END:usize = 16;
}

pub mod const_creature_state{
    pub const all:usize = 1;
    pub const spawn:usize = 2;
    pub const idle:usize = 4;
    pub const run:usize = 8;
    pub const walk:usize = 16;
    pub const fly:usize = 32;
    pub const gahter:usize = 64;
    pub const attack1:usize = 128;
    pub const attack2:usize = 256;
    pub const spell_1:usize = 512;
    pub const spell_2:usize = 1024;
    pub const climb_up:usize = 2048;

    pub const MASK: usize = 0xFFFF;

    pub const run_repel:usize = !run & MASK;
}

pub mod special_aniamtion{
    use bevy::prelude::{Component, Entity};

    #[derive(Component)]
    pub struct ClimbUpWapll{
        pub target_wall:Entity
    }
}


pub mod comp{
    use bevy::{ecs::component::StorageType, prelude::{Component, Entity, Event}};

    #[derive(Component)]
    pub struct DefualtStateTransition;

    #[derive(Clone)]
    pub struct AppointStateTransition<const const_creature_state:usize>;

    impl<const const_creature_state:usize> Component for  AppointStateTransition<const_creature_state>{
        const STORAGE_TYPE: bevy::ecs::component::StorageType = StorageType::Table;

        fn register_component_hooks(_hooks: &mut bevy::ecs::component::ComponentHooks) {
            _hooks.on_insert(|mut world,ent,compoment_id|{
                if const_creature_state != 2 {world.commands().entity(ent).remove::<AppointStateTransition<2>>();};
                if const_creature_state != 4 {world.commands().entity(ent).remove::<AppointStateTransition<4>>();}
                if const_creature_state != 8 {world.commands().entity(ent).remove::<AppointStateTransition<8>>();}
                if const_creature_state != 16 {world.commands().entity(ent).remove::<AppointStateTransition<16>>();}
                if const_creature_state != 32 {world.commands().entity(ent).remove::<AppointStateTransition<32>>();}
                if const_creature_state != 64 {world.commands().entity(ent).remove::<AppointStateTransition<64>>();}
                if const_creature_state != 128 {world.commands().entity(ent).remove::<AppointStateTransition<128>>();}
                if const_creature_state != 256 {world.commands().entity(ent).remove::<AppointStateTransition<256>>();}
                if const_creature_state != 512 {world.commands().entity(ent).remove::<AppointStateTransition<512>>();}
                if const_creature_state != 1024 {world.commands().entity(ent).remove::<AppointStateTransition<1024>>();}
            });
        }
    }

    #[derive(Component)]
    pub struct CheckStateTransition<const const_bool:usize>;

    #[derive(Component)]
    pub struct MainState<const const_creature_state:usize>;

    #[derive(Component)]
    pub struct SubState<const const_base_state:usize>;
}
