//! This crate provides the [`derive@named_array`] derive macro, which allows you to access fields of a
//! struct as if they were elements of an array.
//! This provides an impl of [`Index`], which translates from a `usize` index to the fields, in the
//! order in which they appear.
//!
//! The type of all the fields must be the same, and written identically.
//! For example, if one field is `Option<()>`, and another is `core::option::Option<()>`, the code
//! will be rejected.
//! This is because type information does not exist at the time of macro expansion, so there is no
//! way to confirm that the two refer to the same type.
//!
//! Indexing will panic if the index is out of bounds.
//!
//! # Example
//! ```rust
//! # use named_array::named_array;
//! #[derive(named_array)]
//! struct Example {
//!     a: u32,
//!     b: u32,
//!     c: u32,
//! }
//!
//! # fn main() {
//! let example = Example { a: 1, b: 2, c: 3 };
//! assert_eq!(example[0], example.a);
//! assert_eq!(example[1], example.b);
//! assert_eq!(example[2], example.c);
//! # }
//! ```
//!
//! [`Index`]: ::std::ops::Index

use quote::quote;

/// See the [crate] level documentation.
#[proc_macro_derive(named_array)]
pub fn named_array(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source: syn::ItemStruct = syn::parse(input).expect("Expected struct definition");

    let mut fields = match &source.fields {
        syn::Fields::Named(fields) => fields.named.iter(),
        _ => panic!("only structs with named fields are supported"),
    };

    let mut names = Vec::new();
    let ty = fields
        .next()
        .map(|f| {
            names.push(f.ident.as_ref().unwrap());
            &f.ty
        })
        .expect("Expected at least one field");
    for f in fields {
        if f.ty != *ty {
            panic!("All fields must have the same type");
        }
        names.push(f.ident.as_ref().unwrap());
    }

    let struct_name = source.ident;

    let match_parts = names.iter().enumerate().map(|(i, name)| {
        quote! {
            #i => &self.#name
        }
    });

    quote! {
        impl ::std::ops::Index<usize> for #struct_name {
            type Output = #ty;
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    #(
                        #match_parts,
                    )*
                    i => panic!("Index out of bounds: {}", i),
                }
            }
        }
    }
    .into()
}
