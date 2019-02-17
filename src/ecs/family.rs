use crate::ecs::{Entity, EntityId};
use crate::utils::Bits;

use std::collections::HashMap;

pub struct FamilyMeta {
    pub family: Family,
    pub initialized: bool,
    pub entities: Vec<EntityId>,
}

impl FamilyMeta {
    pub fn new(family: Family) -> Self {
        Self {
            family,
            entities: Vec::new(),
            initialized: false,
        }
    }

    pub fn insert_or_take_from_family(
        &mut self,
        family_i: usize,
        entity: &mut Entity,
        entity_id: EntityId,
    ) {
        let already_in_family = entity.family_bits.get(family_i);
        let should_have = self.family.should_have(entity);

        println!(
            "Does entity already belong to this family? {}\nShould it be in family? {}",
            already_in_family, should_have
        );

        if should_have && !already_in_family {
            println!("Adding entity to family");
            self.entities.push(entity_id);
            entity.family_bits.set(family_i, true);
        } else if !should_have && already_in_family {
            println!("Removing entity from family");

            let entity_index_in_family = self
                .entities
                .iter()
                .position(|id| id == &entity_id)
                .expect("entity_id to have a position in self.entities");
            self.entities.swap_remove(entity_index_in_family);
            entity.family_bits.set(family_i, false);
        }
    }

}

pub struct Family {
    pub all_components: Bits,
    pub any_components: Bits,
    pub exclude_components: Bits,
}

impl Family {
    pub fn new() -> Self {
        Self {
            all_components: Bits::new(),
            any_components: Bits::new(),
            exclude_components: Bits::new(),
        }
    }

    fn should_have(&self, entity: &Entity) -> bool {
        self.all_components.all(&entity.component_bits)
            && (self.any_components.is_zero() || self.any_components.any(&entity.component_bits))
            && self.exclude_components.none(&entity.component_bits)
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
