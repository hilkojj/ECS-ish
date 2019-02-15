use super::entity::Entity;
use crate::utils::Bits;

use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct Family {
    all_components: Bits,
    any_components: Bits,
    exclude_components: Bits,

    pub entities: Vec<Entity>,
}

impl Family {
    pub fn new() -> Self {
        Self {
            all_components: Bits::new(),
            any_components: Bits::new(),
            exclude_components: Bits::new(),

            entities: Vec::new(),
        }
    }
}

impl PartialEq for Family {
    fn eq(&self, other: &Family) -> bool {
        self.all_components == other.all_components
            && self.any_components == other.any_components
            && self.exclude_components == other.exclude_components
    }
}

impl Eq for Family {}
