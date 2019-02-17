use crate::ecs::{AtomicEntity, FamilyBuilder, System};

pub struct TestSystem {}

impl System for TestSystem {
    fn init(&mut self, mut family_builder: FamilyBuilder) {
        println!("Hi im going to specify the family");

        family_builder
            .component::<usize>()
            // .component::<u32>()
            // .component::<u8>()
            .all();
    }

    fn update(
        &mut self,
        entities: &[AtomicEntity],
    ) {
        println!("hi im gonna update {} entities", entities.len());

        for atomic_entity in entities {
            let mut entity = atomic_entity.lock().unwrap();

            let component = entity.comp::<usize>();
            *component += 1;
        }

    }
}
