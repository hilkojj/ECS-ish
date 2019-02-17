use crate::utils::Bits;
use std::any::Any;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

pub type AtomicEntity = Arc<Mutex<Entity>>;

pub type EntityId = u64;

pub struct Entity {
    pub components: Vec<Option<Box<Any>>>,
    pub component_bits: Bits,
    pub family_bits: Bits,
    pub dirty: bool,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            component_bits: Bits::new(),
            family_bits: Bits::new(),
            dirty: false,
        }
    }

    pub(crate) fn add<T>(&mut self, comp: T, comp_index: usize)
    where
        T: 'static,
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

    pub fn comp<T>(&mut self) -> &mut T
    where
        T: 'static,
    {
        let i = 0; // TODOOOOOOOOOOOO
        let comp_opt = self
            .components
            .get_mut(i)
            .expect("i < self.components.len()");
        let any_mut_ref = comp_opt.as_mut().expect("required").deref_mut();
        any_mut_ref
            .downcast_mut::<T>()
            .expect("component must be type T")
    }

    pub fn optional_comp<T>(&mut self) -> Option<&mut T>
    where
        T: 'static,
    {
        let i = 0; // TODOOOOOOOOOOO
        let comp_opt = self.components.get_mut(i)?;
        let mut any_mut_ref = comp_opt.as_mut()?.deref_mut();
        Some(any_mut_ref.downcast_mut::<T>()?)
    }
}
