use heck::ToSnakeCase;
use quote::format_ident;
use syn::Ident;

pub(crate) struct ComponentInfo {
    pub type_name: Ident,
    pub snake_case: Ident,
}

pub(crate) struct WorldSnapshotInput {
    pub resources: Vec<ComponentInfo>,
    pub components: Vec<ComponentInfo>,
}

#[allow(unused)]
impl WorldSnapshotInput {
    fn resource_names(&self) -> Vec<Ident> {
        self.resources.iter().map(|x| x.type_name.clone()).collect()
    }

    fn resource_names_snaked(&self) -> Vec<Ident> {
        self.resources
            .iter()
            .map(|x| x.snake_case.clone())
            .collect()
    }

    fn component_names(&self) -> Vec<Ident> {
        self.components
            .iter()
            .map(|x| x.type_name.clone())
            .collect()
    }

    fn component_names_snaked(&self) -> Vec<Ident> {
        self.components
            .iter()
            .map(|x| x.snake_case.clone())
            .collect()
    }
}

impl syn::parse::Parse for WorldSnapshotInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut resources = Vec::new();
        let mut components = Vec::new();

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(Ident) {
                let ident: Ident = input.parse()?;
                let content;
                syn::parenthesized!(content in input);

                match ident.to_string().as_str() {
                    "Resources" => {
                        let res: Vec<ComponentInfo> = content
                            .parse_terminated(Ident::parse, syn::Token![,])?
                            .into_iter()
                            .map(|type_name| {
                                let snake_case =
                                    format_ident!("{}", type_name.to_string().to_snake_case());
                                ComponentInfo {
                                    type_name,
                                    snake_case,
                                }
                            })
                            .collect();
                        resources.extend(res);
                    }
                    "Components" => {
                        let comps: Vec<ComponentInfo> = content
                            .parse_terminated(Ident::parse, syn::Token![,])?
                            .into_iter()
                            .map(|type_name| {
                                let snake_case =
                                    format_ident!("{}", type_name.to_string().to_snake_case());
                                ComponentInfo {
                                    type_name,
                                    snake_case,
                                }
                            })
                            .collect();
                        components.extend(comps);
                    }
                    _ => {
                        return Err(syn::Error::new(
                            ident.span(),
                            "expected `Resources` or `Components`",
                        ))
                    }
                }

                if !input.is_empty() {
                    input.parse::<syn::Token![,]>()?;
                }
            } else {
                return Err(lookahead.error());
            }
        }

        if resources.is_empty() && components.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "at least one of `Resources` or `Components` must be specified",
            ));
        }

        Ok(WorldSnapshotInput {
            resources,
            components,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse2;

    #[test]
    fn test_resources_and_components() {
        let input = quote! {
            Resources(Frame, RapierContext),
            Components(Vel, Pos, PhysicsHandle)
        };
        let parsed = parse2::<WorldSnapshotInput>(input).unwrap();
        assert_eq!(parsed.resource_names(), vec!["Frame", "RapierContext"]);
        assert_eq!(
            parsed.resource_names_snaked(),
            vec!["frame", "rapier_context"]
        );
        assert_eq!(
            parsed.component_names(),
            vec!["Vel", "Pos", "PhysicsHandle"]
        );
        assert_eq!(
            parsed.component_names_snaked(),
            vec!["vel", "pos", "physics_handle"]
        );
    }

    #[test]
    fn test_resources_only() {
        let input = quote! {
            Resources(Frame, RapierContext)
        };
        let parsed = parse2::<WorldSnapshotInput>(input).unwrap();
        assert_eq!(
            parsed.resource_names_snaked(),
            vec!["frame", "rapier_context"]
        );
        assert!(parsed.components.is_empty());
    }

    #[test]
    fn test_components_only() {
        let input = quote! {
            Components(Vel, Pos, PhysicsHandle)
        };
        let parsed = parse2::<WorldSnapshotInput>(input).unwrap();
        assert!(parsed.resources.is_empty());
        assert_eq!(
            parsed.component_names(),
            vec!["Vel", "Pos", "PhysicsHandle"]
        );
    }

    #[test]
    #[should_panic(expected = "at least one of `Resources` or `Components` must be specified")]
    fn test_empty_input() {
        let input = quote! {};
        parse2::<WorldSnapshotInput>(input).unwrap();
    }

    #[test]
    fn test_reversed_order() {
        let input = quote! {
            Components(Vel, Pos),
            Resources(Frame)
        };
        let parsed = parse2::<WorldSnapshotInput>(input).unwrap();
        assert_eq!(parsed.resource_names(), vec!["Frame"]);
        assert_eq!(parsed.component_names(), vec!["Vel", "Pos"]);
    }

    #[test]
    fn test_multiple_resource_component_groups() {
        let input = quote! {
            Resources(Frame),
            Components(Vel),
            Resources(RapierContext),
            Components(Pos, PhysicsHandle)
        };
        let parsed = parse2::<WorldSnapshotInput>(input).unwrap();
        assert_eq!(parsed.resource_names(), vec!["Frame", "RapierContext"]);
        assert_eq!(
            parsed.component_names(),
            vec!["Vel", "Pos", "PhysicsHandle"]
        );
    }

    #[test]
    #[should_panic(expected = "expected `,`")]
    fn test_missing_comma() {
        let input = quote! {
            Resources(Frame, RapierContext)
            Components(Vel, Pos, PhysicsHandle)
        };
        parse2::<WorldSnapshotInput>(input).unwrap();
    }

    #[test]
    #[should_panic(expected = "expected `Resources` or `Components`")]
    fn test_invalid_group_name() {
        let input = quote! {
            InvalidGroup(Something)
        };
        parse2::<WorldSnapshotInput>(input).unwrap();
    }

    #[test]
    #[should_panic(expected = "expected parentheses")]
    fn test_missing_parentheses() {
        let input = quote! {
            Resources Frame, RapierContext
        };
        parse2::<WorldSnapshotInput>(input).unwrap();
    }
}
