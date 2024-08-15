use bones_ecs::prelude::*;
use bones_snap::bones_snap;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, HasSchema, Deserialize, Serialize, PartialEq)]
struct Pos(i32, i32);
#[derive(Default, Clone, Debug, HasSchema, Deserialize, Serialize, PartialEq)]
struct Vel(i32, i32);

bones_snap! {
    Resources(Entities),
    Components(Pos, Vel),
}

fn main() {
    let mut world = World::new();
    let mut stages = SystemStages::with_core_stages();

    stages.add_startup_system(setup);
    stages.add_system_to_stage(CoreStage::Update, move_system);

    //tick 10 times
    for _ in 0..10 {
        stages.run(&mut world);
    }

    //serialize & deserialize world
    //BonesSnap struct is generated by bones_snap! macro (above)
    let snapshot = BonesSnap::collect(&world);
    let bin = bincode::serialize(&snapshot).unwrap();
    let snapshot: BonesSnap = bincode::deserialize(&bin).unwrap();

    let mut world = World::new();
    snapshot.populate(&mut world);

    //tick again
    for _ in 0..10 {
        stages.run(&mut world);
    }

    //see if results are what are expected
    let positions = world.components.get::<Pos>();
    let positions = positions.borrow();
    let pos1 = positions.get(Entity::new(0, 0)).unwrap();
    let pos2 = positions.get(Entity::new(1, 0)).unwrap();
    assert_eq!(pos1, &Pos(20, 20));
    assert_eq!(pos2, &Pos(-20, -20));

    let vels = world.components.get::<Vel>();
    let vels = vels.borrow();
    let vel1 = vels.get(Entity::new(0, 0)).unwrap();
    let vel2 = vels.get(Entity::new(1, 0)).unwrap();
    assert_eq!(vel1, &Vel(1, 1));
    assert_eq!(vel2, &Vel(-1, -1));
}

fn setup(mut entities: ResMut<Entities>, mut pos: CompMut<Pos>, mut vel: CompMut<Vel>) {
    for i in 0..2 {
        let e = entities.create();
        pos.insert(e, Pos::default());
        let value = 1 - 2 * (i & 1); //even index -> 1, odd index -> -1
        vel.insert(e, Vel(value, value));
    }
}

fn move_system(entities: Res<Entities>, mut pos: CompMut<Pos>, vel: Comp<Vel>) {
    for (_, (pos, vel)) in entities.iter_with((&mut pos, &vel)) {
        pos.0 += vel.0;
        pos.1 += vel.1;
    }
}