use bones_ecs::prelude::Entity;
use serde::{Deserialize, Serialize};
pub use snap_macro::bones_snap;

//OgEntity, since feature serde does not yet exist for bones?
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct OgEntity(pub u32, pub u32);

impl From<Entity> for OgEntity {
    fn from(value: Entity) -> Self {
        Self(value.index(), value.generation())
    }
}

impl From<OgEntity> for Entity {
    fn from(value: OgEntity) -> Self {
        Entity::new(value.0, value.1)
    }
}
