use crate::ecs::{Family, World, ComponentType};

pub struct FamilyBuilder<'a> {
    pub world: &'a mut World,
    component_indexes: Vec<usize>,
    pub family: &'a mut Family,
}

impl<'a> FamilyBuilder<'a> {
    pub fn new(world: &'a mut World, family: &'a mut Family) -> Self {
        Self {
            world,
            family,
            component_indexes: Vec::new(),
        }
    }

    pub fn component<T>(&mut self) -> ComponentType<T>
    where
        T: 'static,
    {
        let i = self.world.component_type_i::<T>();
        self.component_indexes.push(i);
        ComponentType::<T>::new(i)
    }

    pub fn all(&mut self) {
        for i in &self.component_indexes {
            self.family.all_components.set(*i, true);
        }
        self.component_indexes.clear();
    }

    pub fn any(&mut self) {
        for i in &self.component_indexes {
            self.family.any_components.set(*i, true);
        }
        self.component_indexes.clear();
    }

    pub fn none(&mut self) {
        for i in &self.component_indexes {
            self.family.exclude_components.set(*i, true);
        }
        self.component_indexes.clear();
    }
}
