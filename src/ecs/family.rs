use crate::ecs::{AtomicEntity, Entity};
use crate::utils::Bits;

pub struct FamilyMeta {
    pub family: Family,
    pub initialized: bool,
    pub entities: Vec<AtomicEntity>,
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
        atomic_entity: &AtomicEntity,
    ) {
        let already_in_family = entity.family_bits.get(family_i);
        let should_have = self.family.should_have(entity);

        println!(
            "Does entity already belong to this family? {}\nShould it be in family? {}",
            already_in_family, should_have
        );

        if should_have && !already_in_family {
            println!("Adding entity to family");
            entity.register_family(family_i, self.entities.len());
            self.entities.push(atomic_entity.clone());
        } else if !should_have && already_in_family {
            println!("Removing entity from family");

            let index_in_fam = entity
                .index_in_families
                .get(family_i)
                .expect("index in family")
                .expect("index in family");
            self.entities.swap_remove(index_in_fam);
            entity.unregister_family(family_i);
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
