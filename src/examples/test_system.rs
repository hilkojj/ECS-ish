use crate::ecs::{AtomicEntity, ComponentType, FamilyBuilder, System};
use std::{thread, sync::Arc};

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

        let mut handles = Vec::new();

        for atomic_entity in entities {
            let atomic_entity = Arc::clone(atomic_entity);
            let bla = self.usize_comp_type.clone();
            let handle = thread::spawn(move || {
                
                let mut entity = atomic_entity.lock().unwrap();

                let component = entity.comp(&bla);
                println!("before {}", component);
                *component += 1;
                println!("after {}", component);

            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

    }
}
