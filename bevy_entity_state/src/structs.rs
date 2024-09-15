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
}


pub mod comp{
    use bevy::prelude::{Component, Entity, Event};

    #[derive(Component)]
    pub struct DefualtStateTransition;

    #[derive(Component,Clone)]
    pub struct AppointStateTransition<const const_creature_state:usize>;

    #[derive(Component)]
    pub struct CheckStateTransition<const const_bool:usize>;

    #[derive(Component)]
    pub struct MainState<const const_creature_state:usize>;

    #[derive(Component)]
    pub struct SubState<const const_base_state:usize>;
}
