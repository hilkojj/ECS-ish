use crate::{
    utils::Bits,
    ecs::ComponentType
};
use std::{
    any::Any,
    ops::DerefMut,
    sync::{Arc, Mutex}
};

pub type AtomicEntity = Arc<Mutex<Entity>>;

pub type EntityId = u64;

pub type Component = Any + Send + Sync;

pub struct Entity {
    pub(crate) id: EntityId,
    pub(crate) components: Vec<Option<Box<Component>>>,
    pub(crate) component_bits: Bits,
    pub(crate) family_bits: Bits,
    pub(crate) index_in_families: Vec<Option<usize>>,
    pub(crate) dirty: bool,
}

impl Entity {
    pub(crate) fn new(id: EntityId) -> Self {
        Self {
            id,
            components: Vec::new(),
            component_bits: Bits::new(),
            family_bits: Bits::new(),
            index_in_families: Vec::new(),
            dirty: false,
        }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }

    pub(crate) fn add<T>(&mut self, comp: T, comp_index: usize)
    where
        T: 'static + Send + Sync,
    {
        while comp_index >= self.components.len() {
            self.components.push(None);
        }
        let opt = self.components.get_mut(comp_index).expect("self.components.len() > comp_index");
        opt.replace(Box::new(comp));
    }

    pub(crate) fn remove(&mut self, comp_index: usize) -> bool {
        if let Some(opt) = self.components.get_mut(comp_index) {
            opt.take();
            return true
        }
        false
    }

    pub(crate) fn register_family(&mut self, fam_i: usize, index_in_fam: usize) {
        while fam_i >= self.index_in_families.len() {
            self.index_in_families.push(None);
        }
        let opt = self.index_in_families.get_mut(fam_i).expect("option for index_in_family");
        opt.replace(index_in_fam);
        self.family_bits.set(fam_i, true);
    }

    pub(crate) fn unregister_family(&mut self, fam_i: usize) {
        if let Some(opt) = self.index_in_families.get_mut(fam_i) {
            opt.take();
        }
        self.family_bits.set(fam_i, false);
    }

    pub fn comp<T>(&mut self, component_type: &ComponentType<T>) -> &mut T
    where
        T: 'static,
    {
        let comp_opt = self
            .components
            .get_mut(component_type.index)
            .expect("i < self.components.len()");
        let any_mut_ref = comp_opt.as_mut().expect("required").deref_mut();
        any_mut_ref
            .downcast_mut::<T>()
            .expect("component must be type T")
    }

    pub fn optional_comp<T>(&mut self, component_type: &ComponentType<T>) -> Option<&mut T>
    where
        T: 'static,
    {
        let comp_opt = self.components.get_mut(component_type.index)?;
        let any_mut_ref = comp_opt.as_mut()?.deref_mut();
        Some(any_mut_ref.downcast_mut::<T>()?)
    }
}
