//! This crate provides the [`derive@named_array`] derive macro, which allows you to access fields of a
//! struct as if they were elements of an array.
//! This provides an impl's of [`Index`] and [`IndexMut`], which translates from a `usize` index to the fields, in the
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
//! # Tuple structs
//!
//! This can be used with tuple structs as well.
//! However, you may be better off using `struct Foo([u32; 3])` instead of `struct Foo(u32, u32, u32)`.
//!
//! ```rust
//! # use named_array::named_array;
//! #[derive(named_array)]
//! struct Example(u32, u32, u32);
//! # fn main() {
//! let example = Example(1, 2, 3);
//! assert_eq!(example[0], example.0);
//! assert_eq!(example[1], example.1);
//! assert_eq!(example[2], example.2);
//! # }
//! ```
//!
//! [`Index`]: ::core::ops::Index
//! [`IndexMut`]: ::core::ops::IndexMut

use quote::quote;

/// See the [crate] level documentation.
#[proc_macro_derive(named_array)]
pub fn named_array(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = syn::parse_macro_input!(input as syn::DeriveInput);

    let (name, fields) = if let syn::Data::Struct(data) = source.data {
        (source.ident, data.fields)
    } else {
        panic!("Only structs are supported");
    };

    match fields {
        syn::Fields::Named(fields) => make_named(name, fields),
        syn::Fields::Unnamed(fields) => make_unnamed(name, fields),
        _ => panic!("unit structs are not supported"),
    }
}

fn make_named(name: syn::Ident, fields: syn::FieldsNamed) -> proc_macro::TokenStream {
    let mut fields = fields.named.iter();

    let mut errs = Vec::new();
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
            errs.push(syn::Error::new_spanned(
                &f.ty,
                "All fields must have the same type",
            ));
        }
        names.push(f.ident.as_ref().unwrap());
    }

    if !errs.is_empty() {
        let errs = errs.into_iter().map(|e| e.to_compile_error());

        // If there are any errors, return a dummy impl to avoid a flood of errors where indexing
        // gets used.
        return quote! {
            #(#errs)*

            impl ::core::ops::Index<usize> for #name {
                type Output = #ty;
                fn index(&self, _: usize) -> &Self::Output {
                    unimplemented!("Unable to generate code due to previous errors");
                }
            }

            impl ::core::ops::IndexMut<usize> for #name {
                fn index_mut(&mut self, _: usize) -> &mut Self::Output {
                    unimplemented!("Unable to generate code due to previous errors");
                }
            }
        }
        .into();
    }

    let len = names.len();
    let panic_msg = format!("index out of bounds: the len is {len} but the index is {{}}");
    let range1 = 0usize..;
    let range2 = 0usize..;

    quote! {
        impl ::core::ops::Index<usize> for #name {
            type Output = #ty;
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    #(
                        #range1 => &self.#names,
                    )*
                    i => panic!(#panic_msg, i),
                }
            }
        }

        impl ::core::ops::IndexMut<usize> for #name {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            match index {
                #(
                    #range2 => &mut self.#names,
                )*
                i => panic!(#panic_msg, i),
            }
            }
        }
    }
    .into()
}

fn make_unnamed(name: syn::Ident, fields: syn::FieldsUnnamed) -> proc_macro::TokenStream {
    let mut fields = fields.unnamed.iter();

    let len = fields.len();
    let mut errs = Vec::new();
    let ty = fields
        .next()
        .map(|f| &f.ty)
        .expect("Expected at least one field");
    for f in fields {
        if f.ty != *ty {
            errs.push(syn::Error::new_spanned(
                &f.ty,
                "All fields must have the same type",
            ));
        }
    }

    if !errs.is_empty() {
        let errs = errs.into_iter().map(|e| e.to_compile_error());
        // If there are any errors, return a dummy impl to avoid a flood of errors where indexing
        // gets used.
        return quote! {
            #(#errs)*
            impl ::core::ops::Index<usize> for #name {
                type Output = #ty;
                fn index(&self, _: usize) -> &Self::Output {
                    unimplemented!("Unable to generate code due to previous errors");
                }
            }
            impl ::core::ops::IndexMut<usize> for #name {
                fn index_mut(&mut self, _: usize) -> &mut Self::Output {
                    unimplemented!("Unable to generate code due to previous errors");
                }
            }
        }
        .into();
    }

    let panic_msg = format!("index out of bounds: the len is {len} but the index is {{}}");
    let range1 = 0usize..len;
    let range2 = 0usize..len;
    let index1 = (0usize..len).map(syn::Index::from);
    let index2 = (0usize..len).map(syn::Index::from);

    quote! {
        impl ::core::ops::Index<usize> for #name {
            type Output = #ty;
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    #( #range1 => &self.#index1, )*
                    i => panic!(#panic_msg, i),
                }
            }
        }
        impl ::core::ops::IndexMut<usize> for #name {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    #( #range2 => &mut self.#index2, )*
                    i => panic!(#panic_msg, i),
                }
            }
        }
    }
    .into()
}
