use crate::{
    ecs::{AfterUpdate, AtomicEntity, Entity, System},
    utils::ThreadPool,
};
use std::sync::Arc;

pub trait MultiThreadedSystem: 'static + System + Clone + Send + Sync {
    fn process_all(
        &self,
        entities: &[AtomicEntity],
        after_update: AfterUpdate,
        pool: &ThreadPool,
        delta_time: f32,
    ) {
        for atomic_entity in entities {
            let atomic_entity = Arc::clone(atomic_entity);
            let sys = self.clone();
            let after_update = after_update.clone();
            pool.execute(move || {
                let mut entity = atomic_entity.lock().unwrap();
                sys.process(&mut entity, after_update, delta_time);
            });
        }
        while !pool.idle() {} // wait for all entities to be processed.
    }

    fn process(&self, entity: &mut Entity, after_update: AfterUpdate, delta_time: f32);
}
