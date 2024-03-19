#![feature(proc_macro_quote)]
extern crate proc_macro;

use quote::quote;
use syn::{Data, DeriveInput};

#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro::TokenStream::from(input);
    let ast = syn::parse::<DeriveInput>(input).unwrap();

    let ident = &ast.ident;
    let variants = match &ast.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("only enum supported"),
    };

    let variant_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_lit = match &variant.fields {
            syn::Fields::Unit => quote!(#variant_name),
            _ => panic!("only unit supported"),
        };

        let variant_str = variant_name.to_string().trim_start_matches("Protocol").to_lowercase();
        if variant_str == "unknown" {
            quote! {}
        } else {
            quote! {
            #variant_str => Ok(#ident::#variant_name),
        }
        }
    });

    let expanded = quote! {
        impl FromStr for #ident {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #( #variant_arms )*
                    _ => Ok(ProtocolId::ProtocolUnknown),
                }
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}
