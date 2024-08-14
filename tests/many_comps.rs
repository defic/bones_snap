use bones_ecs::prelude::*;
use bones_snap::bones_snap;
use serde::{Deserialize, Serialize};

#[test]
fn test_bones_snap_struct_generation() {
    bones_snap! {
        Components(Comp1, Comp2, Comp3,Comp4, Comp5, Comp6, Comp7, Comp8),
        Components(Comp9, Comp10, Comp11, Comp12, Comp13, Comp14, Comp15, Comp16),
        Components(Comp17, Comp18, Comp19, Comp20, Comp21, Comp22, Comp23, Comp24),
        Components(Comp25, Comp26, Comp27, Comp28, Comp29),
        Resources(Comp1, Comp2)
    }
}

#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp1;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp2;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp3;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp4;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp5;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp6;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp7;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp8;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp9;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp10;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp11;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp12;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp13;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp14;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp15;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp16;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp17;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp18;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp19;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp20;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp21;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp22;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp23;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp24;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp25;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp26;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp27;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp28;
#[derive(Clone, Default, Serialize, Deserialize, HasSchema, Debug)]
struct Comp29;
