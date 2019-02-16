
use crate::ecs::{FamilyBuilder, Family};

pub type SystemId = usize;

pub struct SystemMeta {

    pub id: SystemId,
    pub priority: usize,
    pub system: Box<System>,
    pub family_index: usize

}

pub trait System {
    
    fn specify_family(&mut self, family_builder: FamilyBuilder) -> Family;

}
