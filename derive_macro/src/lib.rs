extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FromSection)]
#[doc = r#"
    Derives the `Section` trait for a struct.

    This macro generates an implementation of the `Section` trait for a struct, 
    enabling automatic parsing of its fields from a `BTreeMap<String, String>`, 
    which represents a section from a configuration file.

    Each field in the struct must implement `FromStr`, as the macro will attempt 
    to parse the corresponding string value for each field in the section map.
    There is no need to import the `Section` trait, as the macro will do so
    automatically.

    # Example

    ```rust
    # use config_tools::Section;
    #[derive(FromSection)]
    struct ServerConfig {
        host: String,
        port: u16,
    }

    let config = Config::load("config.ini")?;
    let server_section = config.section("Server").unwrap();
    let server_config = ServerConfig::from_section(server_section)?;
    println!("{:?}", server_config);
    ```

    In this example, the `ServerConfig` struct will automatically be populated
    from the `[Server]` section of the `config.ini` file, with values for 
    `host` and `port`.
"#]
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