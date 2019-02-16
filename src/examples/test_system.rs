use crate::ecs::{Family, FamilyBuilder, System, EntityId};

pub struct TestSystem {}

impl System for TestSystem {
    fn specify_family(&mut self, mut family_builder: FamilyBuilder) -> Family {

        println!("Hi im going to specify the family");

        family_builder
            .component::<usize>()
            .component::<u32>()
            .component::<u8>()
            .all();

        family_builder.build()
    }

    fn update(&mut self, entity_ids: &Vec<EntityId>) {
        println!("hi im gonna update: {:?}", entity_ids);
    }

}
