extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FromSection)]
pub fn from_section_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match &input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields) => &fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let field_parsing = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_type = &f.ty;

        quote! {
            #field_name: map.get(stringify!(#field_name))
                .and_then(|value| value.parse::<#field_type>().ok())
                .ok_or_else(|| config_tools::Error::ConfigParse(format!("Failed to parse field '{}'", stringify!(#field_name))))?,
        }
    });

    let expanded = quote! {
        use config_tools::Section;
        impl config_tools::Section for #name {
            fn from_section(map: &std::collections::BTreeMap<String, String>) -> Result<Self, config_tools::Error> {
                Ok(Self {
                    #(#field_parsing)*
                })
            }
        }
    };

    TokenStream::from(expanded)
}