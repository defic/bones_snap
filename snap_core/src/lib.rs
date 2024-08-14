use input::WorldSnapshotInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

mod input;

pub fn bones_snap(input: TokenStream) -> TokenStream {
    let input = parse2::<WorldSnapshotInput>(input).unwrap();

    //new stuff
    let component_stores_borrow = input.components.iter().map(|comp| {
        let field_name = &comp.snake_case;
        let type_name = &comp.type_name;
        quote! {
            let #field_name = world.components.get_cell::<#type_name>();
            let #field_name = #field_name.borrow();
        }
    });

    let entity_container_component_fields = input.components.iter().map(|comp| {
        let field_name = &comp.snake_case;
        let type_name = &comp.type_name;
        quote! { pub #field_name: Option<#type_name> }
    });

    let set_entity_container_fields = input.components.iter().map(|comp| {
        let field_name = &comp.snake_case;
        quote! { #field_name: #field_name.get(entity).cloned() }
    });

    //old stuff:

    let resource_fields = input.resources.iter().map(|resource| {
        let field_name = &resource.snake_case;
        let type_name = &resource.type_name;
        quote! { pub #field_name: #type_name }
    });

    let resource_field_initialization = input.resources.iter().map(|resource| {
        let field_name = &resource.snake_case;
        let type_name = &resource.type_name;
        quote! {
            #field_name: (*world.get_resource::<#type_name>().unwrap()).clone()
        }
    });

    let resource_population = input.resources.iter().map(|resource| {
        let field_name = &resource.snake_case;
        quote! {
            world.insert_resource(self.#field_name);
        }
    });

    let component_stores_borrow_mut = input.components.iter().map(|comp| {
        let field_name = &comp.snake_case;
        let type_name = &comp.type_name;
        quote! {
            let #field_name = world.components.get_cell::<#type_name>();
            let mut #field_name = (*#field_name).borrow_mut();
        }
    });

    let populate_inserts = input.components.iter().map(|comp| {
        let field_name = &comp.snake_case;
        quote! {
            if let Some(c) = entity_data.#field_name {
                #field_name.insert(entity, c);
            }
        }
    });

    let expanded = quote! {

        const BITSET_EXP: u32 = 32;
        const BITSET_SIZE: usize = 2usize.saturating_pow(BITSET_EXP);
        const BITSET_SLICE_COUNT: usize = BITSET_SIZE / (32 * 8 / 8);
        static BITSET: once_cell::sync::Lazy<bones_ecs::bitset::BitSetVec> =
            once_cell::sync::Lazy::new(|| {
                bones_ecs::bitset::BitSetVec(vec![[u32::MAX; 8]; BITSET_SLICE_COUNT])
            });

        #[derive(Clone, Default, Serialize, Deserialize, Debug)]
        pub struct SerializableEntity {
            pub entity: bones_ecs::entities::Entity,
            #(#entity_container_component_fields,)*
        }

        impl SerializableEntity {

            pub fn run_collect(world: &World) -> Vec<Self> {

                #(#component_stores_borrow)*

                let entities = (*world.get_resource::<Entities>().unwrap()).clone();
                let mut serializables = vec![];

                //didn't find any other way to iterate over all entities, than: entities.iter_with_bitset
                for (entity) in entities.iter_with_bitset(&*BITSET) {

                    let entity_container = SerializableEntity {
                        entity: entity.clone(),
                        #(#set_entity_container_fields,)*
                    };
                    serializables.push(entity_container);
                }
                serializables
            }

            pub fn run_populate(world: &mut World, input: Vec<Self>) {

                #(#component_stores_borrow_mut)*

                for entity_data in input {
                    let entity: Entity = entity_data.entity;
                    #(#populate_inserts)*
                }
            }
        }

        #[derive(Clone, Default, HasSchema, Serialize, Deserialize)]
        pub struct WorldSnapshot {
            pub entity_containers: Vec<SerializableEntity>,
            #(#resource_fields, )*
        }

        impl WorldSnapshot {
            pub fn collect(world: &World) -> Self {
                let entity_containers = SerializableEntity::run_collect(world);
                WorldSnapshot {
                    entity_containers,
                    #(#resource_field_initialization, )*
                }
            }

            pub fn populate(self, world: &mut World) {
                #(#resource_population )*
                SerializableEntity::run_populate(world, self.entity_containers);
            }
        }
    };

    expanded
}
