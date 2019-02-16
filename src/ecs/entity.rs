
use std::collections::HashMap;
use std::any::{Any, TypeId};
use crate::utils::Bits;

pub type EntityId = u64;

pub struct Entity {

    pub components: HashMap<TypeId, Box<Any>>,
    pub component_bits: Bits,
    pub family_bits: Bits,
    pub index_in_family: Vec<Option<usize>>,
    pub dirty: bool
    
}

impl Entity {

    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            component_bits: Bits::new(),
            family_bits: Bits::new(),
            index_in_family: Vec::new(),
            dirty: false
        }
    }

}
