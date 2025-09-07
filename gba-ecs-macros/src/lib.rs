use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    match input.data {
        syn::Data::Struct(_) => {
            let expanded = quote! {
                impl Component for #name {}
            };

            TokenStream::from(expanded)
        }
        _ => TokenStream::from(
            Error::new(name.span(), "Component can only be derived for structs").to_compile_error(),
        ),
    }
}

#[proc_macro]
pub fn define_world(input: TokenStream) -> TokenStream {
    use syn::{braced, parse::Parse, parse::ParseStream, punctuated::Punctuated, Ident, Token};

    struct WorldDefinition {
        world_name: Ident,
        components: Punctuated<Ident, Token![,]>,
    }

    impl Parse for WorldDefinition {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let world_name = input.parse()?;
            let content;
            braced!(content in input);
            let components = content.parse_terminated(Ident::parse, Token![,])?;

            Ok(WorldDefinition {
                world_name,
                components,
            })
        }
    }

    let world_def = parse_macro_input!(input as WorldDefinition);
    let world_name = world_def.world_name;
    let components = world_def.components;

    let storage_fields = components.iter().map(|comp| {
        let comp_lower = comp.to_string().to_lowercase();
        let field_ident = quote::format_ident!("{}_storage", comp_lower);
        quote! {
            #field_ident: gba_ecs_rs::VecStorage<#comp>
        }
    });

    let get_storage_impls = components.iter().map(|comp| {
        let comp_lower = comp.to_string().to_lowercase();
        let field_ident = quote::format_ident!("{}_storage", comp_lower);
        quote! {
            impl gba_ecs_rs::GetStorage<#comp> for #world_name {
                type Storage = gba_ecs_rs::VecStorage<#comp>;

                fn get_storage(&self) -> &Self::Storage {
                    &self.#field_ident
                }

                fn get_storage_mut(&mut self) -> &mut Self::Storage {
                    &mut self.#field_ident
                }
            }
        }
    });

    let new_fields = components.iter().map(|comp| {
        let comp_lower = comp.to_string().to_lowercase();
        let field_ident = quote::format_ident!("{}_storage", comp_lower);
        quote! {
            #field_ident: <gba_ecs_rs::VecStorage<#comp> as gba_ecs_rs::ComponentStorage<#comp>>::new()
        }
    });

    let expanded = quote! {
        struct #world_name {
            #(#storage_fields,)*
            entity_count: usize,
        }

        #(#get_storage_impls)*

        impl #world_name {
            fn new() -> Self {
                #world_name {
                    #(#new_fields,)*
                    entity_count: 0,
                }
            }

            fn spawn_entity(&mut self) -> gba_ecs_rs::Entity {
                let entity_id = self.entity_count;
                self.entity_count += 1;
                gba_ecs_rs::Entity{ index: entity_id }
            }

            fn add_component<C: gba_ecs_rs::Component>(&mut self, entity: gba_ecs_rs::Entity, component: C)
            where
                Self: gba_ecs_rs::GetStorage<C>,
            {
                <<Self as gba_ecs_rs::GetStorage<C>>::Storage as gba_ecs_rs::ComponentStorage<C>>::insert(<Self as gba_ecs_rs::GetStorage<C>>::get_storage_mut(self), entity, component);
            }

            fn remove_component<C: gba_ecs_rs::Component>(&mut self, entity: gba_ecs_rs::Entity) -> Option<C>
            where
                Self: gba_ecs_rs::GetStorage<C>,
            {
                <<Self as gba_ecs_rs::GetStorage<C>>::Storage as gba_ecs_rs::ComponentStorage<C>>::remove(<Self as gba_ecs_rs::GetStorage<C>>::get_storage_mut(self), entity)
            }

            fn get_component<C: gba_ecs_rs::Component>(&self, entity: gba_ecs_rs::Entity) -> Option<&C>
            where
                Self: gba_ecs_rs::GetStorage<C>,
            {
                <<Self as gba_ecs_rs::GetStorage<C>>::Storage as gba_ecs_rs::ComponentStorage<C>>::get(<Self as gba_ecs_rs::GetStorage<C>>::get_storage(self), entity)
            }

            fn get_component_mut<C: gba_ecs_rs::Component>(&mut self, entity: gba_ecs_rs::Entity) -> Option<&mut C>
            where
                Self: gba_ecs_rs::GetStorage<C>,
            {
                <<Self as gba_ecs_rs::GetStorage<C>>::Storage as gba_ecs_rs::ComponentStorage<C>>::get_mut(<Self as gba_ecs_rs::GetStorage<C>>::get_storage_mut(self), entity)
            }
        }
    };

    TokenStream::from(expanded)
}
