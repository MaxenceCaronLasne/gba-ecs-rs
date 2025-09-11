use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error};

/// Derive macro for implementing the `Component` trait.
///
/// This macro can only be applied to structs and will generate an empty implementation
/// of the `Component` trait. The `Component` trait serves as a marker trait to identify
/// types that can be used as ECS components.
///
/// # Examples
///
/// ```rust,ignore
/// use gba_ecs_macros::Component;
///
/// #[derive(Component)]
/// struct Position {
///     x: f32,
///     y: f32,
/// }
///
/// #[derive(Component)]
/// struct Health {
///     current: i32,
///     max: i32,
/// }
/// ```
///
/// # Errors
///
/// This macro will produce a compile error if applied to anything other than a struct:
///
/// ```rust,ignore
/// #[derive(Component)]
/// enum InvalidComponent { } // This will cause a compile error
/// ```
pub fn derive_component_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let span = name.span();

    match input.data {
        syn::Data::Struct(_) => {
            let expanded = quote! {
                impl Component for #name {}
            };
            TokenStream::from(expanded)
        }
        syn::Data::Enum(_) => {
            let error = Error::new(
                span,
                "Component can only be derived for structs, not enums. \
                 Consider using a struct wrapper if you need enum-like behavior.",
            );
            TokenStream::from(error.to_compile_error())
        }
        syn::Data::Union(_) => {
            let error = Error::new(
                span,
                "Component can only be derived for structs, not unions. \
                 Unions are not supported for ECS components.",
            );
            TokenStream::from(error.to_compile_error())
        }
    }
}

