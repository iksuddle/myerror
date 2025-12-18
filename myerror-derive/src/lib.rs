use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(MyError)]
pub fn derive_answer_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let data = match &input.data {
        syn::Data::Enum(data_enum) => data_enum,
        _ => {
            return generate_error(
                input.ident.span(),
                "the MyError derive macro only works for enums",
            );
        }
    };

    let name = &input.ident;

    let var_impls = data.variants.iter().map(|v| match &v.fields {
        syn::Fields::Unnamed(unnamed_fields) => {
            let var_name = &v.ident;
            // assume there is exactly one field
            let field = unnamed_fields.unnamed.first().unwrap();
            quote! {
                impl From<#field> for #name {
                    fn from(value: #field) -> Self {
                        #name :: #var_name (value)
                    }
                }
            }
        }
        _ => generate_error(v.ident.span(), "all fields must be unnamed").into(),
    });

    let expanded = quote! {
        #(#var_impls)*
    };

    TokenStream::from(expanded)
}

fn generate_error(span: proc_macro2::Span, msg: &str) -> TokenStream {
    TokenStream::from(syn::Error::new(span, msg).to_compile_error())
}
