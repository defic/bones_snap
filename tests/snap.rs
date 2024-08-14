use bones_ecs::prelude::*;
use bones_snap::bones_snap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, HasSchema, Default)]
struct Pos(f32, f32);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, HasSchema, Default)]
struct Vel(f32, f32);

#[test]
fn test_bones_snap_struct_generation() {
    bones_snap! {
        Components(Pos, Vel),
        Resources(Pos, Vel)
    }

    let entity = SerializableEntity {
        entity: Entity::new(0, 0),
        pos: Some(Pos(0.0, 0.0)),
        vel: Some(Vel(1.0, 1.0)),
    };

    let snapshot = WorldSnapshot {
        entities: vec![entity],
        pos: Default::default(),
        vel: Default::default(),
    };

    assert_eq!(snapshot.entities.len(), 1);
    assert_eq!(snapshot.entities[0].entity, Entity::new(0, 0));
    assert_eq!(snapshot.entities[0].pos, Some(Pos(0.0, 0.0)));
    assert_eq!(snapshot.entities[0].vel, Some(Vel(1.0, 1.0)));
}
