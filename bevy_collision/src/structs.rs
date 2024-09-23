pub mod const_collider_type{
    pub const default:usize = 0;
    pub const wall:usize = 2;
}

pub mod comp{
    use bevy::prelude::Component;

    #[derive(Component)]
    pub struct ColliderType<const const_collider_type:usize>;
}