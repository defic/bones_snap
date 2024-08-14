use capture::{generate_capture_jobs, generate_parts};
use input::WorldSnapshotInput;
use itertools::intersperse;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

mod capture;
mod input;

pub fn bones_snap(input: TokenStream) -> TokenStream {
    let input = parse2::<WorldSnapshotInput>(input).unwrap();

    let component_fields = input.components.iter().map(|comp| {
        let field_name = &comp.snake_case;
        let type_name = &comp.type_name;
        quote! { pub #field_name: Option<#type_name> }
    });

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

    let collect_params = input.components.iter().map(|comp| {
        let field_name = &comp.snake_case;
        let type_name = &comp.type_name;
        quote! { #field_name: Comp<#type_name> }
    });

    let iter_with_params = input.components.iter().map(|comp| {
        let field_name = &comp.snake_case;
        quote! { &Optional(&#field_name) }
    });

    let entity_container_fields = input.components.iter().map(|comp| {
        let field_name = &comp.snake_case;
        quote! { #field_name: #field_name.cloned() }
    });

    let component_tuple = if input.components.is_empty() {
        quote! { () }
    } else {
        let components = input.components.iter().map(|comp| &comp.snake_case);
        quote! { (#(#components,)*) }
    };

    let capture_parts = generate_parts(&input.components);
    //let capture_chunks = capture_parts.chunks(26).map(generate_capture_jobs);
    //let joined_captures = capture_chunks.collect::<Vec<_>>();

    let populate_params = input.components.iter().map(|comp| {
        let field_name = &comp.snake_case;
        let type_name = &comp.type_name;
        quote! { mut #field_name: CompMut<#type_name> }
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
        #[derive(Clone, Default, Serialize, Deserialize, Debug)]
        pub struct SerializableEntity {
            pub entity: bones_ecs::entities::Entity,
            #(#component_fields,)*
        }

        impl SerializableEntity {
            fn collect(
                entities: Res<Entities>,
                #(#collect_params,)*
            ) -> Vec<Self> {
                let mut serializables = vec![];

                //#(#joined_captures)*

                /* WORKINGGGG, but limit of 26 items
                for (entity, #component_tuple) in entities.iter_with((
                    #(#iter_with_params,)*
                )) {
                    let entity_container = SerializableEntity {
                        entity: entity.clone(),
                        #(#entity_container_fields,)*
                    };
                    serializables.push(entity_container);
                }
                */

                serializables
            }

            pub fn run_collect(world: &World) -> Vec<Self> {
                world.run_system(Self::collect, ())
            }

            fn populate(
                In(input): In<Vec<Self>>,
                mut entities: ResMut<Entities>,
                #(#populate_params,)*
            ) {
                for entity_data in input {
                    //using "unknown" entities could cause problems, since copying
                    //Entities resource is not supported (yet?)
                    let entity: Entity = entity_data.entity; //entities.create();
                    #(#populate_inserts)*
                }
            }

            pub fn run_populate(world: &mut World, content: Vec<Self>) {
                world.run_system(Self::populate, content);
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
