use crate::ecs::{Family, World};

use std::any::{TypeId};

pub struct FamilyBuilder<'a> {
    pub world: &'a mut World,
    component_indexes: Vec<usize>,
    component_types: Vec<TypeId>,
    pub family: &'a mut Family,
}

impl<'a> FamilyBuilder<'a> {
    pub fn new(world: &'a mut World, family: &'a mut Family) -> Self {
        Self {
            world,
            family,
            component_indexes: Vec::new(),
            component_types: Vec::new()
        }
    }

    pub fn component<T>(&mut self) -> &mut Self
    where
        T: 'static,
    {
        let i = self.world.component_type_i::<T>();
        println!("adding: {}", i);
        self.component_indexes.push(i);
        self.component_types.push(TypeId::of::<T>());
        self
    }

    pub fn all(&mut self) {
        for i in &self.component_indexes {
            self.family.all_components.set(*i, true);
        }
        self.family.component_types.append(&mut self.component_types);
        self.component_indexes.clear();
    }

    pub fn any(&mut self) {
        for i in &self.component_indexes {
            self.family.any_components.set(*i, true);
        }
        self.family.component_types.append(&mut self.component_types);
        self.component_indexes.clear();
    }

    pub fn none(&mut self) {
        for i in &self.component_indexes {
            self.family.exclude_components.set(*i, true);
        }
        self.component_types.clear();
        self.component_indexes.clear();
    }
}
