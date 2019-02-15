use crate::ecs::{Family, FamilyBuilder, System};

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
}
