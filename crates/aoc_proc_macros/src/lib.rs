use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Ident, Result,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct MacroInput {
    name: Ident,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(MacroInput { name: input.parse()? })
    }
}

#[proc_macro]
pub fn scalar_or_simd(input: TokenStream) -> TokenStream {
    let MacroInput { name } = parse_macro_input!(input as MacroInput);

    let scalar_name = format_ident!("scalar_{}", name);
    let scalar_path = format!("./{}.rs", name);
    let simd_path = format!("simd/{}.rs", name);

    let output = quote! {
        #[cfg(feature = "simd")]
        #[path = #simd_path]
        pub mod #name;

        #[cfg(not(feature = "simd"))]
        pub mod #name;

        #[cfg(feature = "simd")]
        #[path = #scalar_path]
        pub mod #scalar_name;
    };

    output.into()
}

