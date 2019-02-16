
use std::collections::HashMap;
use std::any::{Any, TypeId};
use crate::utils::Bits;
use std::rc::Rc;
use std::cell::RefCell;

pub type EntityId = u64;

pub struct Entity {

    pub components: HashMap<TypeId, Rc<Box<RefCell<Any>>>>,
    pub component_bits: Bits,
    pub family_bits: Bits,
    pub dirty: bool
    
}

impl Entity {

    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            component_bits: Bits::new(),
            family_bits: Bits::new(),
            dirty: false
        }
    }

}
