
use crate::ecs::{FamilyBuilder, Family};
use std::fmt;

pub type SystemId = usize;

pub struct SystemMeta {

    pub id: SystemId,
    pub priority: usize,
    pub system: Box<System>,
    pub family_index: usize

}

impl fmt::Debug for SystemMeta {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "System ( id: {}, priority: {}, family_index: {} )", self.id, self.priority, self.family_index)
    }

}
pub trait System {
    
    fn specify_family(&mut self, family_builder: FamilyBuilder) -> Family;

}
