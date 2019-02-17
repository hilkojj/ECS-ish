use crate::ecs::{EntityId, FamilyBuilder, System};

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
        entity_ids: &[EntityId],
    ) {
        println!("hi im gonna update: {:?}", entity_ids);

    }
}
