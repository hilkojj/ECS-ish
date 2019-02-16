use crate::ecs::EntityId;
use crate::utils::Bits;

pub struct FamilyMeta {
    pub family: Family,
    pub entities: Vec<EntityId>,
}

pub struct Family {
    all_components: Bits,
    any_components: Bits,
    exclude_components: Bits,
}

impl Family {
    pub fn new() -> Self {
        Self {
            all_components: Bits::new(),
            any_components: Bits::new(),
            exclude_components: Bits::new(),
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
