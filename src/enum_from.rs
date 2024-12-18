
use syn::{DeriveInput, Fields};
use proc_macro::TokenStream;
use quote::quote;
pub(crate) fn process_enum_from(input: DeriveInput) -> TokenStream  {
    println!("{:#?}", input);
    // get enum variants
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only works on enums"),
    };
    // get the ident
    let ident = input.ident;

    // get the generic
    let generics = input.generics;

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
                        impl #generics From<#ty> for #ident #generics {
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