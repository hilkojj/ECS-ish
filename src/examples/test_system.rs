use crate::ecs::{AtomicEntity, ComponentType, FamilyBuilder, System};

pub struct TestSystem {
    usize_comp_type: ComponentType<usize>,
}

impl System for TestSystem {
    fn create(mut builder: FamilyBuilder) -> Self {
        println!("Hi im going to specify the family");

        let s = Self {
            usize_comp_type: builder.component::<usize>(),
        };
        builder.all();
        s
    }

    fn update(&mut self, entities: &[AtomicEntity]) {
        println!("hi im gonna update {} entities", entities.len());

        for atomic_entity in entities {
            let mut entity = atomic_entity.lock().unwrap();

            let component = entity.comp(&self.usize_comp_type);
            *component += 1;
        }
    }
}
