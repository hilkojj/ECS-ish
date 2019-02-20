use crate::ecs::{AtomicEntity, ComponentType, FamilyBuilder, System, AfterUpdate};

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

    fn update(&mut self, entities: &[AtomicEntity], after_update: AfterUpdate) {
        println!("hi im gonna update {} entities", entities.len());

        for atomic_entity in entities {
            let mut entity = atomic_entity.lock().unwrap();

            let component = entity.comp(&self.usize_comp_type);
            println!("before {}", component);
            *component += 1;
            println!("after {}", component);
        }

        after_update.exec(|world| {
            println!("I am executed after the update, i am going to create an entity");
            let e = world.create_entity();
            world.add_component(e, 33usize);
        });
    }
}
