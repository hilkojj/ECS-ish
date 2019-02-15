
use crate::ecs::{FamilyBuilder, Family};

pub trait System {
    
    fn specify_family(&mut self, family_builder: FamilyBuilder) -> Family;

}
