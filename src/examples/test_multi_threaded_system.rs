use crate::ecs::{AtomicEntity, ComponentType, Entity, FamilyBuilder, MultiThreadedSystem, System};

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

    fn update(&mut self, entities: &[AtomicEntity]) {
        self.process_all(entities);
    }
}

impl MultiThreadedSystem for TestMultiThreadedSystem {
    fn process(&self, entity: &mut Entity) {
        let component = entity.comp(&self.usize_comp_type);
        println!("before {}", component);
        *component += 100;
        println!("after {}", component);
    }
}
