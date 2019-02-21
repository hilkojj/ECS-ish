use crate::ecs::{System, AtomicEntity, Entity, AfterUpdate};
use std::{
    sync::Arc,
    thread
};

pub trait MultiThreadedSystem: 'static + System + Clone + Send + Sync {

    fn process_all(&self, entities: &[AtomicEntity], after_update: AfterUpdate) {
        println!("hi im gonna update {} entities MULTITHREADED", entities.len());

        let mut handles = Vec::new();

        for atomic_entity in entities {
            let atomic_entity = Arc::clone(atomic_entity);
            let sys = self.clone();
            let after_update = after_update.clone();
            let handle = thread::spawn(move || {
                
                let mut entity = atomic_entity.lock().unwrap();
                sys.process(&mut entity, after_update);

            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

    }

    fn process(&self, entity: &mut Entity, after_update: AfterUpdate);

}