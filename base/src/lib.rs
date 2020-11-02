pub struct Object {
    pub id: u32,
}
impl Object {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

#[typetag::serde(tag = "type")]
pub trait Effect {
    fn appliable(&self, obj: &Object) -> bool;
}

use serde::{Serialize, Deserialize};
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ForAllEffect;
impl ForAllEffect {
    pub fn new() -> Self {
        Self
    }
}
#[typetag::serde]
impl Effect for ForAllEffect {
    fn appliable(&self, _: &Object) -> bool {
        true
    }
}

