// for proc-macro
mod enum_from;
mod enum_from_darling;
use proc_macro::TokenStream;
mod auto_debug;
mod auto_deref;


#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    enum_from::process_enum_from(input)
}

#[proc_macro_derive(EnumFromDarling)]
pub fn derive_enum_from_darling(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    enum_from_darling::process_enum_from_darling(input)
}

#[proc_macro_derive(AutoDebug, attributes(debug))]
pub fn derive_auto_debug(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    println!("{:#?}", &input);
    auto_debug::process_auto_debug(input)
}

#[proc_macro_derive(AutoDeref, attributes(deref))]
pub fn derive_auto_dref(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    println!("{:#?}", &input);

    TokenStream::from(auto_deref::process_auto_deref(input))
}