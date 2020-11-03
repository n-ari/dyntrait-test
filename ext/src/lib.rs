use base::{Effect, Object};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SpecificEffect {
    id: u32,
}
impl SpecificEffect {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
#[typetag::serde]
impl Effect for SpecificEffect {
    fn appliable(&self, obj: &Object) -> bool {
        self.id == obj.id
    }
}

pub fn void() {}
