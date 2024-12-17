// for proc-macro
use proc_macro::TokenStream;
use quote::quote;
use syn::Fields;

#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    println!("{:#?}", input);
    // get enum variants
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only works on enums"),
    };
    // get the ident
    let ident = input.ident;

    // for each variant, get hte ident and fields
    let from_impls = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        match &variant.fields {
            Fields::Named(_) => { quote! {} },
            Fields::Unnamed(fields) => {
                // only support one unnamed field
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("should have 1 field");
                    let ty = &field.ty;
                    quote! {
                        impl From<#ty> for #ident {
                            fn from(x: #ty) -> Self {
                                #ident::#variant_ident(x)
                            }
                        }
                    }
                }
            },
            Fields::Unit => {quote! {}},
        }
    });
    // quote return proc-macro2 TokenStream so we need to convert it
    // to TokenStream
    TokenStream::from(quote! {
        #(#from_impls)*
    })
}