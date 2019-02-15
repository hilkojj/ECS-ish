use crate::ecs::{Family, World};

pub struct FamilyBuilder<'a> {
    pub world: &'a mut World,
}

impl<'a> FamilyBuilder<'a> {
    pub fn component<T>(&mut self) -> &mut Self
    where
        T: 'static,
    {
        println!("adding: {}", self.world.component_type_i::<T>());
        self
    }

    pub fn all(&mut self) {}

    pub fn build(&mut self) -> Family {
        Family::new()
    }
}
