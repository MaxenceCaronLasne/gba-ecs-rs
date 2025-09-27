use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, ItemStruct, Field};

/// Attribute macro for automatically adding Component fields and implementing the Component trait.
///
/// This macro adds `next: Option<usize>` and `prev: Option<usize>` fields to your struct
/// and implements the Component trait with the required methods.
///
/// # Example
///
/// ```rust
/// use gba_ecs_rs::component;
///
/// #[component]
/// struct Position {
///     x: i32,
///     y: i32,
/// }
/// ```
///
/// This expands to:
///
/// ```rust
/// struct Position {
///     x: i32,
///     y: i32,
///     next: Option<usize>,
///     prev: Option<usize>,
/// }
///
/// impl Component for Position {
///     fn prev(&self) -> Option<usize> { self.prev }
///     fn next(&self) -> Option<usize> { self.next }
///     fn set_prev(&mut self, prev: Option<usize>) { self.prev = prev; }
///     fn set_next(&mut self, next: Option<usize>) { self.next = next; }
/// }
/// ```
#[proc_macro_attribute]
pub fn component(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input_struct = parse_macro_input!(input as ItemStruct);

    // Only support structs with named fields
    let fields = match &mut input_struct.fields {
        Fields::Named(fields_named) => fields_named,
        _ => {
            return syn::Error::new_spanned(
                &input_struct,
                "component attribute only supports structs with named fields"
            )
            .to_compile_error()
            .into();
        }
    };

    // Check if next and prev fields already exist
    let has_next = fields.named.iter().any(|f| {
        f.ident.as_ref().map(|ident| ident == "next").unwrap_or(false)
    });
    let has_prev = fields.named.iter().any(|f| {
        f.ident.as_ref().map(|ident| ident == "prev").unwrap_or(false)
    });

    if has_next || has_prev {
        return syn::Error::new_spanned(
            &input_struct,
            "component attribute cannot be used on structs that already have 'next' or 'prev' fields"
        )
        .to_compile_error()
        .into();
    }

    // Get the original fields for Default implementation before adding new ones
    let original_field_defaults: Vec<_> = fields.named.iter().map(|f| {
        let field_name = &f.ident;
        quote! { #field_name: Default::default() }
    }).collect();

    // Add the next field
    let next_field: Field = syn::parse_quote! {
        next: Option<usize>
    };

    // Add the prev field
    let prev_field: Field = syn::parse_quote! {
        prev: Option<usize>
    };

    fields.named.push(next_field);
    fields.named.push(prev_field);

    let name = &input_struct.ident;
    let generics = &input_struct.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        #input_struct

        // Implement Component trait
        impl #impl_generics Component for #name #ty_generics #where_clause {
            fn prev(&self) -> Option<usize> {
                self.prev
            }

            fn next(&self) -> Option<usize> {
                self.next
            }

            fn set_prev(&mut self, prev: Option<usize>) {
                self.prev = prev;
            }

            fn set_next(&mut self, next: Option<usize>) {
                self.next = next;
            }
        }

        // Implement Default trait
        impl #impl_generics Default for #name #ty_generics #where_clause {
            fn default() -> Self {
                Self {
                    #(#original_field_defaults,)*
                    next: None,
                    prev: None,
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro for implementing the Component trait.
///
/// This macro implements the Component trait for structs that have
/// `next: Option<usize>` and `prev: Option<usize>` fields.
///
/// # Example
///
/// ```rust
/// #[derive(Component)]
/// struct Position {
///     x: i32,
///     y: i32,
///     next: Option<usize>,
///     prev: Option<usize>,
/// }
/// ```
#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Only support structs with named fields
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => fields_named,
            _ => {
                return syn::Error::new_spanned(
                    &input,
                    "Component derive only supports structs with named fields"
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                &input,
                "Component derive only supports structs"
            )
            .to_compile_error()
            .into();
        }
    };

    // Check if next and prev fields exist
    let has_next = fields.named.iter().any(|f| {
        f.ident.as_ref().map(|ident| ident == "next").unwrap_or(false)
    });
    let has_prev = fields.named.iter().any(|f| {
        f.ident.as_ref().map(|ident| ident == "prev").unwrap_or(false)
    });

    if !has_next || !has_prev {
        return syn::Error::new_spanned(
            &input,
            "Component derive requires structs to have both 'next: Option<usize>' and 'prev: Option<usize>' fields"
        )
        .to_compile_error()
        .into();
    }

    let expanded = quote! {
        // Implement Component trait
        impl #impl_generics Component for #name #ty_generics #where_clause {
            fn prev(&self) -> Option<usize> {
                self.prev
            }

            fn next(&self) -> Option<usize> {
                self.next
            }

            fn set_prev(&mut self, prev: Option<usize>) {
                self.prev = prev;
            }

            fn set_next(&mut self, next: Option<usize>) {
                self.next = next;
            }
        }
    };

    TokenStream::from(expanded)
}