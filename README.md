# bones_snap

Macro for capturing & populating [bones_ecs](https://github.com/fishfolk/bones/) worlds, enabling effortless serde (de)serialization.

## Quick start:

`serde` feature flag needs to be enabled for `bones_ecs` in Cargo.toml

1. Define the resources and components for capturing
```rust
bones_snap! {
    Components(Pos, Vel), // Component types
    Resources(Entities) // Resource types
}
```

2. Serialize:
```rust
let snapshot = BonesSnap::collect(&world); //Calling generated code to take a snapshot
let bin = bincode::serialize(&snapshot).unwrap(); //any serde supported serialization
```

2. Deserialize:
```rust
let snapshot: BonesSnap = bincode::deserialize(&bin).unwrap();
let mut world = World::new();
snapshot.populate(&mut world); //populate is macro generated
```

Enjoy!

## Behind the scenes
The generated code will go through *every* entity, creates EntityContainer for each and captures defined component types by going through their stores.
The solution is simple and naive, but I am not aware of better alternatives. Please feel free to contribute to improve it ❤️

Enough talking! Show me the goods! So this macro call:
```rust
bones_snap! {
    Components(Pos, Vel),
    Resources(Entities)
}
```
Expands to:
```rust
#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SerializableEntity {
    pub entity: bones_ecs::entities::Entity,
    pub pos: Option<Pos>,
    pub vel: Option<Vel>,
}
impl SerializableEntity {
    pub fn run_collect(world: &World) -> Vec<Self> {
        let pos = world.components.get_cell::<Pos>();
        let pos = pos.borrow();
        let vel = world.components.get_cell::<Vel>();
        let vel = vel.borrow();
        let entities = world.resource::<Entities>();
        let mut serializables = Vec::default();
        for entity in entities.iter_with_bitset(entities.bitset()) {
            let entity_container = SerializableEntity {
                entity: entity.clone(),
                pos: pos.get(entity).cloned(),
                vel: vel.get(entity).cloned(),
            };
            serializables.push(entity_container);
        }
        serializables
    }
    pub fn run_populate(world: &mut World, input: Vec<Self>) {
        let pos = world.components.get_cell::<Pos>();
        let mut pos = (*pos).borrow_mut();
        let vel = world.components.get_cell::<Vel>();
        let mut vel = (*vel).borrow_mut();
        for entity_data in input {
            let entity: Entity = entity_data.entity;
            if let Some(c) = entity_data.pos {
                pos.insert(entity, c);
            }
            if let Some(c) = entity_data.vel {
                vel.insert(entity, c);
            }
        }
    }
}
#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct BonesSnap {
    pub entity_containers: Vec<SerializableEntity>,
    pub entities: Option<Entities>,
}
impl BonesSnap {
    pub fn collect(world: &World) -> Self {
        let entity_containers = SerializableEntity::run_collect(world);
        BonesSnap {
            entity_containers,
            entities: world.get_resource::<Entities>().map(|x| (*x).clone()),
        }
    }
    pub fn populate(self, world: &mut World) {
        if let Some(r) = self.entities {
            world.insert_resource(r);
        }
        SerializableEntity::run_populate(world, self.entity_containers);
    }
}
```

## License

This project is licensed under the MIT License.
