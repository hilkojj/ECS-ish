use crate::ecs::*;

use std::{
    any::TypeId,
    collections::HashMap,
    ops::DerefMut,
    sync::{Arc, Mutex},
};

pub struct World {
    component_type_to_i: HashMap<TypeId, usize>,
    component_type_i_counter: usize,

    entities: HashMap<EntityId, AtomicEntity>,
    entity_id_counter: EntityId,
    dirty_entities: Vec<EntityId>,

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
            dirty_entities: Vec::new(),

            family_metas: Vec::new(),
            system_metas: Vec::new(),
            system_id_counter: 0,
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        self.entity_id_counter += 1;

        self.entities.insert(
            self.entity_id_counter,
            Arc::new(Mutex::new(Entity::new(self.entity_id_counter))),
        );

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
        T: 'static + Send + Sync,
    {
        let type_i = self.component_type_id_i(&TypeId::of::<T>());

        if let Some(atomic_entity) = self.entities.get(&entity_id) {
            let mut entity = atomic_entity.lock().unwrap();
            entity.add(component, type_i);
            entity.component_bits.set(type_i, true);
            if !entity.dirty {
                self.dirty_entities.push(entity_id);
                entity.dirty = true;
            }
        }
    }

    pub fn remove_component<T>(&mut self, entity_id: EntityId) -> bool
    where
        T: 'static,
    {
        let type_i = self.component_type_id_i(&TypeId::of::<T>());

        if let Some(atomic_entity) = self.entities.get_mut(&entity_id) {
            let mut entity = atomic_entity.lock().unwrap();
            let removed_comp = entity.remove(type_i);
            entity.component_bits.set(type_i, false);
            if !entity.dirty && removed_comp {
                self.dirty_entities.push(entity_id);
                entity.dirty = true;
            }
            return removed_comp;
        }
        false
    }

    pub fn add_system<T: System>(&'a mut self, priority: usize) -> SystemId
    where
        T: 'static,
    {
        // let the system specify what family of entities it wants:
        let mut family = Family::new();
        let system = T::create(FamilyBuilder::new(self, &mut family));

        let family_index;

        if let Some(i) = self
            .family_metas
            .iter()
            .position(|meta| meta.family == family)
        {
            family_index = i; // family already exists
        } else {
            // family did not exist already -> save it.
            self.family_metas.push(FamilyMeta::new(family));
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
        self.clean_entities();
        self.init_new_families();

        let after_update = AfterUpdate::new();
        for sys_meta in &mut self.system_metas {
            let fam_meta = self
                .family_metas
                .get(sys_meta.family_index)
                .expect("sys_meta.family_index < self.family_metas.len()");
            sys_meta
                .system
                .deref_mut()
                .update(&fam_meta.entities, after_update.clone());
        }
        self.handle_after_update(after_update);
    }

    fn handle_after_update(&mut self, after_update: AfterUpdate) {
        let functions = after_update.functions.lock().unwrap();
        for fun in &*functions {
            fun(self);
        }
    }

    fn clean_entities(&mut self) {
        // for each dirty entity -> recheck family memberships.
        for entity_id in &self.dirty_entities {
            if let Some(atomic_entity) = self.entities.get_mut(&entity_id) {
                let mut entity = atomic_entity.lock().unwrap();
                println!("Rechecking family memberships for Entity {}", entity_id);
                for (fam_i, fam_meta) in self.family_metas.iter_mut().enumerate() {
                    fam_meta.insert_or_take_from_family(fam_i, &mut entity, &atomic_entity);
                }
                entity.dirty = false;
            }
        }
        self.dirty_entities.clear();
    }

    fn init_new_families(&mut self) {
        // for each new family -> find entities
        for (fam_i, fam_meta) in self
            .family_metas
            .iter_mut()
            .filter(|meta| !meta.initialized)
            .enumerate()
        {
            println!("Finding entities for new Family");
            for atomic_entity in &mut self.entities.values() {
                let mut entity = atomic_entity.lock().unwrap();
                fam_meta.insert_or_take_from_family(fam_i, &mut entity, &atomic_entity);
            }
            fam_meta.initialized = true;
        }
    }
}
