use bones_ecs::prelude::*;
use bones_snap::bones_snap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize, HasSchema)]
struct Pos;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema)]
struct Vel;

#[test]
fn test_bones_snap_struct_generation() {
    bones_snap! {
        Components(Pos, Vel),
        Resources(Pos, Vel)
    }
}
