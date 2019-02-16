use crate::ecs::{EntityId, FamilyBuilder, System};
use std::any::{TypeId, Any};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

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
        entity_ids: &Vec<EntityId>,
        components: &HashMap<TypeId, Vec<Option<Rc<Box<RefCell<Any>>>>>>,
    ) {
        println!("hi im gonna update: {:?}", entity_ids);

        let usize_components = components.get(&TypeId::of::<usize>()).expect("a list");

        for (i, entity_id) in entity_ids.iter().enumerate() {

            println!("Updating nr. {} with id: {}", i, entity_id);

            let usize_comp_opt = usize_components.get(i).expect("component option");
            let usize_comp_rc = usize_comp_opt.as_ref().expect("component rc");
            let mut usize_comp_ref_cell = usize_comp_rc.borrow_mut();
            let usize_comp = usize_comp_ref_cell.downcast_mut::<usize>().expect("a fucking 'usize'");
            
            println!("comp: {}", usize_comp);
            *usize_comp += 10;
        }

    }
}
