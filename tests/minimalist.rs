use bones_ecs::prelude::*;
use bones_snap::bones_snap;
use serde::{Deserialize, Serialize};

//Components
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Pos;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Vel;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Vol;

#[test]
fn test_bones_snap_struct_generation() {
    bones_snap! {
        Components(Pos, Vel, Vol),
        Resources(Entities)
    }
}
