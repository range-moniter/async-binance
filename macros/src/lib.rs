use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn generate_permission(input: TokenStream) -> TokenStream {
    let start_str = parse_macro_input!(input as LitStr).value();
    let start_num: u32 = start_str.trim_start_matches("TRD_GRP_").parse().unwrap_or(1);

    let variants: Vec<_> = (start_num..=234)
        .map(|i| {
            let variant_name = format!("TRD_GRP_{:03}", i);
            quote::format_ident!("{}", variant_name)
        })
        .collect();

    let r#gen = quote! {

        use serde::{Deserialize, Serialize};

        #[derive(Debug,Deserialize, Serialize)]
        pub enum Permission {
            SPOT,
            MARGIN,
            LEVERAGED,
            #(#variants),*
        }
    };

    r#gen.into()
}