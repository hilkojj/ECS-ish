use crate::ecs::*;

use std::any::TypeId;
use std::collections::HashMap;

pub struct World {
    component_type_to_i: HashMap<TypeId, usize>,
    component_type_i_counter: usize,

    entities: HashMap<EntityId, Entity>,
    entity_id_counter: EntityId,

    family_metas: Vec<FamilyMeta>,
    system_metas: Vec<SystemMeta>,
    system_id_counter: SystemId,
}

impl<'a> World {
    pub fn new() -> Self {
        Self {
            component_type_to_i: HashMap::new(),
            component_type_i_counter: 0,

            entities: HashMap::new(),
            entity_id_counter: 0,

            family_metas: Vec::new(),
            system_metas: Vec::new(),
            system_id_counter: 0,
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        self.entity_id_counter += 1;

        self.entities.insert(self.entity_id_counter, Entity::new());

        self.entity_id_counter
    }

    pub fn component_type_i<T>(&mut self) -> usize
    where
        T: 'static,
    {
        self.component_type_id_i(&TypeId::of::<T>())
    }

    fn component_type_id_i(&mut self, type_id: &TypeId) -> usize {
        if let Some(i) = self.component_type_to_i.get(type_id) {
            return *i;
        } else {
            self.component_type_to_i
                .insert(*type_id, self.component_type_i_counter);
            self.component_type_i_counter += 1;
            return self.component_type_i_counter - 1;
        }
    }

    pub fn add_component<T>(&mut self, entity_id: EntityId, component: T)
    where
        T: 'static,
    {
        let type_id = TypeId::of::<T>();
        let type_i = self.component_type_id_i(&type_id);

        if let Some(entity) = self.entities.get_mut(&entity_id) {
            entity.components.insert(type_id, Box::from(component));
            entity.component_bits.set(type_i, true);
            println!("{}", entity.component_bits);
        }
    }

    pub fn add_system<T: System>(&'a mut self, mut system: T, priority: usize)
    where
        T: 'static,
    {
        let mut family = system.specify_family(FamilyBuilder { world: self });
        let mut family_index = 0;

        if let Some(i) = self.family_metas.iter().position(|meta| meta.family == family) {
            println!("Family already exists");
            family_index = i;
        } else {
            println!("Family is unique and will be added to World.");
            self.family_metas.push(FamilyMeta {
                family,
                entities: Vec::new()
            });
            family_index = self.family_metas.len() - 1;
        }

        let system_meta = SystemMeta {
            priority,
            system: Box::new(system),
            id: self.system_id_counter,
            family_index
        };

        self.system_metas.push(system_meta);

        self.system_metas.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());
        println!("nr. of systems in World: {}", self.system_metas.len());
        println!("{:?}", self.system_metas);
        self.system_id_counter += 1;
    }
}
