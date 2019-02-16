use crate::ecs::*;

use std::any::TypeId;
use std::collections::HashMap;
use std::ops::DerefMut;

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
            entity.dirty = true;
        }
    }

    pub fn remove_component<T>(&mut self, entity_id: EntityId)
    where
        T: 'static,
    {
        let type_id = TypeId::of::<T>();
        let type_i = self.component_type_id_i(&type_id);

        if let Some(entity) = self.entities.get_mut(&entity_id) {
            entity.components.remove(&type_id);
            entity.component_bits.set(type_i, false);
            entity.dirty = true;
        }
    }

    pub fn add_system<T: System>(&'a mut self, mut system: T, priority: usize) -> SystemId
    where
        T: 'static,
    {
        // let the system specify what family of entities it wants:
        let mut family = Family::new();
        system.init(FamilyBuilder::new(self, &mut family));

        let family_index;

        if let Some(i) = self
            .family_metas
            .iter()
            .position(|meta| meta.family == family)
        {
            family_index = i; // family already exists
        } else {
            // family did not exist already -> save it.
            self.family_metas.push(FamilyMeta {
                family,
                entities: Vec::new(),
            });
            family_index = self.family_metas.len() - 1;
        }

        let system_meta = SystemMeta {
            priority,
            system: Box::new(system),
            id: self.system_id_counter,
            family_index,
        };
        self.system_metas.push(system_meta); // save system

        self.system_metas // sort systems by priority
            .sort_by(|a, b| {
                b.priority
                    .partial_cmp(&a.priority)
                    .expect("system.priority to be comparable")
            });
        self.system_id_counter += 1;
        self.system_id_counter - 1 // return system_id
    }

    pub fn remove_system(&mut self, system_id: SystemId) -> bool {
        if let Some(sys_i) = self.system_metas.iter().position(|sys| sys.id == system_id) {
            self.system_metas.remove(sys_i);
            return true;
        }
        false
    }

    pub fn update(&mut self) {

        let entity_id = 1;
        let entity = self.entities.get_mut(&entity_id).unwrap();
        let fam_meta = self.family_metas.get_mut(0).unwrap();
        fam_meta.insert_or_take_from_family(
            0,
            entity,
            entity_id
        );

        for sys_meta in &mut self.system_metas {
            sys_meta.system.deref_mut().update(
                &self
                    .family_metas
                    .get(sys_meta.family_index)
                    .expect("sys_meta.family_index < self.family_metas.len()")
                    .entities,
            );
        }
    }
}
