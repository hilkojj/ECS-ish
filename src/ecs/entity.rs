
use std::collections::HashMap;
use std::any::{Any, TypeId};
use crate::utils::Bits;

pub type EntityId = u64;

pub struct Entity {

    pub components: HashMap<TypeId, Box<Any>>,
    pub component_bits: Bits
    
}

impl Entity {

    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            component_bits: Bits::new()
        }
    }

}
