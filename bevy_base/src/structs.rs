use bevy::prelude::States;


pub mod const_base{
    pub const any:usize = 0;
    pub const creature:usize = 2;
}

pub mod const_bool{
    pub const correct:usize = 2;
    pub const wrong:usize = 4;

}


pub mod const_link_type{
    pub const state:usize = 2;
    pub const sub_state:usize = 4;
    pub const animation:usize = 8;
}

pub mod const_time{
    pub const timer:usize = 2;
    pub const state_timer:usize = 4;
}

pub mod comp{
    use bevy::{prelude::{Component, Entity}, reflect::TypePath};


    /**
     * marker实际上一个实体可以标记多种marker  
     * 这个marker是几个基本分类
     */
    #[derive(Component)]
    pub struct Marker<const const_base:usize>;

    #[derive(Component)]
    pub struct Name(pub String);

    #[derive(Component)]
    pub struct Link<const const_link_type:usize>{
        pub srouce:Entity,
        pub link:Entity,
    }


    #[derive(Component,TypePath)]
    pub struct TimePass<const const_time:usize>{
        pub start_time:f32,
        pub max_time:f32,
        pub is_over:bool,
        pub is_stop:bool,
        pub elapsed_time:f32
    }

    impl<const const_time:usize> TimePass<const_time> {
        pub fn tick(&mut self,elapsed_time:f32){
            if(self.is_over || self.is_stop){
                return;
            }
            if(self.elapsed_time >= self.max_time){
                self.is_over = true;
                return;
            }
            self.elapsed_time += elapsed_time;
        }

        pub fn stop(&mut self){
            if(self.is_over){
                println!("this TimePass is over can't stop");
                return;
            }
            self.is_stop = true;
        }

        pub fn reset(&mut self){
            self.elapsed_time = self.start_time;
            self.is_over = false;
            self.is_stop = false;
        }
    }
}


#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState{ 
    #[default]
    Init,
    ConfigProcessing, //配置读取阶段
    ConfigProcessingOver,
    ResourceProcessing, //资源处理阶段
    ResourceProcessingOver,
    GameInit,
    GameStart,
}
