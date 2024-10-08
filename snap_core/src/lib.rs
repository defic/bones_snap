use input::BonesSnapInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

mod input;

pub fn bones_snap(input: TokenStream) -> TokenStream {
    let input = parse2::<BonesSnapInput>(input).unwrap();

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

    let resource_fields = input.resources.iter().map(|resource| {
        let field_name = &resource.snake_case;
        let type_name = &resource.type_name;
        quote! { pub #field_name: Option<#type_name> }
    });

    let resource_field_initialization = input.resources.iter().map(|resource| {
        let field_name = &resource.snake_case;
        let type_name = &resource.type_name;
        quote! {
            #field_name: world.get_resource::<#type_name>().map(|x| (*x).clone())
        }
    });

    let resource_population = input.resources.iter().map(|resource| {
        let field_name = &resource.snake_case;
        quote! {
            if let Some(r) = self.#field_name {
                world.insert_resource(r);
            }

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

        #[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
        pub struct SerializableEntity {
            pub entity: bones_ecs::entities::Entity,
            #(#entity_container_component_fields,)*
        }

        impl SerializableEntity {

            pub fn run_collect(world: &World) -> Vec<Self> {

                #(#component_stores_borrow)*

                let entities = world.resource::<Entities>();
                let mut serializables = Vec::default();

                for entity in entities.iter_with_bitset(entities.bitset()) {

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

        #[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
        pub struct BonesSnap {
            pub entity_containers: Vec<SerializableEntity>,
            #(#resource_fields, )*
        }

        impl BonesSnap {
            pub fn collect(world: &World) -> Self {
                let entity_containers = SerializableEntity::run_collect(world);
                BonesSnap {
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
