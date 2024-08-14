use proc_macro2::TokenStream;
use quote::quote;

use crate::input::ComponentInfo;

pub(crate) struct CapturePart {
    optional: TokenStream,
    field_capture: TokenStream,
    component: TokenStream,
}

//The max amount of elements bones_ecs query can have
const CHUNK_SIZE: usize = 26;

pub(crate) fn generate_parts(components: &[ComponentInfo]) -> Vec<CapturePart> {
    components
        .iter()
        .map(|comp| {
            let field_name = &comp.snake_case;
            let optional = quote! { &Optional(&#field_name) };
            let field_capture = quote! { #field_name: #field_name.cloned() };
            let component = quote! { #field_name };
            CapturePart {
                optional,
                field_capture,
                component,
            }
        })
        .collect()
}

pub(crate) fn generate_capture_jobs(parts: &[CapturePart]) -> TokenStream {
    let iter_with_params = parts.iter().map(|part| &part.optional);
    let entity_container_fields = parts.iter().map(|part| &part.field_capture);
    let components = parts.iter().map(|part| &part.component);

    let component_tuple = if parts.is_empty() {
        quote! { () }
    } else {
        quote! { (#(#components,)*) }
    };

    quote! {

        for (entity, #component_tuple) in entities.iter_with((
            #(#iter_with_params,)*
        )) {
            let entity_container = SerializableEntity {
                entity: entity.clone(),
                #(#entity_container_fields,)*
            };
            serializables.push(entity_container);
        }

    }
}
