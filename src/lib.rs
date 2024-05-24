use quote::quote;

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
