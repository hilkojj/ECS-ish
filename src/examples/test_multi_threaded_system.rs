use crate::ecs::{
    AfterUpdate, AtomicEntity, ComponentType, Entity, FamilyBuilder, MultiThreadedSystem, System,
};
use crate::utils::ThreadPool;

#[derive(Clone)]
pub struct TestMultiThreadedSystem {
    usize_comp_type: ComponentType<usize>,
}

impl System for TestMultiThreadedSystem {
    fn create(mut builder: FamilyBuilder) -> Self {
        println!("Hi im going to specify the family");

        let s = Self {
            usize_comp_type: builder.component::<usize>(),
        };
        builder.all();
        s
    }

    fn threaded_update(&mut self, entities: &[AtomicEntity], after_update: AfterUpdate, pool: &ThreadPool, delta_time: f32) {
        self.process_all(entities, after_update, pool, delta_time);
    }
}

impl MultiThreadedSystem for TestMultiThreadedSystem {
    fn process(&self, entity: &mut Entity, after_update: AfterUpdate, delta_time: f32) {
        let component = entity.comp(&self.usize_comp_type);
        println!("before {}", component);
        *component += 100;
        println!("after {}", component);
    }
}
