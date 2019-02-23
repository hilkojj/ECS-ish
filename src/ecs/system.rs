use crate::ecs::{AfterUpdate, AtomicEntity, FamilyBuilder};
use crate::utils::ThreadPool;
use std::fmt;

pub type SystemId = usize;

pub struct SystemMeta {
    pub id: SystemId,
    pub priority: usize,
    pub system: Box<System>,
    pub family_index: usize,
}

impl fmt::Debug for SystemMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "System ( id: {}, priority: {}, family_index: {} )",
            self.id, self.priority, self.family_index
        )
    }
}
pub trait System {
    fn create(family_builder: FamilyBuilder) -> Self
    where
        Self: Sized;

    fn update(&mut self, entities: &[AtomicEntity], after_update: AfterUpdate, delta_time: f32) {}

    fn threaded_update(
        &mut self,
        entities: &[AtomicEntity],
        after_update: AfterUpdate,
        pool: &ThreadPool,
        delta_time: f32,
    ) {
    }
}
